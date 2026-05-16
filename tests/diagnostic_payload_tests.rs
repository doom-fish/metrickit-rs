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
