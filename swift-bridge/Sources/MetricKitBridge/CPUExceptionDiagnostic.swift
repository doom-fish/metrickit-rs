import Foundation
import MetricKit

func mxCPUExceptionDiagnostic(_ diagnostic: MXCPUExceptionDiagnostic) -> [String: Any] {
    var object = mxDiagnosticBase(diagnostic)
    object["callStackTree"] = mxCallStackTree(diagnostic.callStackTree)
    object["totalCPUTime"] = mxMeasurement(diagnostic.totalCPUTime)
    object["totalSampledTime"] = mxMeasurement(diagnostic.totalSampledTime)
    return object
}
