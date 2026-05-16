mod common;

#[test]
fn hang_diagnostic_round_trips_to_json() -> Result<(), Box<dyn std::error::Error>> {
    let diagnostic = common::sample_hang_diagnostic();
    let json = diagnostic.json_representation()?;

    assert!(json.contains("hangDuration"));
    assert!(json.contains("callStackTree"));
    Ok(())
}
