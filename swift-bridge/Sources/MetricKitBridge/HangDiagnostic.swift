import Foundation
import MetricKit

func mxHangDiagnostic(_ diagnostic: MXHangDiagnostic) -> [String: Any] {
    var object = mxDiagnosticBase(diagnostic)
    object["callStackTree"] = mxCallStackTree(diagnostic.callStackTree)
    object["hangDuration"] = mxMeasurement(diagnostic.hangDuration)
    return object
}
