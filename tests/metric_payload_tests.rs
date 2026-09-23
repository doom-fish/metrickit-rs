mod common;

#[test]
fn metric_payload_round_trips_to_json_and_dictionary() -> Result<(), Box<dyn std::error::Error>> {
    let payload = common::sample_metric_payload();

    let json = payload.json_representation()?;
    let dictionary = payload.dictionary_representation()?;

    assert!(json.contains("latestApplicationVersion"));
    assert_eq!(
        dictionary["latestApplicationVersion"].as_str(),
        Some("1.2.3")
    );
    assert!(dictionary["displayMetrics"].is_object());
    assert!(dictionary["signpostMetrics"].is_array());
    Ok(())
}

#[test]
fn metric_payload_with_null_timestamps_still_decodes() -> Result<(), Box<dyn std::error::Error>> {
    let mut dictionary = common::sample_metric_payload().dictionary_representation()?;
    dictionary["timeStampBegin"] = serde_json::Value::Null;
    dictionary["timeStampEnd"] = serde_json::Value::Null;

    let payload: metrickit::MetricPayload = serde_json::from_value(dictionary)?;
    assert!(payload.time_stamp_begin.is_nan());
    assert!(payload.time_stamp_end.is_nan());
    assert_eq!(payload.latest_application_version, "1.2.3");
    Ok(())
}
