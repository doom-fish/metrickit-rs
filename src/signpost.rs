use core::ffi::{c_char, c_void};
use core::ptr;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::average::{Average, Measurement};
use crate::error::{from_swift, MetricKitError};
use crate::ffi;
use crate::histogram::Histogram;
use crate::private::{to_cstring, to_json_string, to_json_value};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SignpostId(pub u64);

impl SignpostId {
    #[must_use]
    pub const fn raw(self) -> u64 {
        self.0
    }
}

pub struct MetricLogHandle {
    raw: *mut c_void,
    category: String,
}

impl core::fmt::Debug for MetricLogHandle {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("MetricLogHandle")
            .field("raw", &self.raw)
            .field("category", &self.category)
            .field("is_valid", &self.is_valid())
            .finish()
    }
}

impl MetricLogHandle {
    pub(crate) fn from_raw(raw: *mut c_void, category: String) -> Self {
        Self { raw, category }
    }

    #[must_use]
    pub fn category(&self) -> &str {
        &self.category
    }

    #[must_use]
    pub fn is_valid(&self) -> bool {
        !self.raw.is_null()
    }

    pub fn make_signpost_id(&self) -> Result<SignpostId, MetricKitError> {
        if self.raw.is_null() {
            return Err(MetricKitError::FrameworkError(
                "MetricKit log handle has already been released".into(),
            ));
        }

        let raw_id = unsafe { ffi::signpost::mx_signpost_log_make_id(self.raw) };
        Ok(SignpostId(raw_id))
    }

    pub fn emit_event(
        &self,
        signpost_id: SignpostId,
        name: impl AsRef<str>,
    ) -> Result<(), MetricKitError> {
        self.emit_named(
            signpost_id,
            name.as_ref(),
            ffi::signpost::mx_signpost_event_emit,
        )
    }

    pub fn interval_begin(
        &self,
        signpost_id: SignpostId,
        name: impl AsRef<str>,
    ) -> Result<(), MetricKitError> {
        self.emit_named(
            signpost_id,
            name.as_ref(),
            ffi::signpost::mx_signpost_interval_begin,
        )
    }

    pub fn animation_interval_begin(
        &self,
        signpost_id: SignpostId,
        name: impl AsRef<str>,
    ) -> Result<(), MetricKitError> {
        self.emit_named(
            signpost_id,
            name.as_ref(),
            ffi::signpost::mx_signpost_animation_interval_begin,
        )
    }

    pub fn interval_end(
        &self,
        signpost_id: SignpostId,
        name: impl AsRef<str>,
    ) -> Result<(), MetricKitError> {
        self.emit_named(
            signpost_id,
            name.as_ref(),
            ffi::signpost::mx_signpost_interval_end,
        )
    }

    fn emit_named(
        &self,
        signpost_id: SignpostId,
        name: &str,
        callback: unsafe extern "C" fn(*mut c_void, u64, *const c_char, *mut *mut c_char) -> i32,
    ) -> Result<(), MetricKitError> {
        if self.raw.is_null() {
            return Err(MetricKitError::FrameworkError(
                "MetricKit log handle has already been released".into(),
            ));
        }

        let name = to_cstring("signpost name", name)?;
        let mut error_ptr = ptr::null_mut();
        let status =
            unsafe { callback(self.raw, signpost_id.raw(), name.as_ptr(), &mut error_ptr) };
        if status != ffi::status::OK {
            return Err(from_swift(status, error_ptr));
        }
        Ok(())
    }
}

impl Drop for MetricLogHandle {
    fn drop(&mut self) {
        if self.raw.is_null() {
            return;
        }

        unsafe { ffi::mx_object_release(self.raw) };
        self.raw = ptr::null_mut();
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignpostIntervalData {
    pub histogrammed_signpost_duration: Histogram,
    #[serde(rename = "cumulativeCPUTime")]
    pub cumulative_cpu_time: Option<Measurement>,
    pub average_memory: Option<Average>,
    pub cumulative_logical_writes: Option<Measurement>,
    pub cumulative_hitch_time_ratio: Option<Measurement>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignpostMetric {
    pub signpost_name: String,
    pub signpost_category: String,
    pub signpost_interval_data: Option<SignpostIntervalData>,
    pub total_count: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignpostRecord {
    pub subsystem: String,
    pub category: String,
    pub name: String,
    pub begin_time_stamp: f64,
    pub end_time_stamp: Option<f64>,
    pub duration: Option<Measurement>,
    pub is_interval: bool,
}

macro_rules! impl_signpost_json_methods {
    ($($type:ty),+ $(,)?) => {
        $(
            impl $type {
                pub fn json_representation(&self) -> Result<String, MetricKitError> {
                    to_json_string(self)
                }

                pub fn dictionary_representation(&self) -> Result<Value, MetricKitError> {
                    to_json_value(self)
                }
            }
        )+
    };
}

impl_signpost_json_methods!(SignpostIntervalData, SignpostMetric, SignpostRecord);
