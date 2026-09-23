# MetricKit macOS coverage audit

Legend:

- ✅ implemented
- 🟡 partial
- ⏭️ skipped (platform-unavailable / deprecated-unavailable / internal-only)

This audit covers the macOS surface of `MetricKit.framework` as shipped in the macOS 26.2 SDK, re-checked on 2026-09-23 against the installed macOS 26.5 SDK headers (no new macOS-available symbols). The crate targets macOS-only usage; iOS-only APIs are explicitly marked skipped instead of being omitted silently.

The Rust models are the crate's own serde schema, built field by field in the Swift bridge. Their `json_representation` / `dictionary_representation` methods serialize that schema and are not interchangeable with Apple's `JSONRepresentation` / `dictionaryRepresentation` output; those rows are marked 🟡. Apple's own JSON is available for whole payloads through `MetricPayload::apple_json_representation` and `DiagnosticPayload::apple_json_representation`.

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
| `MXCallStackTree.h` | `MXCallStackTree.JSONRepresentation` | ✅ | `CallStackTree` holds Apple's JSON tree parsed into a `serde_json::Value`; `json_representation` re-serializes it (same content, key order and number formatting may differ). |
| `MXMetaData.h` | `MXMetaData` properties (`regionFormat`, `osVersion`, `deviceType`, `applicationBuildVersion`, `platformArchitecture`, `lowPowerModeEnabled`, `isTestFlightApp`, `pid`, `bundleIdentifier`) | ✅ | `MetaData` covers the full macOS property set. |
| `MXMetaData.h` | `JSONRepresentation` / `dictionaryRepresentation` | 🟡 | `MetaData::{json_representation,dictionary_representation}` serialize the crate's schema, not Apple's output. |
| `MXMetaData.h` | `DictionaryRepresentation` (deprecated Objective-C spelling) | ⏭️ | macOS-unavailable Objective-C-only spelling; Rust exposes `dictionary_representation`. |
| `MXUnit.h` | `MXUnitSignalBars.bars` | ✅ | `SIGNAL_BARS_UNIT_SYMBOL` constant plus typed `Measurement` values. |
| `MXUnit.h` | `MXUnitAveragePixelLuminance.apl` | ✅ | `AVERAGE_PIXEL_LUMINANCE_UNIT_SYMBOL` constant plus typed `Measurement` values. |

## Metric base + concrete metrics

| Header | API | Status | Notes |
| --- | --- | --- | --- |
| `MXMetric.h` | `MXMetric.JSONRepresentation` / `dictionaryRepresentation` | 🟡 | Every concrete metric wrapper exposes `json_representation` and `dictionary_representation` in the crate's schema; Apple's per-metric JSON is not exposed. |
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
| `MXMetricPayload.h` | `JSONRepresentation` | ✅ | `MetricPayload::apple_json_representation` holds Apple's output for payloads delivered by `MetricKit`; `MetricPayload::json_representation` is the crate's schema. |
| `MXMetricPayload.h` | `dictionaryRepresentation` | 🟡 | `MetricPayload::dictionary_representation` returns the crate's schema as a `serde_json::Value`. |
| `MXMetricPayload.h` | `DictionaryRepresentation` (deprecated Objective-C spelling) | ⏭️ | macOS-unavailable Objective-C-only spelling. |
| `MXDiagnosticPayload.h` | `MXDiagnosticPayload` timestamps + crash/hang/CPU/disk-write arrays | ✅ | `DiagnosticPayload`. |
| `MXDiagnosticPayload.h` | `JSONRepresentation` | ✅ | `DiagnosticPayload::apple_json_representation` holds Apple's output for payloads delivered by `MetricKit`; `DiagnosticPayload::json_representation` is the crate's schema. |
| `MXDiagnosticPayload.h` | `dictionaryRepresentation` | 🟡 | `DiagnosticPayload::dictionary_representation` returns the crate's schema as a `serde_json::Value`. |
| `MXDiagnosticPayload.h` | `appLaunchDiagnostics` | ⏭️ | iOS-only API; unavailable on macOS. |

## Diagnostic base + concrete diagnostics

| Header | API | Status | Notes |
| --- | --- | --- | --- |
| `MXDiagnostic.h` | `MXDiagnostic` (`metaData`, `applicationVersion`, `signpostData`) | ✅ | `Diagnostic` base wrapper plus typed signpost records. |
| `MXDiagnostic.h` | `JSONRepresentation` / `dictionaryRepresentation` | 🟡 | `Diagnostic::{json_representation,dictionary_representation}` and the concrete diagnostics serialize the crate's schema, not Apple's output. |
| `MXCrashDiagnosticObjectiveCExceptionReason.h` | all six properties | ✅ | `CrashDiagnosticObjectiveCExceptionReason`. |
| `MXCrashDiagnosticObjectiveCExceptionReason.h` | `JSONRepresentation` / `dictionaryRepresentation` | 🟡 | Crate schema, not Apple's output. |
| `MXCrashDiagnostic.h` | `MXCrashDiagnostic` (`callStackTree`, `terminationReason`, `virtualMemoryRegionInfo`, `exceptionType`, `exceptionCode`, `signal`, `exceptionReason`) | ✅ | `CrashDiagnostic`. |
| `MXHangDiagnostic.h` | `MXHangDiagnostic` (`callStackTree`, `hangDuration`) | ✅ | `HangDiagnostic`. |
| `MXCPUExceptionDiagnostic.h` | `MXCPUExceptionDiagnostic` (`callStackTree`, `totalCPUTime`, `totalSampledTime`) | ✅ | `CpuExceptionDiagnostic`. |
| `MXDiskWriteExceptionDiagnostic.h` | `MXDiskWriteExceptionDiagnostic` (`callStackTree`, `totalWritesCaused`) | ✅ | `DiskWriteExceptionDiagnostic`. |
| `MXAppLaunchDiagnostic.h` | `MXAppLaunchDiagnostic` | ⏭️ | iOS-only API; unavailable on macOS. |

## Signposts

| Header | API | Status | Notes |
| --- | --- | --- | --- |
| `MXSignpost.h` | `MXSignpostEventEmit` | ✅ | `MetricLogHandle::emit_event`; the name is a `&'static CStr` literal. |
| `MXSignpost.h` | `MXSignpostIntervalBegin` | ✅ | `MetricLogHandle::interval_begin`; the name is a `&'static CStr` literal. |
| `MXSignpost.h` | `MXSignpostAnimationIntervalBegin` | ✅ | `MetricLogHandle::animation_interval_begin`; the name is a `&'static CStr` literal. |
| `MXSignpost.h` | `MXSignpostIntervalEnd` | ✅ | `MetricLogHandle::interval_end`; the name is a `&'static CStr` literal. |
| `MXSignpostRecord.h` | `MXSignpostRecord` fields | ✅ | `SignpostRecord`. |
| `MXSignpostRecord.h` | `JSONRepresentation` / `dictionaryRepresentation` | 🟡 | Crate schema, not Apple's output. |
| `MXSignpost_Private.h` | `_MXSignpostMetricsSnapshot` and `_MXSignpost*` helper macros | ⏭️ | Not exposed as Rust API. The signpost shim expands the same `_MXSignpostMetricsSnapshot()` payload that the public `MXSignpost*` macros expand to, because those macros need the name as a compile-time literal; the shim checks at runtime that the name lies in the binary's `__TEXT` segment. |

## Errors

| Header | API | Status | Notes |
| --- | --- | --- | --- |
| `MXError.h` | `MXErrorDomain` / `MXErrorCode` | ⏭️ | iOS-only API; unavailable on macOS. |
