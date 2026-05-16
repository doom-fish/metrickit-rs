import Foundation
import MetricKit

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
