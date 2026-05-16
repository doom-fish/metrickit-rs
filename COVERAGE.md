# MetricKit macOS coverage audit

Legend:

- ✅ implemented
- 🟡 partial
- ⏭️ skipped (platform-unavailable / deprecated-unavailable / internal-only)

This audit covers the macOS surface of `MetricKit.framework` as shipped in the macOS 26.2 SDK. The crate targets macOS-only usage; iOS-only APIs are explicitly marked skipped instead of being omitted silently.

## MetricManager

| Header | API | Status | Notes |
| --- | --- | --- | --- |
| `MXMetricManager.h` | `MXMetricManager.sharedManager` | ✅ | `MetricManager::shared_manager` and `MetricManager::shared`. |
| `MXMetricManager.h` | `pastPayloads` | ✅ | `MetricManager::past_payloads`. |
| `MXMetricManager.h` | `pastDiagnosticPayloads` | ✅ | `MetricManager::past_diagnostic_payloads`. |
| `MXMetricManager.h` | `makeLogHandleWithCategory:` | ✅ | `MetricManager::make_log_handle` returns `MetricLogHandle`. |
| `MXMetricManager.h` | `addSubscriber:` / `removeSubscriber:` | ✅ | Bridged through `MetricManager::subscribe` + `MetricSubscription` RAII drop/unsubscribe. |
| `MXMetricManager.h` | `extendLaunchMeasurementForTaskID:` | ✅ | `MetricManager::extend_launch_measurement` (macOS 13+). |
| `MXMetricManager.h` | `finishExtendedLaunchMeasurementForTaskID:` | ✅ | `MetricManager::finish_extended_launch_measurement` (macOS 13+). |
| `MXMetricManager.h` | `MXLaunchTaskID` | ✅ | `LaunchTaskId` Rust alias and Swift bridge conversion. |
| `MXMetricManager.h` | `MXMetricManagerSubscriber.didReceiveMetricPayloads:` | ✅ | `MetricSubscriberCallbacks` / `MetricSubscriberDelegate`. |
| `MXMetricManager.h` | `MXMetricManagerSubscriber.didReceiveDiagnosticPayloads:` | ✅ | `MetricSubscriberCallbacks` / `MetricSubscriberDelegate`. |

## Shared utility types

| Header | API | Status | Notes |
| --- | --- | --- | --- |
| `MXAverage.h` | `MXAverage` (`averageMeasurement`, `sampleCount`, `standardDeviation`) | ✅ | `Average` + `Measurement`. |
| `MXHistogram.h` | `MXHistogramBucket` (`bucketStart`, `bucketEnd`, `bucketCount`) | ✅ | `HistogramBucket`. |
| `MXHistogram.h` | `MXHistogram` (`totalBucketCount`, `bucketEnumerator`) | ✅ | `Histogram` with owned bucket vector. |
| `MXCallStackTree.h` | `MXCallStackTree.JSONRepresentation` | ✅ | `CallStackTree::json_representation`. |
| `MXMetaData.h` | `MXMetaData` properties (`regionFormat`, `osVersion`, `deviceType`, `applicationBuildVersion`, `platformArchitecture`, `lowPowerModeEnabled`, `isTestFlightApp`, `pid`, `bundleIdentifier`) | ✅ | `MetaData` covers the full macOS property set. |
| `MXMetaData.h` | `JSONRepresentation` / `dictionaryRepresentation` | ✅ | `MetaData::{json_representation,dictionary_representation}`. |
| `MXMetaData.h` | `DictionaryRepresentation` (deprecated Objective-C spelling) | ⏭️ | macOS-unavailable Objective-C-only spelling; Rust exposes `dictionary_representation`. |
| `MXUnit.h` | `MXUnitSignalBars.bars` | ✅ | `SIGNAL_BARS_UNIT_SYMBOL` constant plus typed `Measurement` values. |
| `MXUnit.h` | `MXUnitAveragePixelLuminance.apl` | ✅ | `AVERAGE_PIXEL_LUMINANCE_UNIT_SYMBOL` constant plus typed `Measurement` values. |

## Metric base + concrete metrics

| Header | API | Status | Notes |
| --- | --- | --- | --- |
| `MXMetric.h` | `MXMetric.JSONRepresentation` / `dictionaryRepresentation` | ✅ | Every concrete metric wrapper exposes `json_representation` and `dictionary_representation`. |
| `MXMetric.h` | `MXMetric.DictionaryRepresentation` (deprecated Objective-C spelling) | ⏭️ | Deprecated and unavailable on macOS. |
| `MXCPUMetric.h` | `MXCPUMetric` (`cumulativeCPUTime`, `cumulativeCPUInstructions`) | ✅ | `CpuMetric`. |
| `MXMemoryMetric.h` | `MXMemoryMetric` (`peakMemoryUsage`, `averageSuspendedMemory`) | ✅ | `MemoryMetric`. |
| `MXGPUMetric.h` | `MXGPUMetric` (`cumulativeGPUTime`) | ✅ | `GpuMetric`. |
| `MXAnimationMetric.h` | `MXAnimationMetric` (`scrollHitchTimeRatio`, `hitchTimeRatio`) | ✅ | `AnimationMetric`. |
| `MXAppLaunchMetric.h` | `MXAppLaunchMetric` (`histogrammedTimeToFirstDraw`, `histogrammedApplicationResumeTime`, `histogrammedOptimizedTimeToFirstDraw`, `histogrammedExtendedLaunch`) | ✅ | `ApplicationLaunchMetric`. |
| `MXAppResponsivenessMetric.h` | `MXAppResponsivenessMetric` (`histogrammedApplicationHangTime`) | ✅ | `ApplicationResponsivenessMetric`. |
| `MXAppRunTimeMetric.h` | `MXAppRunTimeMetric` foreground/background/audio/location durations | ✅ | `ApplicationTimeMetric`. |
| `MXLocationActivityMetric.h` | `MXLocationActivityMetric` six cumulative accuracy-duration fields | ✅ | `LocationActivityMetric`. |
| `MXNetworkTransferMetric.h` | `MXNetworkTransferMetric` Wi-Fi/cellular upload/download fields | ✅ | `NetworkTransferMetric`. |
| `MXDiskIOMetric.h` | `MXDiskIOMetric.cumulativeLogicalWrites` | ✅ | `DiskIoMetric`. |
| `MXDisplayMetric.h` | `MXDisplayMetric.averagePixelLuminance` | ✅ | `DisplayMetric`. |
| `MXCellularConditionMetric.h` | `MXCellularConditionMetric.histogrammedCellularConditionTime` | ✅ | `CellularConditionMetric`. |
| `MXAppExitMetric.h` | `MXForegroundExitData` counters | ✅ | `ForegroundExitData`. |
| `MXAppExitMetric.h` | `MXBackgroundExitData` counters | ✅ | `BackgroundExitData`. |
| `MXAppExitMetric.h` | `MXAppExitMetric.foregroundExitData` / `backgroundExitData` | ✅ | `ApplicationExitMetric`. |
| `MXDiskSpaceUsageMetric.h` | `MXDiskSpaceUsageMetric` file-count and disk-size properties | ✅ | `DiskSpaceUsageMetric`. |
| `MXSignpostMetric.h` | `MXSignpostIntervalData` (`histogrammedSignpostDuration`, `cumulativeCPUTime`, `averageMemory`, `cumulativeLogicalWrites`, `cumulativeHitchTimeRatio`) | ✅ | `SignpostIntervalData`. |
| `MXSignpostMetric.h` | `MXSignpostMetric` (`signpostName`, `signpostCategory`, `signpostIntervalData`, `totalCount`) | ✅ | `SignpostMetric`. |

## Payload wrappers

| Header | API | Status | Notes |
| --- | --- | --- | --- |
| `MXMetricPayload.h` | `MXMetricPayload` metadata (`latestApplicationVersion`, `includesMultipleApplicationVersions`, timestamps) | ✅ | `MetricPayload`. |
| `MXMetricPayload.h` | `cpuMetrics`, `memoryMetrics`, `gpuMetrics`, `animationMetrics`, `applicationLaunchMetrics`, `applicationResponsivenessMetrics`, `applicationTimeMetrics`, `locationActivityMetrics`, `networkTransferMetrics`, `diskIOMetrics`, `displayMetrics`, `cellularConditionMetrics`, `applicationExitMetrics`, `diskSpaceUsageMetrics`, `signpostMetrics`, `metaData` | ✅ | All macOS payload properties are modeled in `MetricPayload`. |
| `MXMetricPayload.h` | `JSONRepresentation` / `dictionaryRepresentation` | ✅ | `MetricPayload::{json_representation,dictionary_representation}`. |
| `MXMetricPayload.h` | `DictionaryRepresentation` (deprecated Objective-C spelling) | ⏭️ | macOS-unavailable Objective-C-only spelling. |
| `MXDiagnosticPayload.h` | `MXDiagnosticPayload` timestamps + crash/hang/CPU/disk-write arrays | ✅ | `DiagnosticPayload`. |
| `MXDiagnosticPayload.h` | `JSONRepresentation` / `dictionaryRepresentation` | ✅ | `DiagnosticPayload::{json_representation,dictionary_representation}`. |
| `MXDiagnosticPayload.h` | `appLaunchDiagnostics` | ⏭️ | iOS-only API; unavailable on macOS. |

## Diagnostic base + concrete diagnostics

| Header | API | Status | Notes |
| --- | --- | --- | --- |
| `MXDiagnostic.h` | `MXDiagnostic` (`metaData`, `applicationVersion`, `signpostData`) | ✅ | `Diagnostic` base wrapper plus typed signpost records. |
| `MXDiagnostic.h` | `JSONRepresentation` / `dictionaryRepresentation` | ✅ | `Diagnostic::{json_representation,dictionary_representation}`. |
| `MXCrashDiagnosticObjectiveCExceptionReason.h` | all six properties + `JSONRepresentation` / `dictionaryRepresentation` | ✅ | `CrashDiagnosticObjectiveCExceptionReason`. |
| `MXCrashDiagnostic.h` | `MXCrashDiagnostic` (`callStackTree`, `terminationReason`, `virtualMemoryRegionInfo`, `exceptionType`, `exceptionCode`, `signal`, `exceptionReason`) | ✅ | `CrashDiagnostic`. |
| `MXHangDiagnostic.h` | `MXHangDiagnostic` (`callStackTree`, `hangDuration`) | ✅ | `HangDiagnostic`. |
| `MXCPUExceptionDiagnostic.h` | `MXCPUExceptionDiagnostic` (`callStackTree`, `totalCPUTime`, `totalSampledTime`) | ✅ | `CpuExceptionDiagnostic`. |
| `MXDiskWriteExceptionDiagnostic.h` | `MXDiskWriteExceptionDiagnostic` (`callStackTree`, `totalWritesCaused`) | ✅ | `DiskWriteExceptionDiagnostic`. |
| `MXAppLaunchDiagnostic.h` | `MXAppLaunchDiagnostic` | ⏭️ | iOS-only API; unavailable on macOS. |

## Signposts

| Header | API | Status | Notes |
| --- | --- | --- | --- |
| `MXSignpost.h` | `MXSignpostEventEmit` | ✅ | `MetricLogHandle::emit_event`. |
| `MXSignpost.h` | `MXSignpostIntervalBegin` | ✅ | `MetricLogHandle::interval_begin`. |
| `MXSignpost.h` | `MXSignpostAnimationIntervalBegin` | ✅ | `MetricLogHandle::animation_interval_begin`. |
| `MXSignpost.h` | `MXSignpostIntervalEnd` | ✅ | `MetricLogHandle::interval_end`. |
| `MXSignpostRecord.h` | `MXSignpostRecord` fields + `JSONRepresentation` / `dictionaryRepresentation` | ✅ | `SignpostRecord`. |
| `MXSignpost_Private.h` | `_MXSignpostMetricsSnapshot` and `_MXSignpost*` helper macros | ⏭️ | Header explicitly marks these implementation details as “DO NOT CALL DIRECTLY”; the public `MXSignpost.h` API is implemented instead. |

## Errors

| Header | API | Status | Notes |
| --- | --- | --- | --- |
| `MXError.h` | `MXErrorDomain` / `MXErrorCode` | ⏭️ | iOS-only API; unavailable on macOS. |
