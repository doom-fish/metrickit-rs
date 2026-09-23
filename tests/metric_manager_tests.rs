mod common;

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use metrickit::{MetricManager, MetricSubscriberCallbacks, MetricSubscriberDelegate};

struct DropCounter(Arc<AtomicUsize>);

impl MetricSubscriberDelegate for DropCounter {}

impl Drop for DropCounter {
    fn drop(&mut self) {
        self.0.fetch_add(1, Ordering::SeqCst);
    }
}

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

#[test]
fn unsubscribing_releases_the_delegate_once_metrickit_lets_go(
) -> Result<(), Box<dyn std::error::Error>> {
    let drops = Arc::new(AtomicUsize::new(0));
    let subscription = MetricManager::shared().subscribe(DropCounter(Arc::clone(&drops)))?;
    assert_eq!(drops.load(Ordering::SeqCst), 0);

    drop(subscription);
    let deadline = Instant::now() + Duration::from_secs(10);
    while drops.load(Ordering::SeqCst) == 0 && Instant::now() < deadline {
        std::thread::sleep(Duration::from_millis(10));
    }
    assert_eq!(drops.load(Ordering::SeqCst), 1);
    Ok(())
}
