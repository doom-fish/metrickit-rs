use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::error::MetricKitError;
use crate::meta_data::MetaData;
use crate::metric::{
    AnimationMetric, ApplicationExitMetric, ApplicationLaunchMetric,
    ApplicationResponsivenessMetric, ApplicationTimeMetric, CellularConditionMetric, CpuMetric,
    DiskIoMetric, DiskSpaceUsageMetric, DisplayMetric, GpuMetric, LocationActivityMetric,
    MemoryMetric, NetworkTransferMetric,
};
use crate::private::{to_json_string, to_json_value};
use crate::signpost::SignpostMetric;

/// Rust representation of MetricKit's `MXMetricPayload`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetricPayload {
    /// Mirrors `MXMetricPayload.latestApplicationVersion`.
    pub latest_application_version: String,
    /// Mirrors `MXMetricPayload.includesMultipleApplicationVersions`.
    pub includes_multiple_application_versions: bool,
    /// Mirrors `MXMetricPayload.timeStampBegin`.
    pub time_stamp_begin: f64,
    /// Mirrors `MXMetricPayload.timeStampEnd`.
    pub time_stamp_end: f64,
    /// Mirrors `MXMetricPayload.cpuMetrics` when MetricKit provides it.
    pub cpu_metrics: Option<CpuMetric>,
    /// Mirrors `MXMetricPayload.memoryMetrics` when MetricKit provides it.
    pub memory_metrics: Option<MemoryMetric>,
    /// Mirrors `MXMetricPayload.gpuMetrics` when MetricKit provides it.
    pub gpu_metrics: Option<GpuMetric>,
    /// Mirrors `MXMetricPayload.animationMetrics` when MetricKit provides it.
    pub animation_metrics: Option<AnimationMetric>,
    /// Mirrors `MXMetricPayload.applicationLaunchMetrics` when MetricKit provides it.
    pub application_launch_metrics: Option<ApplicationLaunchMetric>,
    /// Mirrors `MXMetricPayload.applicationResponsivenessMetrics` when MetricKit provides it.
    pub application_responsiveness_metrics: Option<ApplicationResponsivenessMetric>,
    /// Mirrors `MXMetricPayload.applicationTimeMetrics` when MetricKit provides it.
    pub application_time_metrics: Option<ApplicationTimeMetric>,
    /// Mirrors `MXMetricPayload.locationActivityMetrics` when MetricKit provides it.
    pub location_activity_metrics: Option<LocationActivityMetric>,
    /// Mirrors `MXMetricPayload.networkTransferMetrics` when MetricKit provides it.
    pub network_transfer_metrics: Option<NetworkTransferMetric>,
    /// Mirrors `MXMetricPayload.diskIOMetrics` when MetricKit provides it.
    #[serde(rename = "diskIOMetrics")]
    pub disk_io_metrics: Option<DiskIoMetric>,
    /// Mirrors `MXMetricPayload.displayMetrics` when MetricKit provides it.
    pub display_metrics: Option<DisplayMetric>,
    /// Mirrors `MXMetricPayload.cellularConditionMetrics` when MetricKit provides it.
    pub cellular_condition_metrics: Option<CellularConditionMetric>,
    /// Mirrors `MXMetricPayload.applicationExitMetrics` when MetricKit provides it.
    pub application_exit_metrics: Option<ApplicationExitMetric>,
    /// Mirrors `MXMetricPayload.diskSpaceUsageMetrics` when MetricKit provides it.
    pub disk_space_usage_metrics: Option<DiskSpaceUsageMetric>,
    /// Mirrors `MXMetricPayload.signpostMetrics`.
    #[serde(default)]
    pub signpost_metrics: Vec<SignpostMetric>,
    /// Mirrors `MXMetricPayload.metaData` when MetricKit provides it.
    #[serde(rename = "metaData")]
    pub meta_data: Option<MetaData>,
}

impl MetricPayload {
    /// Returns the JSON representation of this `MXMetricPayload` model.
    pub fn json_representation(&self) -> Result<String, MetricKitError> {
        to_json_string(self)
    }

    /// Returns the dictionary representation of this `MXMetricPayload` model.
    pub fn dictionary_representation(&self) -> Result<Value, MetricKitError> {
        to_json_value(self)
    }
}
