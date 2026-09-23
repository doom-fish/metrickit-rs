use core::ffi::{c_char, c_void, CStr};
use std::ptr;
use std::sync::{Mutex, PoisonError};

use doom_fish_utils::callback_context::CallbackContext;
use serde::Deserialize;

use crate::diagnostic_payload::DiagnosticPayload;
use crate::error::{from_swift, MetricKitError};
use crate::ffi;
use crate::metric_payload::MetricPayload;
use crate::private::{decode_json, to_cstring};
use crate::signpost::MetricLogHandle;

/// Task identifier used with `MetricKit` extended launch APIs.
pub type LaunchTaskId = String;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct MetricManagerEvent {
    event: String,
    #[serde(default)]
    metric_payloads: Vec<MetricPayload>,
    #[serde(default)]
    diagnostic_payloads: Vec<DiagnosticPayload>,
}

/// Delegate trait mirroring `MXMetricManagerSubscriber` delivery callbacks.
pub trait MetricSubscriberDelegate: Send {
    /// Handles `MetricKit` metric payload delivery from `MXMetricManagerSubscriber`.
    fn did_receive_metric_payloads(&mut self, payloads: Vec<MetricPayload>) {
        let _ = payloads;
    }

    /// Handles `MetricKit` diagnostic payload delivery from `MXMetricManagerSubscriber`.
    fn did_receive_diagnostic_payloads(&mut self, payloads: Vec<DiagnosticPayload>) {
        let _ = payloads;
    }
}

type MetricPayloadHandler = Box<dyn FnMut(Vec<MetricPayload>) + Send + 'static>;
type DiagnosticPayloadHandler = Box<dyn FnMut(Vec<DiagnosticPayload>) + Send + 'static>;

/// Builder-style delegate adapter for `MXMetricManagerSubscriber` callbacks.
#[allow(clippy::type_complexity)]
pub struct MetricSubscriberCallbacks {
    metric_payloads: Option<MetricPayloadHandler>,
    diagnostic_payloads: Option<DiagnosticPayloadHandler>,
}

impl MetricSubscriberCallbacks {
    /// Creates an empty callback adapter for `MXMetricManagerSubscriber` events.
    #[must_use]
    pub fn new() -> Self {
        Self {
            metric_payloads: None,
            diagnostic_payloads: None,
        }
    }

    /// Registers a handler for metric payload delivery from `MXMetricManagerSubscriber`.
    #[must_use]
    pub fn on_metric_payloads(
        mut self,
        callback: impl FnMut(Vec<MetricPayload>) + Send + 'static,
    ) -> Self {
        self.metric_payloads = Some(Box::new(callback));
        self
    }

    /// Registers a handler for diagnostic payload delivery from `MXMetricManagerSubscriber`.
    #[must_use]
    pub fn on_diagnostic_payloads(
        mut self,
        callback: impl FnMut(Vec<DiagnosticPayload>) + Send + 'static,
    ) -> Self {
        self.diagnostic_payloads = Some(Box::new(callback));
        self
    }
}

impl Default for MetricSubscriberCallbacks {
    fn default() -> Self {
        Self::new()
    }
}

impl MetricSubscriberDelegate for MetricSubscriberCallbacks {
    fn did_receive_metric_payloads(&mut self, payloads: Vec<MetricPayload>) {
        if let Some(callback) = &mut self.metric_payloads {
            callback(payloads);
        }
    }

    fn did_receive_diagnostic_payloads(&mut self, payloads: Vec<DiagnosticPayload>) {
        if let Some(callback) = &mut self.diagnostic_payloads {
            callback(payloads);
        }
    }
}

type SubscriberState = Mutex<Box<dyn MetricSubscriberDelegate>>;

/// Active subscriber registration returned by `MXMetricManager.add(_:)`.
pub struct MetricSubscription {
    raw: *mut c_void,
    context: CallbackContext<SubscriberState>,
}

/// Rust handle for `MetricKit`'s shared `MXMetricManager`.
pub struct MetricManager;

unsafe extern "C" fn metric_event_trampoline(user_info: *mut c_void, payload_json: *const c_char) {
    if payload_json.is_null() {
        return;
    }

    let payload_json = unsafe { CStr::from_ptr(payload_json) };
    unsafe {
        CallbackContext::<SubscriberState>::with(
            user_info,
            "MXMetricManagerSubscriber delivery",
            |state| {
                let Ok(event) =
                    serde_json::from_slice::<MetricManagerEvent>(payload_json.to_bytes())
                else {
                    return;
                };

                let mut delegate = state.lock().unwrap_or_else(PoisonError::into_inner);
                match event.event.as_str() {
                    "didReceiveMetricPayloads" => {
                        delegate.did_receive_metric_payloads(event.metric_payloads);
                    }
                    "didReceiveDiagnosticPayloads" => {
                        delegate.did_receive_diagnostic_payloads(event.diagnostic_payloads);
                    }
                    _ => {}
                }
            },
        );
    }
}

impl MetricManager {
    /// Returns the shared `MXMetricManager` handle.
    #[must_use]
    pub const fn shared() -> Self {
        Self
    }

    /// Returns the shared `MXMetricManager` handle using Apple's naming.
    #[must_use]
    pub const fn shared_manager() -> Self {
        Self
    }

    /// Returns cached `MXMetricPayload` values from `MetricKit`.
    pub fn past_payloads(&self) -> Result<Vec<MetricPayload>, MetricKitError> {
        let ptr = unsafe { ffi::manager::mx_metric_manager_past_payloads_json() };
        if ptr.is_null() {
            return Ok(Vec::new());
        }
        decode_json(ptr)
    }

    /// Returns cached `MXDiagnosticPayload` values from `MetricKit`.
    pub fn past_diagnostic_payloads(&self) -> Result<Vec<DiagnosticPayload>, MetricKitError> {
        let ptr = unsafe { ffi::manager::mx_metric_manager_past_diagnostic_payloads_json() };
        if ptr.is_null() {
            return Ok(Vec::new());
        }
        decode_json(ptr)
    }

    /// Creates a `MetricKit` signpost log handle via `MXMetricManager.makeLogHandle(category:)`.
    pub fn make_log_handle(
        &self,
        category: impl AsRef<str>,
    ) -> Result<MetricLogHandle, MetricKitError> {
        let category = category.as_ref();
        let category = to_cstring("category", category)?;
        let mut raw = ptr::null_mut();
        let mut error_ptr = ptr::null_mut();
        let status = unsafe {
            ffi::signpost::mx_metric_manager_make_log_handle(
                category.as_ptr(),
                &mut raw,
                &mut error_ptr,
            )
        };
        if status != ffi::status::OK {
            return Err(from_swift(status, error_ptr));
        }
        if raw.is_null() {
            return Err(MetricKitError::FrameworkError(
                "Swift bridge returned a null MetricKit log handle".into(),
            ));
        }

        Ok(MetricLogHandle::from_raw(
            raw,
            category.to_string_lossy().into_owned(),
        ))
    }

    /// Calls `MXMetricManager.extendLaunchMeasurement(forTaskID:)`.
    pub fn extend_launch_measurement(
        &self,
        task_id: impl AsRef<str>,
    ) -> Result<(), MetricKitError> {
        Self::invoke_launch_measurement(
            task_id.as_ref(),
            ffi::manager::mx_metric_manager_extend_launch_measurement,
        )
    }

    /// Calls `MXMetricManager.finishExtendedLaunchMeasurement(forTaskID:)`.
    pub fn finish_extended_launch_measurement(
        &self,
        task_id: impl AsRef<str>,
    ) -> Result<(), MetricKitError> {
        Self::invoke_launch_measurement(
            task_id.as_ref(),
            ffi::manager::mx_metric_manager_finish_extended_launch_measurement,
        )
    }

    /// Registers an `MXMetricManagerSubscriber` delegate with `MetricKit`.
    pub fn subscribe<D>(&self, delegate: D) -> Result<MetricSubscription, MetricKitError>
    where
        D: MetricSubscriberDelegate + 'static,
    {
        let delegate: Box<dyn MetricSubscriberDelegate> = Box::new(delegate);
        let context = CallbackContext::new(Mutex::new(delegate));
        let user_info = context.retained_ptr();
        let mut raw = ptr::null_mut();
        let mut error_ptr = ptr::null_mut();
        let status = unsafe {
            ffi::manager::mx_metric_manager_add_subscriber(
                Some(metric_event_trampoline),
                user_info,
                Some(CallbackContext::<SubscriberState>::RELEASE),
                &raw mut raw,
                &raw mut error_ptr,
            )
        };
        if status != ffi::status::OK {
            unsafe { (CallbackContext::<SubscriberState>::RELEASE)(user_info) };
            return Err(from_swift(status, error_ptr));
        }
        if raw.is_null() {
            return Err(MetricKitError::FrameworkError(
                "Swift bridge returned a null MetricKit subscriber handle".into(),
            ));
        }

        Ok(MetricSubscription { raw, context })
    }

    fn invoke_launch_measurement(
        task_id: &str,
        callback: unsafe extern "C" fn(*const c_char, *mut *mut c_char) -> i32,
    ) -> Result<(), MetricKitError> {
        let task_id = to_cstring("task_id", task_id)?;
        let mut error_ptr = ptr::null_mut();
        let status = unsafe { callback(task_id.as_ptr(), &mut error_ptr) };
        if status != ffi::status::OK {
            return Err(from_swift(status, error_ptr));
        }
        Ok(())
    }
}

impl Default for MetricManager {
    fn default() -> Self {
        Self::shared()
    }
}

impl MetricSubscription {
    /// Unregisters this `MXMetricManagerSubscriber` handle.
    pub fn unsubscribe(self) {
        drop(self);
    }

    /// Returns whether this subscriber handle is still active.
    #[must_use]
    pub fn is_active(&self) -> bool {
        !self.raw.is_null()
    }
}

impl Drop for MetricSubscription {
    fn drop(&mut self) {
        self.context.deactivate();
        if self.raw.is_null() {
            return;
        }

        unsafe {
            ffi::manager::mx_metric_manager_remove_subscriber(self.raw);
            ffi::mx_object_release(self.raw);
        }
        self.raw = ptr::null_mut();
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::{Arc, Mutex};

    use doom_fish_utils::callback_context::CallbackContext;

    use super::{metric_event_trampoline, MetricSubscriberDelegate, SubscriberState};
    use crate::metric_payload::MetricPayload;

    struct Probe {
        deliveries: Arc<AtomicUsize>,
        drops: Arc<AtomicUsize>,
    }

    impl MetricSubscriberDelegate for Probe {
        fn did_receive_metric_payloads(&mut self, payloads: Vec<MetricPayload>) {
            assert!(payloads.is_empty());
            self.deliveries.fetch_add(1, Ordering::SeqCst);
        }
    }

    impl Drop for Probe {
        fn drop(&mut self) {
            self.drops.fetch_add(1, Ordering::SeqCst);
        }
    }

    #[test]
    fn late_delivery_after_the_rust_handle_is_gone_is_ignored_and_safe() {
        let deliveries = Arc::new(AtomicUsize::new(0));
        let drops = Arc::new(AtomicUsize::new(0));
        let delegate: Box<dyn MetricSubscriberDelegate> = Box::new(Probe {
            deliveries: Arc::clone(&deliveries),
            drops: Arc::clone(&drops),
        });
        let context: CallbackContext<SubscriberState> = CallbackContext::new(Mutex::new(delegate));
        let foreign = context.retained_ptr();
        let event = c"{\"event\":\"didReceiveMetricPayloads\",\"metricPayloads\":[]}";

        unsafe { metric_event_trampoline(foreign, event.as_ptr()) };
        assert_eq!(deliveries.load(Ordering::SeqCst), 1);

        drop(context);
        unsafe { metric_event_trampoline(foreign, event.as_ptr()) };
        assert_eq!(deliveries.load(Ordering::SeqCst), 1);
        assert_eq!(drops.load(Ordering::SeqCst), 0);

        unsafe { (CallbackContext::<SubscriberState>::RELEASE)(foreign) };
        assert_eq!(drops.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn malformed_delivery_is_ignored() {
        let deliveries = Arc::new(AtomicUsize::new(0));
        let drops = Arc::new(AtomicUsize::new(0));
        let delegate: Box<dyn MetricSubscriberDelegate> = Box::new(Probe {
            deliveries: Arc::clone(&deliveries),
            drops: Arc::clone(&drops),
        });
        let context: CallbackContext<SubscriberState> = CallbackContext::new(Mutex::new(delegate));

        unsafe { metric_event_trampoline(context.as_ptr(), c"[]".as_ptr()) };
        unsafe { metric_event_trampoline(context.as_ptr(), core::ptr::null()) };
        unsafe { metric_event_trampoline(core::ptr::null_mut(), c"{}".as_ptr()) };
        assert_eq!(deliveries.load(Ordering::SeqCst), 0);

        drop(context);
        assert_eq!(drops.load(Ordering::SeqCst), 1);
    }
}
