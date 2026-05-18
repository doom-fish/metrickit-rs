#![doc = include_str!("../README.md")]
//!
//! ---
//!
//! # API documentation
//!
//! Safe Rust bindings for Apple's
//! [MetricKit](https://developer.apple.com/documentation/metrickit)
//! framework.
#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(
    clippy::missing_const_for_fn,
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::new_without_default
)]

/// MetricKit average and measurement models.
pub mod average;
/// MetricKit call-stack tree models.
pub mod call_stack_tree;
/// MetricKit CPU exception diagnostic models.
pub mod cpu_exception_diagnostic;
/// MetricKit crash diagnostic models.
pub mod crash_diagnostic;
/// Shared MetricKit diagnostic context models.
pub mod diagnostic;
/// MetricKit diagnostic payload models.
pub mod diagnostic_payload;
/// MetricKit disk-write exception diagnostic models.
pub mod disk_write_exception_diagnostic;
/// Error types returned by the MetricKit wrapper.
pub mod error;
/// Low-level FFI bindings backing the MetricKit wrapper.
pub mod ffi;
/// MetricKit hang diagnostic models.
pub mod hang_diagnostic;
/// MetricKit histogram models.
pub mod histogram;
/// Re-exports for MetricKit manager types.
pub mod manager;
/// MetricKit metadata models.
pub mod meta_data;
/// MetricKit metric models.
pub mod metric;
/// MetricKit manager and subscriber APIs.
pub mod metric_manager;
/// MetricKit metric payload models.
pub mod metric_payload;
/// Aggregated MetricKit model re-exports.
pub mod metrics;
mod private;
/// MetricKit signpost models and helpers.
pub mod signpost;

pub use average::{Average, Measurement};
pub use call_stack_tree::CallStackTree;
pub use cpu_exception_diagnostic::CpuExceptionDiagnostic;
pub use crash_diagnostic::{CrashDiagnostic, CrashDiagnosticObjectiveCExceptionReason};
pub use diagnostic::Diagnostic;
pub use diagnostic_payload::DiagnosticPayload;
pub use disk_write_exception_diagnostic::DiskWriteExceptionDiagnostic;
pub use error::MetricKitError;
pub use hang_diagnostic::HangDiagnostic;
pub use histogram::{Histogram, HistogramBucket};
pub use manager::{
    LaunchTaskId, MetricManager, MetricSubscriberCallbacks, MetricSubscriberDelegate,
    MetricSubscription,
};
pub use meta_data::MetaData;
pub use metric::{
    AnimationMetric, ApplicationExitMetric, ApplicationLaunchMetric,
    ApplicationResponsivenessMetric, ApplicationTimeMetric, BackgroundExitData,
    CellularConditionMetric, CpuMetric, DiskIoMetric, DiskSpaceUsageMetric, DisplayMetric,
    ForegroundExitData, GpuMetric, LocationActivityMetric, MemoryMetric, NetworkTransferMetric,
    AVERAGE_PIXEL_LUMINANCE_UNIT_SYMBOL, SIGNAL_BARS_UNIT_SYMBOL,
};
pub use metric_payload::MetricPayload;
pub use signpost::{
    MetricLogHandle, SignpostId, SignpostIntervalData, SignpostMetric, SignpostRecord,
};

/// Common imports.
pub mod prelude {
    pub use crate::average::{Average, Measurement};
    pub use crate::call_stack_tree::CallStackTree;
    pub use crate::cpu_exception_diagnostic::CpuExceptionDiagnostic;
    pub use crate::crash_diagnostic::{CrashDiagnostic, CrashDiagnosticObjectiveCExceptionReason};
    pub use crate::diagnostic::Diagnostic;
    pub use crate::diagnostic_payload::DiagnosticPayload;
    pub use crate::disk_write_exception_diagnostic::DiskWriteExceptionDiagnostic;
    pub use crate::error::MetricKitError;
    pub use crate::hang_diagnostic::HangDiagnostic;
    pub use crate::histogram::{Histogram, HistogramBucket};
    pub use crate::manager::{
        LaunchTaskId, MetricManager, MetricSubscriberCallbacks, MetricSubscriberDelegate,
        MetricSubscription,
    };
    pub use crate::meta_data::MetaData;
    pub use crate::metric::{
        AnimationMetric, ApplicationExitMetric, ApplicationLaunchMetric,
        ApplicationResponsivenessMetric, ApplicationTimeMetric, BackgroundExitData,
        CellularConditionMetric, CpuMetric, DiskIoMetric, DiskSpaceUsageMetric, DisplayMetric,
        ForegroundExitData, GpuMetric, LocationActivityMetric, MemoryMetric, NetworkTransferMetric,
        AVERAGE_PIXEL_LUMINANCE_UNIT_SYMBOL, SIGNAL_BARS_UNIT_SYMBOL,
    };
    pub use crate::metric_payload::MetricPayload;
    pub use crate::signpost::{
        MetricLogHandle, SignpostId, SignpostIntervalData, SignpostMetric, SignpostRecord,
    };
}
