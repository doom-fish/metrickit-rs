# metrickit-rs

Safe Rust bindings for Apple's [MetricKit](https://developer.apple.com/documentation/metrickit) framework on macOS.

> **Status:** v0.1.0 covers `MXMetricManager.shared`, subscriber registration, typed metric payloads, crash / hang / CPU / disk-write diagnostic payloads, and the `MXAverage` / `MXHistogram` statistical helpers used across MetricKit.

## Quick start

```rust,no_run
use metrickit::{MetricManager, MetricSubscriberCallbacks};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manager = MetricManager::shared();

    let subscription = manager.subscribe(
        MetricSubscriberCallbacks::new().on_metric_payloads(|payloads| {
            println!("received {} metric payload(s)", payloads.len());
        }),
    )?;

    println!("cached payloads: {}", manager.past_payloads()?.len());
    drop(subscription);
    Ok(())
}
```

## Highlights

- `MetricManager::shared`, `past_payloads`, `past_diagnostic_payloads`, and RAII `MetricSubscription`
- Delegate-to-Rust subscriber callbacks for `didReceive(_ payloads: [MXMetricPayload])` and diagnostic delivery
- Typed `MetricPayload`, `DiagnosticPayload`, and per-metric structs for CPU, memory, GPU, animation, launch, responsiveness, runtime, location, network, and disk I/O metrics
- Statistical helpers `Measurement`, `Average`, `Histogram`, and `HistogramBucket`
- Crash, hang, CPU-exception, and disk-write diagnostics with structured Objective-C exception details and raw call-stack-tree JSON

## Delivery semantics

`MetricKit` typically delivers aggregated payloads roughly once per day when the app is running. The smoke example only verifies that a subscriber can be added and removed successfully — it does not wait for payload delivery.

## Smoke example

Run the framework smoke test with:

```bash
cargo run --all-features --example 01_smoke
```

It registers a temporary subscriber with `MXMetricManager.shared`, prints the number of cached payloads already visible to the process, unsubscribes, and exits with `✅ metrickit subscribe + unsubscribe OK`.

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
