import Foundation
import MetricKit

func mxCPUMetric(_ metric: MXCPUMetric) -> [String: Any] {
    [
        "cumulativeCPUTime": mxMeasurement(metric.cumulativeCPUTime),
        "cumulativeCPUInstructions": {
            if #available(macOS 11.0, *) {
                return mxMeasurement(metric.cumulativeCPUInstructions)
            }
            return NSNull()
        }(),
    ]
}

func mxMemoryMetric(_ metric: MXMemoryMetric) -> [String: Any] {
    [
        "peakMemoryUsage": mxMeasurement(metric.peakMemoryUsage),
        "averageSuspendedMemory": mxAverage(metric.averageSuspendedMemory),
    ]
}

func mxGPUMetric(_ metric: MXGPUMetric) -> [String: Any] {
    ["cumulativeGPUTime": mxMeasurement(metric.cumulativeGPUTime)]
}

func mxAnimationMetric(_ metric: MXAnimationMetric) -> [String: Any] {
    [
        "scrollHitchTimeRatio": mxMeasurement(metric.scrollHitchTimeRatio),
        "hitchTimeRatio": {
            if #available(macOS 26.0, *) {
                return mxMeasurement(metric.hitchTimeRatio)
            }
            return NSNull()
        }(),
    ]
}

func mxApplicationLaunchMetric(_ metric: MXAppLaunchMetric) -> [String: Any] {
    [
        "histogrammedTimeToFirstDraw": mxHistogram(metric.histogrammedTimeToFirstDraw),
        "histogrammedApplicationResumeTime": mxHistogram(metric.histogrammedApplicationResumeTime),
        "histogrammedOptimizedTimeToFirstDraw": {
            if #available(macOS 12.2, *) {
                return mxHistogram(metric.histogrammedOptimizedTimeToFirstDraw)
            }
            return NSNull()
        }(),
        "histogrammedExtendedLaunch": {
            if #available(macOS 13.0, *) {
                return mxHistogram(metric.histogrammedExtendedLaunch)
            }
            return NSNull()
        }(),
    ]
}

func mxApplicationResponsivenessMetric(_ metric: MXAppResponsivenessMetric) -> [String: Any] {
    [
        "histogrammedApplicationHangTime": mxHistogram(metric.histogrammedApplicationHangTime),
    ]
}

func mxApplicationTimeMetric(_ metric: MXAppRunTimeMetric) -> [String: Any] {
    [
        "cumulativeForegroundTime": mxMeasurement(metric.cumulativeForegroundTime),
        "cumulativeBackgroundTime": mxMeasurement(metric.cumulativeBackgroundTime),
        "cumulativeBackgroundAudioTime": mxMeasurement(metric.cumulativeBackgroundAudioTime),
        "cumulativeBackgroundLocationTime": mxMeasurement(metric.cumulativeBackgroundLocationTime),
    ]
}

func mxLocationActivityMetric(_ metric: MXLocationActivityMetric) -> [String: Any] {
    [
        "cumulativeBestAccuracyTime": mxMeasurement(metric.cumulativeBestAccuracyTime),
        "cumulativeBestAccuracyForNavigationTime": mxMeasurement(metric.cumulativeBestAccuracyForNavigationTime),
        "cumulativeNearestTenMetersAccuracyTime": mxMeasurement(metric.cumulativeNearestTenMetersAccuracyTime),
        "cumulativeHundredMetersAccuracyTime": mxMeasurement(metric.cumulativeHundredMetersAccuracyTime),
        "cumulativeKilometerAccuracyTime": mxMeasurement(metric.cumulativeKilometerAccuracyTime),
        "cumulativeThreeKilometersAccuracyTime": mxMeasurement(metric.cumulativeThreeKilometersAccuracyTime),
    ]
}

func mxNetworkTransferMetric(_ metric: MXNetworkTransferMetric) -> [String: Any] {
    [
        "cumulativeWifiUpload": mxMeasurement(metric.cumulativeWifiUpload),
        "cumulativeWifiDownload": mxMeasurement(metric.cumulativeWifiDownload),
        "cumulativeCellularUpload": mxMeasurement(metric.cumulativeCellularUpload),
        "cumulativeCellularDownload": mxMeasurement(metric.cumulativeCellularDownload),
    ]
}

func mxDiskIOMetric(_ metric: MXDiskIOMetric) -> [String: Any] {
    ["cumulativeLogicalWrites": mxMeasurement(metric.cumulativeLogicalWrites)]
}

func mxMetricPayload(_ payload: MXMetricPayload) -> [String: Any] {
    [
        "latestApplicationVersion": payload.latestApplicationVersion,
        "includesMultipleApplicationVersions": payload.includesMultipleApplicationVersions,
        "timeStampBegin": payload.timeStampBegin.timeIntervalSince1970,
        "timeStampEnd": payload.timeStampEnd.timeIntervalSince1970,
        "cpuMetrics": payload.cpuMetrics.map(mxCPUMetric) ?? NSNull(),
        "memoryMetrics": payload.memoryMetrics.map(mxMemoryMetric) ?? NSNull(),
        "gpuMetrics": payload.gpuMetrics.map(mxGPUMetric) ?? NSNull(),
        "animationMetrics": payload.animationMetrics.map(mxAnimationMetric) ?? NSNull(),
        "applicationLaunchMetrics": payload.applicationLaunchMetrics.map(mxApplicationLaunchMetric) ?? NSNull(),
        "applicationResponsivenessMetrics": payload.applicationResponsivenessMetrics.map(mxApplicationResponsivenessMetric) ?? NSNull(),
        "applicationTimeMetrics": payload.applicationTimeMetrics.map(mxApplicationTimeMetric) ?? NSNull(),
        "locationActivityMetrics": payload.locationActivityMetrics.map(mxLocationActivityMetric) ?? NSNull(),
        "networkTransferMetrics": payload.networkTransferMetrics.map(mxNetworkTransferMetric) ?? NSNull(),
        "diskIOMetrics": payload.diskIOMetrics.map(mxDiskIOMetric) ?? NSNull(),
    ]
}

func mxDiagnosticBase(_ diagnostic: MXDiagnostic) -> [String: Any] {
    ["applicationVersion": diagnostic.applicationVersion]
}

@available(macOS 14.0, *)
func mxCrashDiagnosticObjectiveCExceptionReason(
    _ reason: MXCrashDiagnosticObjectiveCExceptionReason
) -> [String: Any] {
    [
        "composedMessage": reason.composedMessage,
        "formatString": reason.formatString,
        "arguments": reason.arguments,
        "exceptionType": reason.exceptionType,
        "className": reason.className,
        "exceptionName": reason.exceptionName,
    ]
}

func mxCrashDiagnostic(_ diagnostic: MXCrashDiagnostic) -> [String: Any] {
    var object = mxDiagnosticBase(diagnostic)
    object["callStackTree"] = mxCallStackTree(diagnostic.callStackTree)
    object["terminationReason"] = diagnostic.terminationReason ?? NSNull()
    object["virtualMemoryRegionInfo"] = diagnostic.virtualMemoryRegionInfo ?? NSNull()
    object["exceptionType"] = diagnostic.exceptionType ?? NSNull()
    object["exceptionCode"] = diagnostic.exceptionCode ?? NSNull()
    object["signal"] = diagnostic.signal ?? NSNull()
    if #available(macOS 14.0, *) {
        object["exceptionReason"] = diagnostic.exceptionReason.map(mxCrashDiagnosticObjectiveCExceptionReason) ?? NSNull()
    } else {
        object["exceptionReason"] = NSNull()
    }
    return object
}

func mxHangDiagnostic(_ diagnostic: MXHangDiagnostic) -> [String: Any] {
    var object = mxDiagnosticBase(diagnostic)
    object["callStackTree"] = mxCallStackTree(diagnostic.callStackTree)
    object["hangDuration"] = mxMeasurement(diagnostic.hangDuration)
    return object
}

func mxCPUExceptionDiagnostic(_ diagnostic: MXCPUExceptionDiagnostic) -> [String: Any] {
    var object = mxDiagnosticBase(diagnostic)
    object["callStackTree"] = mxCallStackTree(diagnostic.callStackTree)
    object["totalCPUTime"] = mxMeasurement(diagnostic.totalCPUTime)
    object["totalSampledTime"] = mxMeasurement(diagnostic.totalSampledTime)
    return object
}

func mxDiskWriteExceptionDiagnostic(_ diagnostic: MXDiskWriteExceptionDiagnostic) -> [String: Any] {
    var object = mxDiagnosticBase(diagnostic)
    object["callStackTree"] = mxCallStackTree(diagnostic.callStackTree)
    object["totalWritesCaused"] = mxMeasurement(diagnostic.totalWritesCaused)
    return object
}

func mxDiagnosticPayload(_ payload: MXDiagnosticPayload) -> [String: Any] {
    [
        "timeStampBegin": payload.timeStampBegin.timeIntervalSince1970,
        "timeStampEnd": payload.timeStampEnd.timeIntervalSince1970,
        "crashDiagnostics": payload.crashDiagnostics?.map(mxCrashDiagnostic) ?? [],
        "hangDiagnostics": payload.hangDiagnostics?.map(mxHangDiagnostic) ?? [],
        "cpuExceptionDiagnostics": payload.cpuExceptionDiagnostics?.map(mxCPUExceptionDiagnostic) ?? [],
        "diskWriteExceptionDiagnostics": payload.diskWriteExceptionDiagnostics?.map(mxDiskWriteExceptionDiagnostic) ?? [],
    ]
}
