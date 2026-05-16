# metrickit-rs

Safe Rust bindings for Apple's [MetricKit](https://developer.apple.com/documentation/metrickit) framework on macOS.

> **Status:** v0.2.0 covers the macOS MetricKit surface exposed by `MetricKit.framework`, including `MXMetricManager`, typed `MXMetricPayload` / `MXDiagnosticPayload` models, `MXCallStackTree`, MetricKit signpost emission, statistical helpers, metadata, signpost metrics / records, and crash / hang / CPU / disk-write diagnostics. iOS-only APIs are documented in [`COVERAGE.md`](COVERAGE.md).

## Quick start

```rust,no_run
use metrickit::{MetricManager, MetricSubscriberCallbacks};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manager = MetricManager::shared_manager();
    let log_handle = manager.make_log_handle("example.startup")?;
    let signpost_id = log_handle.make_signpost_id()?;
    log_handle.emit_event(signpost_id, "startup")?;

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

- `MetricManager::shared_manager`, cached payload access, RAII subscriber registration, `MetricKit` signpost log handles, and extended-launch helpers.
- Typed `MetricPayload`, `DiagnosticPayload`, `Diagnostic`, `MetaData`, and `CallStackTree` wrappers.
- Metric models for CPU, memory, GPU, animation, launch, responsiveness, runtime, location, network, disk I/O, display, cellular condition, application-exit, disk-space, and signpost metrics.
- Statistical helpers `Measurement`, `Average`, `Histogram`, and histogram buckets.
- Crash, hang, CPU-exception, and disk-write diagnostics with structured Objective-C exception reasons and signpost records.
- Twelve numbered examples and twelve integration test files covering every logical area listed in the v0.2.0 expansion.

## Delivery semantics

`MetricKit` typically delivers aggregated payloads roughly once per day when the app is running. The examples and tests use cached payload lookups plus deterministic sample models so they succeed on a headless development machine without waiting for `MetricKit` delivery.

## Examples

Run all examples with:

```bash
for ex in examples/*.rs; do cargo run --example "$(basename "$ex" .rs)"; done
```

Notable examples:

- `01_smoke` — `MXMetricManager` shared-manager, cached payload, subscription, and log-handle smoke test.
- `05_signpost_emit` — emits `MetricKit` signpost events and intervals from Rust.
- `06_metric_models` — serializes the extended metric surface, including display, cellular, exit, and disk-space metrics.

## Coverage audit

See [`COVERAGE.md`](COVERAGE.md) for the macOS `MetricKit` audit, including implemented APIs and the iOS-only APIs intentionally skipped by this macOS crate.

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
