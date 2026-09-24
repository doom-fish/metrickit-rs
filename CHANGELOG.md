# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.0] - Unreleased

### Fixed

- Subscriber state lives in a retained callback context that the Swift subscriber releases in `deinit`. Previously `MetricSubscription`'s drop freed the state right after `removeSubscriber:`, while `MetricKit` still held, and could still deliver to, the subscriber.
- Extended launch measurement no longer hops to the main thread with `DispatchQueue.main.sync`, which deadlocked hosts whose main thread does not service the main queue. Off the main thread the calls return `MetricKitError::MainThreadRequired`.
- NaN and infinite numbers no longer raise an uncatchable Objective-C exception while the bridge encodes JSON: they are bridged as `null`, and the non-optional `f64` fields decode `null` as NaN.
- `mx_signpost_bridge.m` no longer contains a raw NUL byte, which made git treat it as binary.
- `COVERAGE.md` and the coverage audits no longer describe the crate's serde JSON as Apple's `JSONRepresentation`; those rows are partial.
- `build.rs` no longer adds the toolchain's Swift 5.5 back-deployment directory (`usr/lib/swift-5.5/macosx`) to the rpath of the crate's tests and examples. The rpath pointed into Xcode, so it never made back-deployment work on other machines; `libswift_Concurrency` resolves through `/usr/lib/swift`.

### Changed

- **Breaking:** `MetricLogHandle::emit_event`, `interval_begin`, `animation_interval_begin`, and `interval_end` take `name: &'static CStr`. Names outside the binary's `__TEXT` segment, such as leaked heap strings, and empty names return `MetricKitError::InvalidArgument`.
- **Breaking:** `MetricPayload` and `DiagnosticPayload` have a new public field, `apple_json_representation`.
- Depends on `doom-fish-utils` 0.4.1 for the subscriber callback context.
- `rust-version` is now 1.82.

### Added

- `MetricPayload::apple_json_representation` and `DiagnosticPayload::apple_json_representation`, Apple's `jsonRepresentation()` output for payloads delivered by `MetricKit`.
- `MetricKitError::MainThreadRequired`.

### Removed

- The empty `MetricKitBridge.h` placeholder header and its module map.

## [0.2.2] - 2026-05-20

- Clippy hygiene sweep: cleared all `-D warnings` lints across the crate. No public API change.

## [0.2.1] - 2026-05-18

### Changed

- Added concise rustdoc coverage across the public MetricKit wrapper surface outside the FFI modules, including Apple counterpart references on the documented types and fields.

## [0.2.0] - 2026-05-16

### Added

- Split the Swift bridge across logical-area files for `MetricManager`, `MXMetricPayload`, `MXDiagnosticPayload`, `MXCallStackTree`, `MXSignpost`, `MXMetric`, `MXAverage`, `MXHistogram`, `MXCrashDiagnostic`, `MXHangDiagnostic`, `MXCPUExceptionDiagnostic`, and `MXDiskWriteExceptionDiagnostic`.
- Added typed Rust modules for MetricKit metadata, call-stack trees, signpost metrics / records, display metrics, cellular-condition metrics, application-exit metrics, disk-space-usage metrics, and base diagnostic context.
- Added `MetricManager::shared_manager`, `LaunchTaskId`, `MetricManager::make_log_handle`, extended-launch helpers, `MetricLogHandle`, and `SignpostId`.
- Added twelve numbered examples plus twelve integration test files that cover every logical area in the v0.2.0 expansion.
- Added `COVERAGE.md`, a macOS MetricKit API audit that marks implemented, skipped, and unavailable APIs.

### Changed

- Expanded `MetricPayload` and `DiagnosticPayload` to cover MetricKit metadata, signpost data, display / cellular / exit / disk-space metrics, and typed call-stack trees.
- Preserved Apple JSON key casing for round-trips involving CPU / GPU / disk-I/O MetricKit payload fields.
- Refreshed the README for the expanded v0.2.0 API surface.

## [0.1.0] - 2026-05-16

### Added

- `MetricManager` wrapper over `MXMetricManager.shared` with cached-payload access plus subscribe / unsubscribe lifecycle management.
- Delegate-to-Rust callback bridging for `MXMetricManagerSubscriber` metric and diagnostic delivery.
- Typed Rust models for `MXMetricPayload`, `MXDiagnosticPayload`, `MXCPUMetric`, `MXMemoryMetric`, `MXGPUMetric`, `MXAnimationMetric`, `MXAppLaunchMetric`, `MXAppResponsivenessMetric`, `MXAppRunTimeMetric`, `MXLocationActivityMetric`, `MXNetworkTransferMetric`, and `MXDiskIOMetric`.
- Statistical helper types for `MXAverage`, `MXHistogram`, and histogram buckets.
- Typed crash, hang, CPU-exception, and disk-write diagnostic snapshots, including Objective-C exception reason metadata and call-stack-tree JSON.
- Smoke example `examples/01_smoke.rs` that verifies MetricKit subscriber registration and removal without waiting for daily payload delivery.
