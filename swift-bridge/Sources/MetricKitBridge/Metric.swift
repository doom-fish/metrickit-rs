import Foundation
import MetricKit

func mxCPUMetric(_ metric: MXCPUMetric) -> [String: Any] {
    [
        "cumulativeCPUTime": mxMeasurement(metric.cumulativeCPUTime),
        "cumulativeCPUInstructions": mxMeasurement(metric.cumulativeCPUInstructions),
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

func mxDisplayMetric(_ metric: MXDisplayMetric) -> [String: Any] {
    [
        "averagePixelLuminance": metric.averagePixelLuminance.map(mxAverage) ?? NSNull(),
    ]
}

func mxCellularConditionMetric(_ metric: MXCellularConditionMetric) -> [String: Any] {
    [
        "histogrammedCellularConditionTime": mxHistogram(metric.histogrammedCellularConditionTime),
    ]
}

func mxForegroundExitData(_ data: MXForegroundExitData) -> [String: Any] {
    [
        "cumulativeNormalAppExitCount": data.cumulativeNormalAppExitCount,
        "cumulativeMemoryResourceLimitExitCount": data.cumulativeMemoryResourceLimitExitCount,
        "cumulativeBadAccessExitCount": data.cumulativeBadAccessExitCount,
        "cumulativeAbnormalExitCount": data.cumulativeAbnormalExitCount,
        "cumulativeIllegalInstructionExitCount": data.cumulativeIllegalInstructionExitCount,
        "cumulativeAppWatchdogExitCount": data.cumulativeAppWatchdogExitCount,
    ]
}

func mxBackgroundExitData(_ data: MXBackgroundExitData) -> [String: Any] {
    [
        "cumulativeNormalAppExitCount": data.cumulativeNormalAppExitCount,
        "cumulativeMemoryResourceLimitExitCount": data.cumulativeMemoryResourceLimitExitCount,
        "cumulativeCPUResourceLimitExitCount": data.cumulativeCPUResourceLimitExitCount,
        "cumulativeMemoryPressureExitCount": data.cumulativeMemoryPressureExitCount,
        "cumulativeBadAccessExitCount": data.cumulativeBadAccessExitCount,
        "cumulativeAbnormalExitCount": data.cumulativeAbnormalExitCount,
        "cumulativeIllegalInstructionExitCount": data.cumulativeIllegalInstructionExitCount,
        "cumulativeAppWatchdogExitCount": data.cumulativeAppWatchdogExitCount,
        "cumulativeSuspendedWithLockedFileExitCount": data.cumulativeSuspendedWithLockedFileExitCount,
        "cumulativeBackgroundTaskAssertionTimeoutExitCount": data.cumulativeBackgroundTaskAssertionTimeoutExitCount,
    ]
}

func mxApplicationExitMetric(_ metric: MXAppExitMetric) -> [String: Any] {
    [
        "foregroundExitData": mxForegroundExitData(metric.foregroundExitData),
        "backgroundExitData": mxBackgroundExitData(metric.backgroundExitData),
    ]
}

@available(macOS 26.0, *)
func mxDiskSpaceUsageMetric(_ metric: MXDiskSpaceUsageMetric) -> [String: Any] {
    [
        "totalBinaryFileSize": mxMeasurement(metric.totalBinaryFileSize),
        "totalBinaryFileCount": metric.totalBinaryFileCount,
        "totalDataFileSize": mxMeasurement(metric.totalDataFileSize),
        "totalDataFileCount": metric.totalDataFileCount,
        "totalCacheFolderSize": mxMeasurement(metric.totalCacheFolderSize),
        "totalCloneSize": mxMeasurement(metric.totalCloneSize),
        "totalDiskSpaceUsedSize": mxMeasurement(metric.totalDiskSpaceUsedSize),
        "totalDiskSpaceCapacity": mxMeasurement(metric.totalDiskSpaceCapacity),
    ]
}
