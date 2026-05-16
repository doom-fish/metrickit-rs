import Foundation
import MetricKit

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
        "displayMetrics": payload.displayMetrics.map(mxDisplayMetric) ?? NSNull(),
        "cellularConditionMetrics": payload.cellularConditionMetrics.map(mxCellularConditionMetric) ?? NSNull(),
        "applicationExitMetrics": payload.applicationExitMetrics.map(mxApplicationExitMetric) ?? NSNull(),
        "diskSpaceUsageMetrics": {
            if #available(macOS 26.0, *) {
                return payload.diskSpaceUsageMetrics.map(mxDiskSpaceUsageMetric) ?? NSNull()
            }
            return NSNull()
        }(),
        "signpostMetrics": payload.signpostMetrics?.map(mxSignpostMetric) ?? [],
        "metaData": payload.metaData.map(mxMetaData) ?? NSNull(),
    ]
}
