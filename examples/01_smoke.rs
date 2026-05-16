use metrickit::{MetricManager, MetricSubscriberCallbacks};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manager = MetricManager::shared();

    let metric_payloads = manager.past_payloads()?;
    let diagnostic_payloads = manager.past_diagnostic_payloads()?;
    println!(
        "cached payloads: {} metric / {} diagnostic",
        metric_payloads.len(),
        diagnostic_payloads.len()
    );

    let subscription = manager.subscribe(MetricSubscriberCallbacks::new())?;
    assert!(subscription.is_active());
    drop(subscription);

    println!("✅ metrickit subscribe + unsubscribe OK");
    Ok(())
}
