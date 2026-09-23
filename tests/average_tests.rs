mod common;

#[test]
fn average_helpers_report_known_sample_count() {
    let average = common::sample_average();

    assert!(average.has_known_sample_count());
    assert_eq!(average.average_measurement.unit_symbol, "MB");
}

#[test]
fn non_finite_numbers_bridged_as_null_decode_as_nan() -> Result<(), Box<dyn std::error::Error>> {
    let average: metrickit::Average = serde_json::from_str(
        r#"{"averageMeasurement":{"value":null,"unitSymbol":"MB","unitType":"UnitInformationStorage"},"sampleCount":0,"standardDeviation":null}"#,
    )?;
    assert!(average.average_measurement.value.is_nan());
    assert!(average.standard_deviation.is_nan());

    let encoded = serde_json::to_value(&average)?;
    assert!(encoded["standardDeviation"].is_null());
    assert!(encoded["averageMeasurement"]["value"].is_null());
    Ok(())
}
