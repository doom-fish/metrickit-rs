import Foundation
import MetricKit

func mxDiskWriteExceptionDiagnostic(_ diagnostic: MXDiskWriteExceptionDiagnostic) -> [String: Any] {
    var object = mxDiagnosticBase(diagnostic)
    object["callStackTree"] = mxCallStackTree(diagnostic.callStackTree)
    object["totalWritesCaused"] = mxMeasurement(diagnostic.totalWritesCaused)
    return object
}
