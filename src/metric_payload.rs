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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetricPayload {
    pub latest_application_version: String,
    pub includes_multiple_application_versions: bool,
    pub time_stamp_begin: f64,
    pub time_stamp_end: f64,
    pub cpu_metrics: Option<CpuMetric>,
    pub memory_metrics: Option<MemoryMetric>,
    pub gpu_metrics: Option<GpuMetric>,
    pub animation_metrics: Option<AnimationMetric>,
    pub application_launch_metrics: Option<ApplicationLaunchMetric>,
    pub application_responsiveness_metrics: Option<ApplicationResponsivenessMetric>,
    pub application_time_metrics: Option<ApplicationTimeMetric>,
    pub location_activity_metrics: Option<LocationActivityMetric>,
    pub network_transfer_metrics: Option<NetworkTransferMetric>,
    #[serde(rename = "diskIOMetrics")]
    pub disk_io_metrics: Option<DiskIoMetric>,
    pub display_metrics: Option<DisplayMetric>,
    pub cellular_condition_metrics: Option<CellularConditionMetric>,
    pub application_exit_metrics: Option<ApplicationExitMetric>,
    pub disk_space_usage_metrics: Option<DiskSpaceUsageMetric>,
    #[serde(default)]
    pub signpost_metrics: Vec<SignpostMetric>,
    #[serde(rename = "metaData")]
    pub meta_data: Option<MetaData>,
}

impl MetricPayload {
    pub fn json_representation(&self) -> Result<String, MetricKitError> {
        to_json_string(self)
    }

    pub fn dictionary_representation(&self) -> Result<Value, MetricKitError> {
        to_json_value(self)
    }
}
