# Changelog

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
