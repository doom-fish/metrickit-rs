mod common;

use metrickit::MetricManager;

#[test]
fn signpost_log_handle_can_emit_metrickit_signposts() -> Result<(), Box<dyn std::error::Error>> {
    let manager = MetricManager::shared();
    let log_handle = manager.make_log_handle("tests.signpost")?;

    let event_id = log_handle.make_signpost_id()?;
    log_handle.emit_event(event_id, "unit-event")?;

    let interval_id = log_handle.make_signpost_id()?;
    log_handle.interval_begin(interval_id, "unit-interval")?;
    log_handle.interval_end(interval_id, "unit-interval")?;

    assert_eq!(log_handle.category(), "tests.signpost");
    Ok(())
}

#[test]
fn signpost_models_round_trip_to_json() -> Result<(), Box<dyn std::error::Error>> {
    let metric = common::sample_signpost_metric();
    let record = common::sample_signpost_record();

    assert!(metric.json_representation()?.contains("signpostName"));
    assert!(record.dictionary_representation()?["isInterval"].is_boolean());
    Ok(())
}
