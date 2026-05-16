#[path = "support/mod.rs"]
mod support;

use metrickit::MetricManager;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let payload = MetricManager::shared()
        .past_payloads()?
        .into_iter()
        .next()
        .unwrap_or_else(support::sample_metric_payload);

    println!("{}", payload.json_representation()?);
    Ok(())
}
