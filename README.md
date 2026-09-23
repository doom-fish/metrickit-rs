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
    log_handle.emit_event(signpost_id, c"startup")?;

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

## Requirements

- macOS 12 or later (the Swift bridge's deployment target).
- Extended launch measurement needs macOS 13; signpost records and some metadata fields need macOS 14; disk-space metrics and `bundleIdentifier` need macOS 26. On older systems the launch-measurement calls return an error and those fields are `None` or empty.
- Xcode or the Command Line Tools, so `build.rs` can run `swift build`.

## Signpost names

`os_signpost` records a signpost name as an offset into the emitting binary, so names must be string literals. `MetricLogHandle::emit_event`, `interval_begin`, `animation_interval_begin`, and `interval_end` take `&'static CStr` (write `c"name"`), and the bridge rejects names that are not in the binary's read-only `__TEXT` segment, such as leaked heap strings, with `MetricKitError::InvalidArgument`.

## Threading

`MetricManager::extend_launch_measurement` and `finish_extended_launch_measurement` must be called on the main thread, as `MXMetricManager` requires. Called from any other thread they return `MetricKitError::MainThreadRequired` instead of blocking on the main queue, which would deadlock command-line and async-runtime hosts whose main thread does not service it.

Subscriber callbacks run on a `MetricKit` thread. After a `MetricSubscription` is dropped no further payloads reach the delegate; the delegate itself is dropped once `MetricKit` releases its subscriber, which can happen shortly afterwards on a `MetricKit` thread.

## JSON

The Rust models use the crate's own serde schema, filled in field by field by the Swift bridge. `json_representation()` and `dictionary_representation()` serialize that schema; the output is not Apple's `jsonRepresentation()` format and is not interchangeable with it. For metrics or crash-reporting backends that expect Apple's JSON, use `MetricPayload::apple_json_representation` and `DiagnosticPayload::apple_json_representation`, which hold Apple's output for payloads delivered by `MetricKit` (`None` for payloads built in Rust). `CallStackTree` holds Apple's call-stack JSON as a `serde_json::Value`.

Non-finite numbers are bridged as `null`; the non-optional `f64` fields decode `null` as NaN.

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
