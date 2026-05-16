use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Measurement {
    pub value: f64,
    pub unit_symbol: String,
    pub unit_type: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Average {
    pub average_measurement: Measurement,
    pub sample_count: i64,
    pub standard_deviation: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistogramBucket {
    pub bucket_start: Measurement,
    pub bucket_end: Measurement,
    pub bucket_count: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Histogram {
    pub total_bucket_count: usize,
    pub buckets: Vec<HistogramBucket>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CpuMetric {
    pub cumulative_cpu_time: Measurement,
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
    pub disk_io_metrics: Option<DiskIoMetric>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CrashDiagnosticObjectiveCExceptionReason {
    pub composed_message: String,
    pub format_string: String,
    pub arguments: Vec<String>,
    pub exception_type: String,
    pub class_name: String,
    pub exception_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CrashDiagnostic {
    pub application_version: String,
    pub call_stack_tree: Value,
    pub termination_reason: Option<String>,
    pub virtual_memory_region_info: Option<String>,
    pub exception_type: Option<i64>,
    pub exception_code: Option<i64>,
    pub signal: Option<i64>,
    pub exception_reason: Option<CrashDiagnosticObjectiveCExceptionReason>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HangDiagnostic {
    pub application_version: String,
    pub call_stack_tree: Value,
    pub hang_duration: Measurement,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CpuExceptionDiagnostic {
    pub application_version: String,
    pub call_stack_tree: Value,
    pub total_cpu_time: Measurement,
    pub total_sampled_time: Measurement,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiskWriteExceptionDiagnostic {
    pub application_version: String,
    pub call_stack_tree: Value,
    pub total_writes_caused: Measurement,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiagnosticPayload {
    pub time_stamp_begin: f64,
    pub time_stamp_end: f64,
    #[serde(default)]
    pub crash_diagnostics: Vec<CrashDiagnostic>,
    #[serde(default)]
    pub hang_diagnostics: Vec<HangDiagnostic>,
    #[serde(default)]
    pub cpu_exception_diagnostics: Vec<CpuExceptionDiagnostic>,
    #[serde(default)]
    pub disk_write_exception_diagnostics: Vec<DiskWriteExceptionDiagnostic>,
}
