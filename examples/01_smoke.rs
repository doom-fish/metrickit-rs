use metrickit::{MetricManager, MetricSubscriberCallbacks};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manager = MetricManager::shared_manager();
    let metric_payloads = manager.past_payloads()?;
    let diagnostic_payloads = manager.past_diagnostic_payloads()?;
    let subscription = manager.subscribe(MetricSubscriberCallbacks::new())?;
    let log_handle = manager.make_log_handle("examples.01_smoke")?;

    println!(
        "cached payloads: {} metric / {} diagnostic (log category: {})",
        metric_payloads.len(),
        diagnostic_payloads.len(),
        log_handle.category()
    );
    assert!(subscription.is_active());
    drop(log_handle);
    drop(subscription);

    println!("✅ metrickit subscribe + unsubscribe OK");
    Ok(())
}
