import Foundation
import MetricKit

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
