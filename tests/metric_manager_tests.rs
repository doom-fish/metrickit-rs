mod common;

use std::sync::{Arc, Mutex};

use metrickit::{MetricManager, MetricSubscriberCallbacks};

#[test]
fn metric_manager_smoke_paths_succeed() -> Result<(), Box<dyn std::error::Error>> {
    let manager = MetricManager::shared();
    let _ = manager.past_payloads()?;
    let _ = manager.past_diagnostic_payloads()?;

    let metric_invocations = Arc::new(Mutex::new(0usize));
    let diagnostic_invocations = Arc::new(Mutex::new(0usize));
    let metric_counter = Arc::clone(&metric_invocations);
    let diagnostic_counter = Arc::clone(&diagnostic_invocations);

    let subscription = manager.subscribe(
        MetricSubscriberCallbacks::new()
            .on_metric_payloads(move |_| {
                *metric_counter.lock().expect("metric counter poisoned") += 1;
            })
            .on_diagnostic_payloads(move |_| {
                *diagnostic_counter
                    .lock()
                    .expect("diagnostic counter poisoned") += 1;
            }),
    )?;

    assert!(subscription.is_active());
    drop(subscription);

    assert_eq!(
        *metric_invocations.lock().expect("metric counter poisoned"),
        0
    );
    assert_eq!(
        *diagnostic_invocations
            .lock()
            .expect("diagnostic counter poisoned"),
        0
    );
    Ok(())
}
