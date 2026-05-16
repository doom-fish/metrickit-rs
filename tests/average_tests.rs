mod common;

#[test]
fn average_helpers_report_known_sample_count() {
    let average = common::sample_average();

    assert!(average.has_known_sample_count());
    assert_eq!(average.average_measurement.unit_symbol, "MB");
}
