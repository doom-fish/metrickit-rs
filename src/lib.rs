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

pub mod average;
pub mod call_stack_tree;
pub mod cpu_exception_diagnostic;
pub mod crash_diagnostic;
pub mod diagnostic;
pub mod diagnostic_payload;
pub mod disk_write_exception_diagnostic;
pub mod error;
pub mod ffi;
pub mod hang_diagnostic;
pub mod histogram;
pub mod manager;
pub mod meta_data;
pub mod metric;
pub mod metric_manager;
pub mod metric_payload;
pub mod metrics;
mod private;
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
