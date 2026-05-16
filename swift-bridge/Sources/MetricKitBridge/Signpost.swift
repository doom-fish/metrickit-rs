import Foundation
import MetricKit
import MetricKitSignpostC
import os.log

private final class MXRustLogHandleHolder {
    let logHandle: OSLog

    init(logHandle: OSLog) {
        self.logHandle = logHandle
    }
}

func mxSignpostIntervalData(_ data: MXSignpostIntervalData) -> [String: Any] {
    [
        "histogrammedSignpostDuration": mxHistogram(data.histogrammedSignpostDuration),
        "cumulativeCPUTime": data.cumulativeCPUTime.map(mxMeasurement) ?? NSNull(),
        "averageMemory": data.averageMemory.map(mxAverage) ?? NSNull(),
        "cumulativeLogicalWrites": data.cumulativeLogicalWrites.map(mxMeasurement) ?? NSNull(),
        "cumulativeHitchTimeRatio": {
            if #available(macOS 12.0, *) {
                return data.cumulativeHitchTimeRatio.map(mxMeasurement) ?? NSNull()
            }
            return NSNull()
        }(),
    ]
}

func mxSignpostMetric(_ metric: MXSignpostMetric) -> [String: Any] {
    [
        "signpostName": metric.signpostName,
        "signpostCategory": metric.signpostCategory,
        "signpostIntervalData": metric.signpostIntervalData.map(mxSignpostIntervalData) ?? NSNull(),
        "totalCount": metric.totalCount,
    ]
}

@available(macOS 14.0, *)
func mxSignpostRecord(_ record: MXSignpostRecord) -> [String: Any] {
    [
        "subsystem": record.subsystem,
        "category": record.category,
        "name": record.name,
        "beginTimeStamp": record.beginTimeStamp.timeIntervalSince1970,
        "endTimeStamp": record.endTimeStamp?.timeIntervalSince1970 ?? NSNull(),
        "duration": record.duration.map(mxMeasurement) ?? NSNull(),
        "isInterval": record.isInterval,
    ]
}

@_cdecl("mx_metric_manager_make_log_handle")
public func mx_metric_manager_make_log_handle(
    _ category: UnsafePointer<CChar>?,
    _ outHandle: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    outHandle.pointee = nil
    guard let category else {
        mxWriteError(errorOut, "missing MetricKit log category")
        return MX_INVALID_ARGUMENT
    }

    let categoryString = String(cString: category)
    guard !categoryString.isEmpty else {
        mxWriteError(errorOut, "MetricKit log category cannot be empty")
        return MX_INVALID_ARGUMENT
    }

    let holder = MXRustLogHandleHolder(logHandle: MXMetricManager.makeLogHandle(category: categoryString))
    outHandle.pointee = mxRetain(holder)
    return MX_OK
}

@_cdecl("mx_signpost_log_make_id")
public func mx_signpost_log_make_id(_ handle: UnsafeMutableRawPointer?) -> UInt64 {
    guard let handle else { return 0 }
    let holder: MXRustLogHandleHolder = mxBorrow(handle)
    return mx_metrickit_signpost_make_id(holder.logHandle)
}

private func mxSignpostCall(
    _ handle: UnsafeMutableRawPointer?,
    _ signpostID: UInt64,
    _ name: UnsafePointer<CChar>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    callback: (OSLog, UInt64, UnsafePointer<CChar>) -> Void
) -> Int32 {
    guard let handle else {
        mxWriteError(errorOut, "missing MetricKit log handle")
        return MX_INVALID_ARGUMENT
    }
    guard let name else {
        mxWriteError(errorOut, "missing MetricKit signpost name")
        return MX_INVALID_ARGUMENT
    }

    let nameString = String(cString: name)
    guard !nameString.isEmpty else {
        mxWriteError(errorOut, "MetricKit signpost name cannot be empty")
        return MX_INVALID_ARGUMENT
    }
    guard signpostID != 0 else {
        mxWriteError(errorOut, "MetricKit signpost ID cannot be zero")
        return MX_INVALID_ARGUMENT
    }

    let holder: MXRustLogHandleHolder = mxBorrow(handle)
    callback(holder.logHandle, signpostID, name)
    return MX_OK
}

@_cdecl("mx_signpost_event_emit")
public func mx_signpost_event_emit(
    _ handle: UnsafeMutableRawPointer?,
    _ signpostID: UInt64,
    _ name: UnsafePointer<CChar>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    mxSignpostCall(handle, signpostID, name, errorOut) {
        mx_metrickit_signpost_event_emit($0, $1, $2)
    }
}

@_cdecl("mx_signpost_interval_begin")
public func mx_signpost_interval_begin(
    _ handle: UnsafeMutableRawPointer?,
    _ signpostID: UInt64,
    _ name: UnsafePointer<CChar>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    mxSignpostCall(handle, signpostID, name, errorOut) {
        mx_metrickit_signpost_interval_begin($0, $1, $2)
    }
}

@_cdecl("mx_signpost_animation_interval_begin")
public func mx_signpost_animation_interval_begin(
    _ handle: UnsafeMutableRawPointer?,
    _ signpostID: UInt64,
    _ name: UnsafePointer<CChar>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    mxSignpostCall(handle, signpostID, name, errorOut) {
        mx_metrickit_signpost_animation_interval_begin($0, $1, $2)
    }
}

@_cdecl("mx_signpost_interval_end")
public func mx_signpost_interval_end(
    _ handle: UnsafeMutableRawPointer?,
    _ signpostID: UInt64,
    _ name: UnsafePointer<CChar>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    mxSignpostCall(handle, signpostID, name, errorOut) {
        mx_metrickit_signpost_interval_end($0, $1, $2)
    }
}
