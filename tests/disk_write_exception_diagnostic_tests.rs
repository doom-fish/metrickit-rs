mod common;

#[test]
fn disk_write_exception_diagnostic_round_trips_to_dictionary(
) -> Result<(), Box<dyn std::error::Error>> {
    let diagnostic = common::sample_disk_write_exception_diagnostic();
    let dictionary = diagnostic.dictionary_representation()?;

    assert!(dictionary["totalWritesCaused"].is_object());
    assert!(dictionary["callStackTree"].is_object());
    Ok(())
}
