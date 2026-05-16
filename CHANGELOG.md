# Changelog

## [0.1.0] - 2026-05-16

### Added

- `MetricManager` wrapper over `MXMetricManager.shared` with cached-payload access plus subscribe / unsubscribe lifecycle management.
- Delegate-to-Rust callback bridging for `MXMetricManagerSubscriber` metric and diagnostic delivery.
- Typed Rust models for `MXMetricPayload`, `MXDiagnosticPayload`, `MXCPUMetric`, `MXMemoryMetric`, `MXGPUMetric`, `MXAnimationMetric`, `MXAppLaunchMetric`, `MXAppResponsivenessMetric`, `MXAppRunTimeMetric`, `MXLocationActivityMetric`, `MXNetworkTransferMetric`, and `MXDiskIOMetric`.
- Statistical helper types for `MXAverage`, `MXHistogram`, and histogram buckets.
- Typed crash, hang, CPU-exception, and disk-write diagnostic snapshots, including Objective-C exception reason metadata and call-stack-tree JSON.
- Smoke example `examples/01_smoke.rs` that verifies MetricKit subscriber registration and removal without waiting for daily payload delivery.
