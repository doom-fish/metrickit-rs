use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::average::{Average, Measurement};
use crate::error::MetricKitError;
use crate::histogram::Histogram;
use crate::private::{to_json_string, to_json_value};

pub const SIGNAL_BARS_UNIT_SYMBOL: &str = "bars";
pub const AVERAGE_PIXEL_LUMINANCE_UNIT_SYMBOL: &str = "apl";

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CpuMetric {
    #[serde(rename = "cumulativeCPUTime")]
    pub cumulative_cpu_time: Measurement,
    #[serde(rename = "cumulativeCPUInstructions")]
    pub cumulative_cpu_instructions: Option<Measurement>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MemoryMetric {
    pub peak_memory_usage: Measurement,
    pub average_suspended_memory: Average,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GpuMetric {
    #[serde(rename = "cumulativeGPUTime")]
    pub cumulative_gpu_time: Measurement,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnimationMetric {
    pub scroll_hitch_time_ratio: Measurement,
    pub hitch_time_ratio: Option<Measurement>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationLaunchMetric {
    pub histogrammed_time_to_first_draw: Histogram,
    pub histogrammed_application_resume_time: Histogram,
    pub histogrammed_optimized_time_to_first_draw: Option<Histogram>,
    pub histogrammed_extended_launch: Option<Histogram>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationResponsivenessMetric {
    pub histogrammed_application_hang_time: Histogram,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationTimeMetric {
    pub cumulative_foreground_time: Measurement,
    pub cumulative_background_time: Measurement,
    pub cumulative_background_audio_time: Measurement,
    pub cumulative_background_location_time: Measurement,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocationActivityMetric {
    pub cumulative_best_accuracy_time: Measurement,
    pub cumulative_best_accuracy_for_navigation_time: Measurement,
    pub cumulative_nearest_ten_meters_accuracy_time: Measurement,
    pub cumulative_hundred_meters_accuracy_time: Measurement,
    pub cumulative_kilometer_accuracy_time: Measurement,
    pub cumulative_three_kilometers_accuracy_time: Measurement,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkTransferMetric {
    pub cumulative_wifi_upload: Measurement,
    pub cumulative_wifi_download: Measurement,
    pub cumulative_cellular_upload: Measurement,
    pub cumulative_cellular_download: Measurement,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiskIoMetric {
    pub cumulative_logical_writes: Measurement,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DisplayMetric {
    pub average_pixel_luminance: Option<Average>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CellularConditionMetric {
    pub histogrammed_cellular_condition_time: Histogram,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ForegroundExitData {
    pub cumulative_normal_app_exit_count: u64,
    pub cumulative_memory_resource_limit_exit_count: u64,
    pub cumulative_bad_access_exit_count: u64,
    pub cumulative_abnormal_exit_count: u64,
    pub cumulative_illegal_instruction_exit_count: u64,
    pub cumulative_app_watchdog_exit_count: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackgroundExitData {
    pub cumulative_normal_app_exit_count: u64,
    pub cumulative_memory_resource_limit_exit_count: u64,
    #[serde(rename = "cumulativeCPUResourceLimitExitCount")]
    pub cumulative_cpu_resource_limit_exit_count: u64,
    pub cumulative_memory_pressure_exit_count: u64,
    pub cumulative_bad_access_exit_count: u64,
    pub cumulative_abnormal_exit_count: u64,
    pub cumulative_illegal_instruction_exit_count: u64,
    pub cumulative_app_watchdog_exit_count: u64,
    pub cumulative_suspended_with_locked_file_exit_count: u64,
    pub cumulative_background_task_assertion_timeout_exit_count: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationExitMetric {
    pub foreground_exit_data: ForegroundExitData,
    pub background_exit_data: BackgroundExitData,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiskSpaceUsageMetric {
    pub total_binary_file_size: Measurement,
    pub total_binary_file_count: i64,
    pub total_data_file_size: Measurement,
    pub total_data_file_count: i64,
    pub total_cache_folder_size: Measurement,
    pub total_clone_size: Measurement,
    pub total_disk_space_used_size: Measurement,
    pub total_disk_space_capacity: Measurement,
}

macro_rules! impl_metric_json_methods {
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

impl_metric_json_methods!(
    CpuMetric,
    MemoryMetric,
    GpuMetric,
    AnimationMetric,
    ApplicationLaunchMetric,
    ApplicationResponsivenessMetric,
    ApplicationTimeMetric,
    LocationActivityMetric,
    NetworkTransferMetric,
    DiskIoMetric,
    DisplayMetric,
    CellularConditionMetric,
    ApplicationExitMetric,
    DiskSpaceUsageMetric,
);
