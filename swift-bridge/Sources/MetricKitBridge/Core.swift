import Foundation
import MetricKit

public let MX_OK: Int32 = 0
public let MX_INVALID_ARGUMENT: Int32 = -1
public let MX_FRAMEWORK_ERROR: Int32 = -2

@inline(__always)
public func mxRetain(_ object: some AnyObject) -> UnsafeMutableRawPointer {
    Unmanaged.passRetained(object).toOpaque()
}

@inline(__always)
public func mxBorrow<T: AnyObject>(_ ptr: UnsafeMutableRawPointer, as _: T.Type = T.self) -> T {
    Unmanaged<T>.fromOpaque(ptr).takeUnretainedValue()
}

@_cdecl("mx_object_release")
public func mx_object_release(_ ptr: UnsafeMutableRawPointer?) {
    guard let ptr else { return }
    Unmanaged<AnyObject>.fromOpaque(ptr).release()
}

@inline(__always)
public func mxCString(_ string: String) -> UnsafeMutablePointer<CChar>? {
    string.withCString { strdup($0) }
}

@inline(__always)
public func mxWriteError(
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ message: String
) {
    errorOut?.pointee = mxCString(message)
}

func mxJSONSafe(_ value: Any) -> Any {
    switch value {
    case let dict as [String: Any]:
        return dict.mapValues(mxJSONSafe)
    case let array as [Any]:
        return array.map(mxJSONSafe)
    case let date as Date:
        return date.timeIntervalSince1970
    case let data as Data:
        return data.base64EncodedString()
    case let number as NSNumber:
        return number
    case let string as String:
        return string
    case _ as NSNull:
        return NSNull()
    default:
        return String(describing: value)
    }
}

func mxJSONString(_ value: Any) -> String {
    do {
        let safe = mxJSONSafe(value)
        let data = try JSONSerialization.data(withJSONObject: safe, options: [.sortedKeys])
        return String(data: data, encoding: .utf8) ?? "[]"
    } catch {
        return "[]"
    }
}

func mxJSONObject(from data: Data) -> Any {
    (try? JSONSerialization.jsonObject(with: data, options: [])) ?? NSNull()
}

func mxMeasurement<UnitType: Unit>(_ measurement: Measurement<UnitType>) -> [String: Any] {
    [
        "value": measurement.value,
        "unitSymbol": measurement.unit.symbol,
        "unitType": String(describing: type(of: measurement.unit)),
    ]
}

func mxAverage<UnitType: Unit>(_ average: MXAverage<UnitType>) -> [String: Any] {
    [
        "averageMeasurement": mxMeasurement(average.averageMeasurement),
        "sampleCount": average.sampleCount,
        "standardDeviation": average.standardDeviation,
    ]
}

func mxHistogram<UnitType: Unit>(_ histogram: MXHistogram<UnitType>) -> [String: Any] {
    var buckets: [[String: Any]] = []
    let enumerator = histogram.bucketEnumerator
    while let bucket = enumerator.nextObject() as? MXHistogramBucket<UnitType> {
        buckets.append([
            "bucketStart": mxMeasurement(bucket.bucketStart),
            "bucketEnd": mxMeasurement(bucket.bucketEnd),
            "bucketCount": bucket.bucketCount,
        ])
    }

    return [
        "totalBucketCount": histogram.totalBucketCount,
        "buckets": buckets,
    ]
}

func mxCallStackTree(_ tree: MXCallStackTree) -> Any {
    mxJSONObject(from: tree.jsonRepresentation())
}
