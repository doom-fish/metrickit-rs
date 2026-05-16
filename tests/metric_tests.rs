mod common;

use metrickit::{AVERAGE_PIXEL_LUMINANCE_UNIT_SYMBOL, SIGNAL_BARS_UNIT_SYMBOL};

#[test]
fn metric_models_cover_extended_metric_surface() -> Result<(), Box<dyn std::error::Error>> {
    let display_metric = common::sample_display_metric();
    let cellular_metric = common::sample_cellular_condition_metric();
    let exit_metric = common::sample_application_exit_metric();
    let disk_space_metric = common::sample_disk_space_usage_metric();

    assert_eq!(AVERAGE_PIXEL_LUMINANCE_UNIT_SYMBOL, "apl");
    assert_eq!(SIGNAL_BARS_UNIT_SYMBOL, "bars");
    assert!(display_metric
        .json_representation()?
        .contains("averagePixelLuminance"));
    assert!(
        cellular_metric.dictionary_representation()?["histogrammedCellularConditionTime"]
            .is_object()
    );
    assert_eq!(
        exit_metric
            .background_exit_data
            .cumulative_normal_app_exit_count,
        4
    );
    assert_eq!(disk_space_metric.total_binary_file_count, 12);
    Ok(())
}
