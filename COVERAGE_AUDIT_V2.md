# metrickit-rs coverage audit v2 (vs MacOSX26.2.sdk)

SDK_PUBLIC_SYMBOLS: 65
VERIFIED: 51
PARTIAL: 7
GAPS: 0
EXEMPT: 7
COVERAGE_PCT: 87.93

This audit verifies `metrickit-rs` against the macOS 26.2 `MetricKit.framework` SDK headers. Symbol enumeration includes all public classes, protocols, properties, and methods from the Obj-C framework excluding iOS-only and unavailable-on-macOS APIs. Every macOS-available entry is wrapped, but seven JSON entries are PARTIAL because the crate serializes its own schema instead of returning Apple's output.

_Corrected on 2026-09-23: rows that combined properties with `JSONRepresentation` / `dictionaryRepresentation` are split, and JSON rows where the crate serializes its own serde schema instead of Apple's output are counted as PARTIAL, not VERIFIED. Payload `JSONRepresentation` stays VERIFIED through `apple_json_representation`. COVERAGE_PCT is VERIFIED / (SDK_PUBLIC_SYMBOLS - EXEMPT) over the grouped header entries below. Re-checked against the installed MacOSX26.5.sdk headers: no new macOS-available symbols._

## 🟢 VERIFIED
| Symbol | Kind | Header | Wrapped by |
| --- | --- | --- | --- |
| `MXMetricManager.sharedManager` | class property | `MXMetricManager.h` | MetricManager::shared_manager and MetricManager::shared. |
| `pastPayloads` | property group | `MXMetricManager.h` | MetricManager::past_payloads. |
| `pastDiagnosticPayloads` | property group | `MXMetricManager.h` | MetricManager::past_diagnostic_payloads. |
| `makeLogHandleWithCategory:` | method group | `MXMetricManager.h` | MetricManager::make_log_handle returns MetricLogHandle. |
| `addSubscriber: / removeSubscriber:` | method group | `MXMetricManager.h` | Bridged through MetricManager::subscribe + MetricSubscription RAII drop/unsubscribe. |
| `extendLaunchMeasurementForTaskID:` | method group | `MXMetricManager.h` | MetricManager::extend_launch_measurement (macOS 13+). |
| `finishExtendedLaunchMeasurementForTaskID:` | method group | `MXMetricManager.h` | MetricManager::finish_extended_launch_measurement (macOS 13+). |
| `MXLaunchTaskID` | typedef | `MXMetricManager.h` | LaunchTaskId Rust alias and Swift bridge conversion. |
| `MXMetricManagerSubscriber.didReceiveMetricPayloads:` | protocol method | `MXMetricManager.h` | MetricSubscriberCallbacks / MetricSubscriberDelegate. |
| `MXMetricManagerSubscriber.didReceiveDiagnosticPayloads:` | protocol method | `MXMetricManager.h` | MetricSubscriberCallbacks / MetricSubscriberDelegate. |
| `MXAverage (averageMeasurement, sampleCount, standardDeviation)` | type group | `MXAverage.h` | Average + Measurement. |
| `MXHistogramBucket (bucketStart, bucketEnd, bucketCount)` | type group | `MXHistogram.h` | HistogramBucket. |
| `MXHistogram (totalBucketCount, bucketEnumerator)` | type group | `MXHistogram.h` | Histogram with owned bucket vector. |
| `MXCallStackTree.JSONRepresentation` | method group | `MXCallStackTree.h` | CallStackTree holds Apple's JSON tree as a serde_json::Value; json_representation re-serializes it. |
| `MXMetaData properties (regionFormat, osVersion, deviceType, applicationBuildVersion, platformArchitecture, lowPowerModeEnabled, isTestFlightApp, pid, bundleIdentifier)` | property group | `MXMetaData.h` | MetaData covers the full macOS property set. |
| `MXUnitSignalBars.bars` | class property | `MXUnit.h` | SIGNAL_BARS_UNIT_SYMBOL constant plus typed Measurement values. |
| `MXUnitAveragePixelLuminance.apl` | class property | `MXUnit.h` | AVERAGE_PIXEL_LUMINANCE_UNIT_SYMBOL constant plus typed Measurement values. |
| `MXCPUMetric (cumulativeCPUTime, cumulativeCPUInstructions)` | type group | `MXCPUMetric.h` | CpuMetric. |
| `MXMemoryMetric (peakMemoryUsage, averageSuspendedMemory)` | type group | `MXMemoryMetric.h` | MemoryMetric. |
| `MXGPUMetric (cumulativeGPUTime)` | type group | `MXGPUMetric.h` | GpuMetric. |
| `MXAnimationMetric (scrollHitchTimeRatio, hitchTimeRatio)` | type group | `MXAnimationMetric.h` | AnimationMetric. |
| `MXAppLaunchMetric (histogrammedTimeToFirstDraw, histogrammedApplicationResumeTime, histogrammedOptimizedTimeToFirstDraw, histogrammedExtendedLaunch)` | type group | `MXAppLaunchMetric.h` | ApplicationLaunchMetric. |
| `MXAppResponsivenessMetric (histogrammedApplicationHangTime)` | type group | `MXAppResponsivenessMetric.h` | ApplicationResponsivenessMetric. |
| `MXAppRunTimeMetric foreground/background/audio/location durations` | property group | `MXAppRunTimeMetric.h` | ApplicationTimeMetric. |
| `MXLocationActivityMetric six cumulative accuracy-duration fields` | property group | `MXLocationActivityMetric.h` | LocationActivityMetric. |
| `MXNetworkTransferMetric Wi-Fi/cellular upload/download fields` | property group | `MXNetworkTransferMetric.h` | NetworkTransferMetric. |
| `MXDiskIOMetric.cumulativeLogicalWrites` | property group | `MXDiskIOMetric.h` | DiskIoMetric. |
| `MXDisplayMetric.averagePixelLuminance` | property group | `MXDisplayMetric.h` | DisplayMetric. |
| `MXCellularConditionMetric.histogrammedCellularConditionTime` | type group | `MXCellularConditionMetric.h` | CellularConditionMetric. |
| `MXForegroundExitData counters` | property group | `MXAppExitMetric.h` | ForegroundExitData. |
| `MXBackgroundExitData counters` | property group | `MXAppExitMetric.h` | BackgroundExitData. |
| `MXAppExitMetric.foregroundExitData / backgroundExitData` | property group | `MXAppExitMetric.h` | ApplicationExitMetric. |
| `MXDiskSpaceUsageMetric file-count and disk-size properties` | property group | `MXDiskSpaceUsageMetric.h` | DiskSpaceUsageMetric. |
| `MXSignpostIntervalData (histogrammedSignpostDuration, cumulativeCPUTime, averageMemory, cumulativeLogicalWrites, cumulativeHitchTimeRatio)` | type group | `MXSignpostMetric.h` | SignpostIntervalData. |
| `MXSignpostMetric (signpostName, signpostCategory, signpostIntervalData, totalCount)` | type group | `MXSignpostMetric.h` | SignpostMetric. |
| `MXMetricPayload metadata (latestApplicationVersion, includesMultipleApplicationVersions, timestamps)` | property group | `MXMetricPayload.h` | MetricPayload. |
| `cpuMetrics, memoryMetrics, gpuMetrics, animationMetrics, applicationLaunchMetrics, applicationResponsivenessMetrics, applicationTimeMetrics, locationActivityMetrics, networkTransferMetrics, diskIOMetrics, displayMetrics, cellularConditionMetrics, applicationExitMetrics, diskSpaceUsageMetrics, signpostMetrics, metaData` | property group | `MXMetricPayload.h` | All macOS payload properties are modeled in MetricPayload. |
| `JSONRepresentation` | method | `MXMetricPayload.h` | MetricPayload::apple_json_representation (Apple's output); json_representation is the crate schema. |
| `MXDiagnosticPayload timestamps + crash/hang/CPU/disk-write arrays` | property group | `MXDiagnosticPayload.h` | DiagnosticPayload. |
| `JSONRepresentation` | method | `MXDiagnosticPayload.h` | DiagnosticPayload::apple_json_representation (Apple's output); json_representation is the crate schema. |
| `MXDiagnostic (metaData, applicationVersion, signpostData)` | type group | `MXDiagnostic.h` | Diagnostic base wrapper plus typed signpost records. |
| `all six properties` | type group | `MXCrashDiagnosticObjectiveCExceptionReason.h` | CrashDiagnosticObjectiveCExceptionReason. |
| `MXCrashDiagnostic (callStackTree, terminationReason, virtualMemoryRegionInfo, exceptionType, exceptionCode, signal, exceptionReason)` | type group | `MXCrashDiagnostic.h` | CrashDiagnostic. |
| `MXHangDiagnostic (callStackTree, hangDuration)` | type group | `MXHangDiagnostic.h` | HangDiagnostic. |
| `MXCPUExceptionDiagnostic (callStackTree, totalCPUTime, totalSampledTime)` | type group | `MXCPUExceptionDiagnostic.h` | CpuExceptionDiagnostic. |
| `MXDiskWriteExceptionDiagnostic (callStackTree, totalWritesCaused)` | type group | `MXDiskWriteExceptionDiagnostic.h` | DiskWriteExceptionDiagnostic. |
| `MXSignpostEventEmit` | function-like macro | `MXSignpost.h` | MetricLogHandle::emit_event (name: &'static CStr literal). |
| `MXSignpostIntervalBegin` | function-like macro | `MXSignpost.h` | MetricLogHandle::interval_begin (name: &'static CStr literal). |
| `MXSignpostAnimationIntervalBegin` | function-like macro | `MXSignpost.h` | MetricLogHandle::animation_interval_begin (name: &'static CStr literal). |
| `MXSignpostIntervalEnd` | function-like macro | `MXSignpost.h` | MetricLogHandle::interval_end (name: &'static CStr literal). |
| `MXSignpostRecord fields` | type group | `MXSignpostRecord.h` | SignpostRecord. |

## 🟡 PARTIAL
| Symbol | Kind | Header | Notes |
| --- | --- | --- | --- |
| `JSONRepresentation / dictionaryRepresentation` | method group | `MXMetaData.h` | MetaData::{json_representation,dictionary_representation} serialize the crate schema, not Apple's output. |
| `MXMetric.JSONRepresentation / dictionaryRepresentation` | method group | `MXMetric.h` | Concrete metric wrappers serialize the crate schema; Apple's per-metric JSON is not exposed. |
| `dictionaryRepresentation` | method | `MXMetricPayload.h` | MetricPayload::dictionary_representation returns the crate schema. |
| `dictionaryRepresentation` | method | `MXDiagnosticPayload.h` | DiagnosticPayload::dictionary_representation returns the crate schema. |
| `JSONRepresentation / dictionaryRepresentation` | method group | `MXDiagnostic.h` | Diagnostic and concrete diagnostics serialize the crate schema. |
| `JSONRepresentation / dictionaryRepresentation` | method group | `MXCrashDiagnosticObjectiveCExceptionReason.h` | Crate schema, not Apple's output. |
| `JSONRepresentation / dictionaryRepresentation` | method group | `MXSignpostRecord.h` | Crate schema, not Apple's output. |

## 🔴 GAPS
No current gaps identified in the macOS-available `MetricKit` surface.
| Symbol | Kind | Header | Notes |
| --- | --- | --- | --- |

## ⏭️ EXEMPT
| Symbol | Kind | Header | Reason | SDK attribute |
| --- | --- | --- | --- | --- |
| `DictionaryRepresentation (deprecated Objective-C spelling)` | deprecated method | `MXMetaData.h` | macOS-unavailable Objective-C-only spelling; Rust exposes dictionary_representation. | `API_DEPRECATED_WITH_REPLACEMENT("Use dictionaryRepresentation", ios(13.0, API_TO_BE_DEPRECATED)) API_UNAVAILABLE(macos, tvos, watchos) NS_REFINED_FOR_SWIFT` |
| `MXMetric.DictionaryRepresentation (deprecated Objective-C spelling)` | deprecated method | `MXMetric.h` | Deprecated and unavailable on macOS. | `API_DEPRECATED_WITH_REPLACEMENT("Use dictionaryRepresentation", ios(13.0, API_TO_BE_DEPRECATED)) API_UNAVAILABLE(macos, tvos, watchos) NS_REFINED_FOR_SWIFT` |
| `DictionaryRepresentation (deprecated Objective-C spelling)` | deprecated method | `MXMetricPayload.h` | macOS-unavailable Objective-C-only spelling. | `API_DEPRECATED_WITH_REPLACEMENT("Use dictionaryRepresentation", ios(13.0, API_TO_BE_DEPRECATED)) API_UNAVAILABLE(macos, tvos, watchos) NS_REFINED_FOR_SWIFT` |
| `appLaunchDiagnostics` | unavailable property | `MXDiagnosticPayload.h` | iOS-only API; unavailable on macOS. | `API_AVAILABLE(ios(16.0)) API_UNAVAILABLE(macos, tvos, watchos)` |
| `MXAppLaunchDiagnostic` | unavailable class | `MXAppLaunchDiagnostic.h` | iOS-only API; unavailable on macOS. | `API_AVAILABLE(ios(16.0)) API_UNAVAILABLE(macos, tvos, watchos)` |
| `_MXSignpostMetricsSnapshot and _MXSignpost* helper macros` | private helper group | `MXSignpost_Private.h` | Not exposed as Rust API. The header marks these as "DO NOT CALL DIRECTLY", but the signpost shim expands the same _MXSignpostMetricsSnapshot() payload the public MXSignpost* macros expand to, because those macros need a compile-time name literal. | `#pragma mark - Implementation details. DO NOT CALL DIRECTLY` |
| `MXErrorDomain / MXErrorCode` | constant/enum group | `MXError.h` | iOS-only API; unavailable on macOS. | `API_AVAILABLE(ios(16.0)) API_UNAVAILABLE(macos, tvos, watchos)` |
