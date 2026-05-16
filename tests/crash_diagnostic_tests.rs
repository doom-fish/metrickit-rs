mod common;

#[test]
fn crash_diagnostic_includes_exception_reason_and_call_stack(
) -> Result<(), Box<dyn std::error::Error>> {
    let diagnostic = common::sample_crash_diagnostic();

    let dictionary = diagnostic.dictionary_representation()?;
    assert_eq!(
        dictionary["terminationReason"].as_str(),
        Some("namespace SIGNAL, code 11")
    );
    assert!(dictionary["callStackTree"].is_object());
    assert!(dictionary["exceptionReason"].is_object());
    Ok(())
}
