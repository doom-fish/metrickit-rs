import Foundation
import MetricKit

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
