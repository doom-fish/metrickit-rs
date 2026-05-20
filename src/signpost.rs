use core::ffi::{c_char, c_void};
use core::ptr;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::average::{Average, Measurement};
use crate::error::{from_swift, MetricKitError};
use crate::ffi;
use crate::histogram::Histogram;
use crate::private::{to_cstring, to_json_string, to_json_value};

/// Rust wrapper for a signpost identifier used by `MetricKit` signpost APIs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SignpostId(
    /// Stores the raw signpost identifier value.
    pub u64,
);

impl SignpostId {
    /// Returns the raw signpost identifier value.
    #[must_use]
    pub const fn raw(self) -> u64 {
        self.0
    }
}

/// `MetricKit` signpost log handle created by `MXMetricManager.makeLogHandle(category:)`.
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

    /// Returns the `MetricKit` signpost category for this log handle.
    #[must_use]
    pub fn category(&self) -> &str {
        &self.category
    }

    /// Returns whether this `MetricKit` log handle still owns a native object.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        !self.raw.is_null()
    }

    /// Creates a new signpost identifier backed by this `MetricKit` log handle.
    pub fn make_signpost_id(&self) -> Result<SignpostId, MetricKitError> {
        if self.raw.is_null() {
            return Err(MetricKitError::FrameworkError(
                "MetricKit log handle has already been released".into(),
            ));
        }

        let raw_id = unsafe { ffi::signpost::mx_signpost_log_make_id(self.raw) };
        Ok(SignpostId(raw_id))
    }

    /// Emits a one-shot `MetricKit` signpost event.
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

    /// Begins a `MetricKit` signpost interval.
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

    /// Begins an animation interval correlated with `MXAnimationMetric` data.
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

    /// Ends a `MetricKit` signpost interval.
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

/// Rust representation of `MetricKit`'s `MXSignpostIntervalData`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignpostIntervalData {
    /// Mirrors `MXSignpostIntervalData.histogrammedSignpostDuration`.
    pub histogrammed_signpost_duration: Histogram,
    /// Mirrors `MXSignpostIntervalData.cumulativeCPUTime` when `MetricKit` provides it.
    #[serde(rename = "cumulativeCPUTime")]
    pub cumulative_cpu_time: Option<Measurement>,
    /// Mirrors `MXSignpostIntervalData.averageMemory` when `MetricKit` provides it.
    pub average_memory: Option<Average>,
    /// Mirrors `MXSignpostIntervalData.cumulativeLogicalWrites` when `MetricKit` provides it.
    pub cumulative_logical_writes: Option<Measurement>,
    /// Mirrors `MXSignpostIntervalData.cumulativeHitchTimeRatio` when `MetricKit` provides it.
    pub cumulative_hitch_time_ratio: Option<Measurement>,
}

/// Rust representation of `MetricKit`'s `MXSignpostMetric`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignpostMetric {
    /// Mirrors `MXSignpostMetric.signpostName`.
    pub signpost_name: String,
    /// Mirrors `MXSignpostMetric.signpostCategory`.
    pub signpost_category: String,
    /// Mirrors `MXSignpostMetric.signpostIntervalData` when `MetricKit` provides it.
    pub signpost_interval_data: Option<SignpostIntervalData>,
    /// Mirrors `MXSignpostMetric.totalCount`.
    pub total_count: u64,
}

/// Rust representation of `MetricKit`'s `MXSignpost`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignpostRecord {
    /// Mirrors `MXSignpost.subsystem`.
    pub subsystem: String,
    /// Mirrors `MXSignpost.category`.
    pub category: String,
    /// Mirrors `MXSignpost.name`.
    pub name: String,
    /// Mirrors `MXSignpost.beginTimeStamp`.
    pub begin_time_stamp: f64,
    /// Mirrors `MXSignpost.endTimeStamp` when `MetricKit` provides it.
    pub end_time_stamp: Option<f64>,
    /// Mirrors `MXSignpost.duration` when `MetricKit` provides it.
    pub duration: Option<Measurement>,
    /// Mirrors `MXSignpost.isInterval`.
    pub is_interval: bool,
}

macro_rules! impl_signpost_json_methods {
    ($($type:ty),+ $(,)?) => {
        $(
            impl $type {
                /// Returns the JSON representation of this `MetricKit` signpost model.
                pub fn json_representation(&self) -> Result<String, MetricKitError> {
                    to_json_string(self)
                }

                /// Returns the dictionary representation of this `MetricKit` signpost model.
                pub fn dictionary_representation(&self) -> Result<Value, MetricKitError> {
                    to_json_value(self)
                }
            }
        )+
    };
}

impl_signpost_json_methods!(SignpostIntervalData, SignpostMetric, SignpostRecord);
