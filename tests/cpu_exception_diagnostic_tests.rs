mod common;

#[test]
fn cpu_exception_diagnostic_round_trips_to_dictionary() -> Result<(), Box<dyn std::error::Error>> {
    let diagnostic = common::sample_cpu_exception_diagnostic();
    let dictionary = diagnostic.dictionary_representation()?;

    assert!(dictionary["totalCPUTime"].is_object());
    assert!(dictionary["totalSampledTime"].is_object());
    Ok(())
}
