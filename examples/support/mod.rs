#![allow(dead_code)]
#![allow(clippy::missing_const_for_fn)]

use metrickit::{
    AnimationMetric, ApplicationExitMetric, ApplicationLaunchMetric,
    ApplicationResponsivenessMetric, ApplicationTimeMetric, Average, BackgroundExitData,
    CallStackTree, CellularConditionMetric, CpuExceptionDiagnostic, CpuMetric, CrashDiagnostic,
    CrashDiagnosticObjectiveCExceptionReason, Diagnostic, DiagnosticPayload, DiskIoMetric,
    DiskSpaceUsageMetric, DiskWriteExceptionDiagnostic, DisplayMetric, ForegroundExitData,
    GpuMetric, HangDiagnostic, Histogram, HistogramBucket, LocationActivityMetric, Measurement,
    MemoryMetric, MetaData, MetricPayload, NetworkTransferMetric, SignpostIntervalData,
    SignpostMetric, SignpostRecord, AVERAGE_PIXEL_LUMINANCE_UNIT_SYMBOL, SIGNAL_BARS_UNIT_SYMBOL,
};
use serde_json::json;

pub fn sample_duration_measurement() -> Measurement {
    Measurement::new(1.25, "s", "UnitDuration")
}

pub fn sample_ratio_measurement() -> Measurement {
    Measurement::new(0.2, "", "Unit")
}

pub fn sample_size_measurement() -> Measurement {
    Measurement::new(256.0, "MB", "UnitInformationStorage")
}

pub fn sample_instruction_measurement() -> Measurement {
    Measurement::new(4096.0, "instructions", "Unit")
}

pub fn sample_average() -> Average {
    Average {
        average_measurement: sample_size_measurement(),
        sample_count: 4,
        standard_deviation: 0.5,
    }
}

pub fn sample_histogram() -> Histogram {
    Histogram {
        total_bucket_count: 2,
        buckets: vec![
            HistogramBucket {
                bucket_start: Measurement::new(0.0, "s", "UnitDuration"),
                bucket_end: Measurement::new(1.0, "s", "UnitDuration"),
                bucket_count: 2,
            },
            HistogramBucket {
                bucket_start: Measurement::new(1.0, "s", "UnitDuration"),
                bucket_end: Measurement::new(2.0, "s", "UnitDuration"),
                bucket_count: 1,
            },
        ],
    }
}

pub fn sample_meta_data() -> MetaData {
    MetaData {
        region_format: "en_US".into(),
        os_version: "26.0".into(),
        device_type: "MacBookPro".into(),
        application_build_version: "42".into(),
        platform_architecture: Some("arm64".into()),
        low_power_mode_enabled: Some(false),
        is_test_flight_app: Some(false),
        pid: Some(1234),
        bundle_identifier: Some("fish.doom.metrickit".into()),
    }
}

pub fn sample_call_stack_tree() -> CallStackTree {
    CallStackTree::new(json!({
        "callStacks": [
            {
                "threadAttributed": true,
                "callStackRootFrames": [
                    {
                        "binaryName": "MetrickitExample",
                        "binaryUUID": "00000000-0000-0000-0000-000000000000",
                        "offsetIntoBinaryTextSegment": 16,
                        "sampleCount": 1
                    }
                ]
            }
        ]
    }))
}

pub fn sample_signpost_record() -> SignpostRecord {
    SignpostRecord {
        subsystem: "fish.doom.metrickit".into(),
        category: "sample".into(),
        name: "payload-processing".into(),
        begin_time_stamp: 100.0,
        end_time_stamp: Some(101.5),
        duration: Some(Measurement::new(1500.0, "ms", "UnitDuration")),
        is_interval: true,
    }
}

pub fn sample_diagnostic() -> Diagnostic {
    Diagnostic {
        meta_data: sample_meta_data(),
        application_version: "1.2.3".into(),
        signpost_data: vec![sample_signpost_record()],
    }
}

pub fn sample_cpu_metric() -> CpuMetric {
    CpuMetric {
        cumulative_cpu_time: sample_duration_measurement(),
        cumulative_cpu_instructions: Some(sample_instruction_measurement()),
    }
}

pub fn sample_memory_metric() -> MemoryMetric {
    MemoryMetric {
        peak_memory_usage: sample_size_measurement(),
        average_suspended_memory: sample_average(),
    }
}

pub fn sample_gpu_metric() -> GpuMetric {
    GpuMetric {
        cumulative_gpu_time: sample_duration_measurement(),
    }
}

pub fn sample_animation_metric() -> AnimationMetric {
    AnimationMetric {
        scroll_hitch_time_ratio: sample_ratio_measurement(),
        hitch_time_ratio: Some(sample_ratio_measurement()),
    }
}

pub fn sample_application_launch_metric() -> ApplicationLaunchMetric {
    ApplicationLaunchMetric {
        histogrammed_time_to_first_draw: sample_histogram(),
        histogrammed_application_resume_time: sample_histogram(),
        histogrammed_optimized_time_to_first_draw: Some(sample_histogram()),
        histogrammed_extended_launch: Some(sample_histogram()),
    }
}

pub fn sample_application_responsiveness_metric() -> ApplicationResponsivenessMetric {
    ApplicationResponsivenessMetric {
        histogrammed_application_hang_time: sample_histogram(),
    }
}

pub fn sample_application_time_metric() -> ApplicationTimeMetric {
    ApplicationTimeMetric {
        cumulative_foreground_time: sample_duration_measurement(),
        cumulative_background_time: sample_duration_measurement(),
        cumulative_background_audio_time: sample_duration_measurement(),
        cumulative_background_location_time: sample_duration_measurement(),
    }
}

pub fn sample_location_activity_metric() -> LocationActivityMetric {
    LocationActivityMetric {
        cumulative_best_accuracy_time: sample_duration_measurement(),
        cumulative_best_accuracy_for_navigation_time: sample_duration_measurement(),
        cumulative_nearest_ten_meters_accuracy_time: sample_duration_measurement(),
        cumulative_hundred_meters_accuracy_time: sample_duration_measurement(),
        cumulative_kilometer_accuracy_time: sample_duration_measurement(),
        cumulative_three_kilometers_accuracy_time: sample_duration_measurement(),
    }
}

pub fn sample_network_transfer_metric() -> NetworkTransferMetric {
    NetworkTransferMetric {
        cumulative_wifi_upload: sample_size_measurement(),
        cumulative_wifi_download: sample_size_measurement(),
        cumulative_cellular_upload: sample_size_measurement(),
        cumulative_cellular_download: sample_size_measurement(),
    }
}

pub fn sample_disk_io_metric() -> DiskIoMetric {
    DiskIoMetric {
        cumulative_logical_writes: sample_size_measurement(),
    }
}

pub fn sample_display_metric() -> DisplayMetric {
    DisplayMetric {
        average_pixel_luminance: Some(Average {
            average_measurement: Measurement::new(
                42.0,
                AVERAGE_PIXEL_LUMINANCE_UNIT_SYMBOL,
                "MXUnitAveragePixelLuminance",
            ),
            sample_count: 8,
            standard_deviation: 1.5,
        }),
    }
}

pub fn sample_cellular_condition_metric() -> CellularConditionMetric {
    CellularConditionMetric {
        histogrammed_cellular_condition_time: Histogram {
            total_bucket_count: 1,
            buckets: vec![HistogramBucket {
                bucket_start: Measurement::new(0.0, SIGNAL_BARS_UNIT_SYMBOL, "MXUnitSignalBars"),
                bucket_end: Measurement::new(4.0, SIGNAL_BARS_UNIT_SYMBOL, "MXUnitSignalBars"),
                bucket_count: 6,
            }],
        },
    }
}

pub fn sample_foreground_exit_data() -> ForegroundExitData {
    ForegroundExitData {
        cumulative_normal_app_exit_count: 2,
        cumulative_memory_resource_limit_exit_count: 1,
        cumulative_bad_access_exit_count: 0,
        cumulative_abnormal_exit_count: 1,
        cumulative_illegal_instruction_exit_count: 0,
        cumulative_app_watchdog_exit_count: 0,
    }
}

pub fn sample_background_exit_data() -> BackgroundExitData {
    BackgroundExitData {
        cumulative_normal_app_exit_count: 4,
        cumulative_memory_resource_limit_exit_count: 1,
        cumulative_cpu_resource_limit_exit_count: 0,
        cumulative_memory_pressure_exit_count: 0,
        cumulative_bad_access_exit_count: 0,
        cumulative_abnormal_exit_count: 0,
        cumulative_illegal_instruction_exit_count: 0,
        cumulative_app_watchdog_exit_count: 0,
        cumulative_suspended_with_locked_file_exit_count: 0,
        cumulative_background_task_assertion_timeout_exit_count: 0,
    }
}

pub fn sample_application_exit_metric() -> ApplicationExitMetric {
    ApplicationExitMetric {
        foreground_exit_data: sample_foreground_exit_data(),
        background_exit_data: sample_background_exit_data(),
    }
}

pub fn sample_disk_space_usage_metric() -> DiskSpaceUsageMetric {
    DiskSpaceUsageMetric {
        total_binary_file_size: sample_size_measurement(),
        total_binary_file_count: 12,
        total_data_file_size: sample_size_measurement(),
        total_data_file_count: 24,
        total_cache_folder_size: sample_size_measurement(),
        total_clone_size: sample_size_measurement(),
        total_disk_space_used_size: sample_size_measurement(),
        total_disk_space_capacity: Measurement::new(1024.0, "GB", "UnitInformationStorage"),
    }
}

pub fn sample_signpost_interval_data() -> SignpostIntervalData {
    SignpostIntervalData {
        histogrammed_signpost_duration: sample_histogram(),
        cumulative_cpu_time: Some(sample_duration_measurement()),
        average_memory: Some(sample_average()),
        cumulative_logical_writes: Some(sample_size_measurement()),
        cumulative_hitch_time_ratio: Some(sample_ratio_measurement()),
    }
}

pub fn sample_signpost_metric() -> SignpostMetric {
    SignpostMetric {
        signpost_name: "payload-processing".into(),
        signpost_category: "sample".into(),
        signpost_interval_data: Some(sample_signpost_interval_data()),
        total_count: 3,
    }
}

pub fn sample_metric_payload() -> MetricPayload {
    MetricPayload {
        latest_application_version: "1.2.3".into(),
        includes_multiple_application_versions: false,
        time_stamp_begin: 10.0,
        time_stamp_end: 20.0,
        cpu_metrics: Some(sample_cpu_metric()),
        memory_metrics: Some(sample_memory_metric()),
        gpu_metrics: Some(sample_gpu_metric()),
        animation_metrics: Some(sample_animation_metric()),
        application_launch_metrics: Some(sample_application_launch_metric()),
        application_responsiveness_metrics: Some(sample_application_responsiveness_metric()),
        application_time_metrics: Some(sample_application_time_metric()),
        location_activity_metrics: Some(sample_location_activity_metric()),
        network_transfer_metrics: Some(sample_network_transfer_metric()),
        disk_io_metrics: Some(sample_disk_io_metric()),
        display_metrics: Some(sample_display_metric()),
        cellular_condition_metrics: Some(sample_cellular_condition_metric()),
        application_exit_metrics: Some(sample_application_exit_metric()),
        disk_space_usage_metrics: Some(sample_disk_space_usage_metric()),
        signpost_metrics: vec![sample_signpost_metric()],
        meta_data: Some(sample_meta_data()),
    }
}

pub fn sample_exception_reason() -> CrashDiagnosticObjectiveCExceptionReason {
    CrashDiagnosticObjectiveCExceptionReason {
        composed_message: "Index 3 beyond bounds".into(),
        format_string: "Index %@ beyond bounds".into(),
        arguments: vec!["3".into()],
        exception_type: "NSRangeException".into(),
        class_name: "NSException".into(),
        exception_name: "NSRangeException".into(),
    }
}

pub fn sample_crash_diagnostic() -> CrashDiagnostic {
    CrashDiagnostic {
        diagnostic: sample_diagnostic(),
        call_stack_tree: sample_call_stack_tree(),
        termination_reason: Some("namespace SIGNAL, code 11".into()),
        virtual_memory_region_info: Some("MALLOC_TINY".into()),
        exception_type: Some(11),
        exception_code: Some(0xdead_beef),
        signal: Some(11),
        exception_reason: Some(sample_exception_reason()),
    }
}

pub fn sample_hang_diagnostic() -> HangDiagnostic {
    HangDiagnostic {
        diagnostic: sample_diagnostic(),
        call_stack_tree: sample_call_stack_tree(),
        hang_duration: sample_duration_measurement(),
    }
}

pub fn sample_cpu_exception_diagnostic() -> CpuExceptionDiagnostic {
    CpuExceptionDiagnostic {
        diagnostic: sample_diagnostic(),
        call_stack_tree: sample_call_stack_tree(),
        total_cpu_time: sample_duration_measurement(),
        total_sampled_time: sample_duration_measurement(),
    }
}

pub fn sample_disk_write_exception_diagnostic() -> DiskWriteExceptionDiagnostic {
    DiskWriteExceptionDiagnostic {
        diagnostic: sample_diagnostic(),
        call_stack_tree: sample_call_stack_tree(),
        total_writes_caused: sample_size_measurement(),
    }
}

pub fn sample_diagnostic_payload() -> DiagnosticPayload {
    DiagnosticPayload {
        time_stamp_begin: 10.0,
        time_stamp_end: 20.0,
        crash_diagnostics: vec![sample_crash_diagnostic()],
        hang_diagnostics: vec![sample_hang_diagnostic()],
        cpu_exception_diagnostics: vec![sample_cpu_exception_diagnostic()],
        disk_write_exception_diagnostics: vec![sample_disk_write_exception_diagnostic()],
    }
}
