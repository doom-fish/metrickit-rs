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
