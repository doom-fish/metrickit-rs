use core::ffi::{c_char, c_void};
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::ptr;
use std::sync::Mutex;

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

struct CallbackState {
    delegate: Mutex<Box<dyn MetricSubscriberDelegate>>,
}

/// Active subscriber registration returned by `MXMetricManager.add(_:)`.
pub struct MetricSubscription {
    raw: *mut c_void,
    _callback_state: Box<CallbackState>,
}

/// Rust handle for `MetricKit`'s shared `MXMetricManager`.
pub struct MetricManager;

unsafe extern "C" fn metric_event_trampoline(user_info: *mut c_void, payload_json: *const c_char) {
    if user_info.is_null() || payload_json.is_null() {
        return;
    }

    let _ = catch_unwind(AssertUnwindSafe(|| {
        let state = unsafe { &*user_info.cast::<CallbackState>() };
        let payload_json = unsafe { core::ffi::CStr::from_ptr(payload_json) }
            .to_string_lossy()
            .into_owned();
        let Ok(event): Result<MetricManagerEvent, _> = serde_json::from_str(&payload_json) else {
            return;
        };

        let mut delegate = match state.delegate.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };

        match event.event.as_str() {
            "didReceiveMetricPayloads" => {
                delegate.did_receive_metric_payloads(event.metric_payloads);
            }
            "didReceiveDiagnosticPayloads" => {
                delegate.did_receive_diagnostic_payloads(event.diagnostic_payloads);
            }
            _ => {}
        }
    }));
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
        let callback_state = Box::new(CallbackState {
            delegate: Mutex::new(Box::new(delegate)),
        });
        let user_info = std::ptr::from_ref(callback_state.as_ref())
            .cast_mut()
            .cast::<c_void>();
        let mut raw = ptr::null_mut();
        let mut error_ptr = ptr::null_mut();
        let status = unsafe {
            ffi::manager::mx_metric_manager_add_subscriber(
                Some(metric_event_trampoline),
                user_info,
                &mut raw,
                &mut error_ptr,
            )
        };
        if status != ffi::status::OK {
            return Err(from_swift(status, error_ptr));
        }
        if raw.is_null() {
            return Err(MetricKitError::FrameworkError(
                "Swift bridge returned a null MetricKit subscriber handle".into(),
            ));
        }

        Ok(MetricSubscription {
            raw,
            _callback_state: callback_state,
        })
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
