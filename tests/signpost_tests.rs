mod common;

use std::ffi::{CStr, CString};

use metrickit::{MetricKitError, MetricManager};

#[test]
fn signpost_log_handle_can_emit_metrickit_signposts() -> Result<(), Box<dyn std::error::Error>> {
    let manager = MetricManager::shared();
    let log_handle = manager.make_log_handle("tests.signpost")?;

    let event_id = log_handle.make_signpost_id()?;
    log_handle.emit_event(event_id, c"unit-event")?;

    let interval_id = log_handle.make_signpost_id()?;
    log_handle.interval_begin(interval_id, c"unit-interval")?;
    log_handle.interval_end(interval_id, c"unit-interval")?;

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

#[test]
fn signpost_names_must_be_literals_in_this_binary() -> Result<(), Box<dyn std::error::Error>> {
    let manager = MetricManager::shared();
    let log_handle = manager.make_log_handle("tests.signpost.names")?;
    let signpost_id = log_handle.make_signpost_id()?;

    let leaked: &'static CStr = Box::leak(CString::new("heap-name")?.into_boxed_c_str());
    assert!(matches!(
        log_handle.emit_event(signpost_id, leaked),
        Err(MetricKitError::InvalidArgument(_))
    ));
    assert!(matches!(
        log_handle.interval_begin(signpost_id, leaked),
        Err(MetricKitError::InvalidArgument(_))
    ));
    assert!(matches!(
        log_handle.emit_event(signpost_id, c""),
        Err(MetricKitError::InvalidArgument(_))
    ));

    log_handle.animation_interval_begin(signpost_id, c"unit-animation")?;
    log_handle.interval_end(signpost_id, c"unit-animation")?;
    Ok(())
}
