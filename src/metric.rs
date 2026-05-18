use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::average::{Average, Measurement};
use crate::error::MetricKitError;
use crate::histogram::Histogram;
use crate::private::{to_json_string, to_json_value};

/// Unit symbol MetricKit uses for `MXCellularConditionMetric` signal bars.
pub const SIGNAL_BARS_UNIT_SYMBOL: &str = "bars";
/// Unit symbol MetricKit uses for `MXDisplayMetric.averagePixelLuminance`.
pub const AVERAGE_PIXEL_LUMINANCE_UNIT_SYMBOL: &str = "apl";

/// Rust representation of MetricKit's `MXCPUMetric`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CpuMetric {
    /// Mirrors `MXCPUMetric.cumulativeCPUTime`.
    #[serde(rename = "cumulativeCPUTime")]
    pub cumulative_cpu_time: Measurement,
    /// Mirrors `MXCPUMetric.cumulativeCPUInstructions` when MetricKit provides it.
    #[serde(rename = "cumulativeCPUInstructions")]
    pub cumulative_cpu_instructions: Option<Measurement>,
}

/// Rust representation of MetricKit's `MXMemoryMetric`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MemoryMetric {
    /// Mirrors `MXMemoryMetric.peakMemoryUsage`.
    pub peak_memory_usage: Measurement,
    /// Mirrors `MXMemoryMetric.averageSuspendedMemory`.
    pub average_suspended_memory: Average,
}

/// Rust representation of MetricKit's `MXGPUMetric`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GpuMetric {
    /// Mirrors `MXGPUMetric.cumulativeGPUTime`.
    #[serde(rename = "cumulativeGPUTime")]
    pub cumulative_gpu_time: Measurement,
}

/// Rust representation of MetricKit's `MXAnimationMetric`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnimationMetric {
    /// Mirrors `MXAnimationMetric.scrollHitchTimeRatio`.
    pub scroll_hitch_time_ratio: Measurement,
    /// Mirrors `MXAnimationMetric.hitchTimeRatio` when MetricKit provides it.
    pub hitch_time_ratio: Option<Measurement>,
}

/// Rust representation of MetricKit's `MXAppLaunchMetric`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationLaunchMetric {
    /// Mirrors `MXAppLaunchMetric.histogrammedTimeToFirstDraw`.
    pub histogrammed_time_to_first_draw: Histogram,
    /// Mirrors `MXAppLaunchMetric.histogrammedApplicationResumeTime`.
    pub histogrammed_application_resume_time: Histogram,
    /// Mirrors `MXAppLaunchMetric.histogrammedOptimizedTimeToFirstDraw` when available.
    pub histogrammed_optimized_time_to_first_draw: Option<Histogram>,
    /// Mirrors `MXAppLaunchMetric.histogrammedExtendedLaunch` when available.
    pub histogrammed_extended_launch: Option<Histogram>,
}

/// Rust representation of MetricKit's `MXAppResponsivenessMetric`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationResponsivenessMetric {
    /// Mirrors `MXAppResponsivenessMetric.histogrammedApplicationHangTime`.
    pub histogrammed_application_hang_time: Histogram,
}

/// Rust representation of MetricKit's `MXAppRunTimeMetric`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationTimeMetric {
    /// Mirrors `MXAppRunTimeMetric.cumulativeForegroundTime`.
    pub cumulative_foreground_time: Measurement,
    /// Mirrors `MXAppRunTimeMetric.cumulativeBackgroundTime`.
    pub cumulative_background_time: Measurement,
    /// Mirrors `MXAppRunTimeMetric.cumulativeBackgroundAudioTime`.
    pub cumulative_background_audio_time: Measurement,
    /// Mirrors `MXAppRunTimeMetric.cumulativeBackgroundLocationTime`.
    pub cumulative_background_location_time: Measurement,
}

/// Rust representation of MetricKit's `MXLocationActivityMetric`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocationActivityMetric {
    /// Mirrors `MXLocationActivityMetric.cumulativeBestAccuracyTime`.
    pub cumulative_best_accuracy_time: Measurement,
    /// Mirrors `MXLocationActivityMetric.cumulativeBestAccuracyForNavigationTime`.
    pub cumulative_best_accuracy_for_navigation_time: Measurement,
    /// Mirrors `MXLocationActivityMetric.cumulativeNearestTenMetersAccuracyTime`.
    pub cumulative_nearest_ten_meters_accuracy_time: Measurement,
    /// Mirrors `MXLocationActivityMetric.cumulativeHundredMetersAccuracyTime`.
    pub cumulative_hundred_meters_accuracy_time: Measurement,
    /// Mirrors `MXLocationActivityMetric.cumulativeKilometerAccuracyTime`.
    pub cumulative_kilometer_accuracy_time: Measurement,
    /// Mirrors `MXLocationActivityMetric.cumulativeThreeKilometersAccuracyTime`.
    pub cumulative_three_kilometers_accuracy_time: Measurement,
}

/// Rust representation of MetricKit's `MXNetworkTransferMetric`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkTransferMetric {
    /// Mirrors `MXNetworkTransferMetric.cumulativeWiFiUpload`.
    pub cumulative_wifi_upload: Measurement,
    /// Mirrors `MXNetworkTransferMetric.cumulativeWiFiDownload`.
    pub cumulative_wifi_download: Measurement,
    /// Mirrors `MXNetworkTransferMetric.cumulativeCellularUpload`.
    pub cumulative_cellular_upload: Measurement,
    /// Mirrors `MXNetworkTransferMetric.cumulativeCellularDownload`.
    pub cumulative_cellular_download: Measurement,
}

/// Rust representation of MetricKit's `MXDiskIOMetric`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiskIoMetric {
    /// Mirrors `MXDiskIOMetric.cumulativeLogicalWrites`.
    pub cumulative_logical_writes: Measurement,
}

/// Rust representation of MetricKit's `MXDisplayMetric`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DisplayMetric {
    /// Mirrors `MXDisplayMetric.averagePixelLuminance` when MetricKit provides it.
    pub average_pixel_luminance: Option<Average>,
}

/// Rust representation of MetricKit's `MXCellularConditionMetric`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CellularConditionMetric {
    /// Mirrors `MXCellularConditionMetric.histogrammedCellularConditionTime`.
    pub histogrammed_cellular_condition_time: Histogram,
}

/// Rust representation of MetricKit's `MXForegroundExitData`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ForegroundExitData {
    /// Mirrors `MXForegroundExitData.cumulativeNormalAppExitCount`.
    pub cumulative_normal_app_exit_count: u64,
    /// Mirrors `MXForegroundExitData.cumulativeMemoryResourceLimitExitCount`.
    pub cumulative_memory_resource_limit_exit_count: u64,
    /// Mirrors `MXForegroundExitData.cumulativeBadAccessExitCount`.
    pub cumulative_bad_access_exit_count: u64,
    /// Mirrors `MXForegroundExitData.cumulativeAbnormalExitCount`.
    pub cumulative_abnormal_exit_count: u64,
    /// Mirrors `MXForegroundExitData.cumulativeIllegalInstructionExitCount`.
    pub cumulative_illegal_instruction_exit_count: u64,
    /// Mirrors `MXForegroundExitData.cumulativeAppWatchdogExitCount`.
    pub cumulative_app_watchdog_exit_count: u64,
}

/// Rust representation of MetricKit's `MXBackgroundExitData`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackgroundExitData {
    /// Mirrors `MXBackgroundExitData.cumulativeNormalAppExitCount`.
    pub cumulative_normal_app_exit_count: u64,
    /// Mirrors `MXBackgroundExitData.cumulativeMemoryResourceLimitExitCount`.
    pub cumulative_memory_resource_limit_exit_count: u64,
    /// Mirrors `MXBackgroundExitData.cumulativeCPUResourceLimitExitCount`.
    #[serde(rename = "cumulativeCPUResourceLimitExitCount")]
    pub cumulative_cpu_resource_limit_exit_count: u64,
    /// Mirrors `MXBackgroundExitData.cumulativeMemoryPressureExitCount`.
    pub cumulative_memory_pressure_exit_count: u64,
    /// Mirrors `MXBackgroundExitData.cumulativeBadAccessExitCount`.
    pub cumulative_bad_access_exit_count: u64,
    /// Mirrors `MXBackgroundExitData.cumulativeAbnormalExitCount`.
    pub cumulative_abnormal_exit_count: u64,
    /// Mirrors `MXBackgroundExitData.cumulativeIllegalInstructionExitCount`.
    pub cumulative_illegal_instruction_exit_count: u64,
    /// Mirrors `MXBackgroundExitData.cumulativeAppWatchdogExitCount`.
    pub cumulative_app_watchdog_exit_count: u64,
    /// Mirrors `MXBackgroundExitData.cumulativeSuspendedWithLockedFileExitCount`.
    pub cumulative_suspended_with_locked_file_exit_count: u64,
    /// Mirrors `MXBackgroundExitData.cumulativeBackgroundTaskAssertionTimeoutExitCount`.
    pub cumulative_background_task_assertion_timeout_exit_count: u64,
}

/// Rust representation of MetricKit's `MXAppExitMetric`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationExitMetric {
    /// Mirrors `MXAppExitMetric.foregroundExitData`.
    pub foreground_exit_data: ForegroundExitData,
    /// Mirrors `MXAppExitMetric.backgroundExitData`.
    pub background_exit_data: BackgroundExitData,
}

/// Rust representation of MetricKit's `MXDiskSpaceUsageMetric`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiskSpaceUsageMetric {
    /// Mirrors `MXDiskSpaceUsageMetric.totalBinaryFileSize`.
    pub total_binary_file_size: Measurement,
    /// Mirrors `MXDiskSpaceUsageMetric.totalBinaryFileCount`.
    pub total_binary_file_count: i64,
    /// Mirrors `MXDiskSpaceUsageMetric.totalDataFileSize`.
    pub total_data_file_size: Measurement,
    /// Mirrors `MXDiskSpaceUsageMetric.totalDataFileCount`.
    pub total_data_file_count: i64,
    /// Mirrors `MXDiskSpaceUsageMetric.totalCacheFolderSize`.
    pub total_cache_folder_size: Measurement,
    /// Mirrors `MXDiskSpaceUsageMetric.totalCloneSize`.
    pub total_clone_size: Measurement,
    /// Mirrors `MXDiskSpaceUsageMetric.totalDiskSpaceUsedSize`.
    pub total_disk_space_used_size: Measurement,
    /// Mirrors `MXDiskSpaceUsageMetric.totalDiskSpaceCapacity`.
    pub total_disk_space_capacity: Measurement,
}

macro_rules! impl_metric_json_methods {
    ($($type:ty),+ $(,)?) => {
        $(
            impl $type {
                /// Returns the JSON representation of this MetricKit model.
                pub fn json_representation(&self) -> Result<String, MetricKitError> {
                    to_json_string(self)
                }

                /// Returns the dictionary representation of this MetricKit model.
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
