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

pub mod error;
pub mod ffi;
pub mod manager;
pub mod metrics;
mod private;

pub use error::MetricKitError;
pub use manager::{
    MetricManager, MetricSubscriberCallbacks, MetricSubscriberDelegate, MetricSubscription,
};
pub use metrics::{
    AnimationMetric, ApplicationLaunchMetric, ApplicationResponsivenessMetric,
    ApplicationTimeMetric, Average, CpuExceptionDiagnostic, CpuMetric, CrashDiagnostic,
    CrashDiagnosticObjectiveCExceptionReason, DiagnosticPayload, DiskIoMetric,
    DiskWriteExceptionDiagnostic, GpuMetric, HangDiagnostic, Histogram, HistogramBucket,
    LocationActivityMetric, Measurement, MemoryMetric, MetricPayload, NetworkTransferMetric,
};

/// Common imports.
pub mod prelude {
    pub use crate::error::MetricKitError;
    pub use crate::manager::{
        MetricManager, MetricSubscriberCallbacks, MetricSubscriberDelegate, MetricSubscription,
    };
    pub use crate::metrics::{
        AnimationMetric, ApplicationLaunchMetric, ApplicationResponsivenessMetric,
        ApplicationTimeMetric, Average, CpuExceptionDiagnostic, CpuMetric, CrashDiagnostic,
        CrashDiagnosticObjectiveCExceptionReason, DiagnosticPayload, DiskIoMetric,
        DiskWriteExceptionDiagnostic, GpuMetric, HangDiagnostic, Histogram, HistogramBucket,
        LocationActivityMetric, Measurement, MemoryMetric, MetricPayload, NetworkTransferMetric,
    };
}
