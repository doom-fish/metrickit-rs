mod common;

#[test]
fn diagnostic_payload_round_trips_to_json_and_dictionary() -> Result<(), Box<dyn std::error::Error>>
{
    let payload = common::sample_diagnostic_payload();

    let json = payload.json_representation()?;
    let dictionary = payload.dictionary_representation()?;

    assert!(json.contains("crashDiagnostics"));
    assert_eq!(
        dictionary["hangDiagnostics"].as_array().map(Vec::len),
        Some(1)
    );
    assert_eq!(
        dictionary["cpuExceptionDiagnostics"]
            .as_array()
            .map(Vec::len),
        Some(1)
    );
    Ok(())
}

#[test]
fn diagnostic_payload_with_null_timestamps_still_decodes() -> Result<(), Box<dyn std::error::Error>>
{
    let mut dictionary = common::sample_diagnostic_payload().dictionary_representation()?;
    dictionary["timeStampBegin"] = serde_json::Value::Null;

    let payload: metrickit::DiagnosticPayload = serde_json::from_value(dictionary)?;
    assert!(payload.time_stamp_begin.is_nan());
    assert!(payload.time_stamp_end.is_finite());
    Ok(())
}

#[test]
fn diagnostic_payload_carries_apple_json_when_bridged() -> Result<(), Box<dyn std::error::Error>> {
    let mut bridged = common::sample_diagnostic_payload().dictionary_representation()?;
    bridged["appleJSONRepresentation"] = serde_json::Value::String("{}".to_owned());

    let payload: metrickit::DiagnosticPayload = serde_json::from_value(bridged)?;
    assert_eq!(payload.apple_json_representation.as_deref(), Some("{}"));
    assert!(!payload
        .json_representation()?
        .contains("appleJSONRepresentation"));
    Ok(())
}
