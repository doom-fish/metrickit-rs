import Foundation
import MetricKit

func mxDiagnosticBase(_ diagnostic: MXDiagnostic) -> [String: Any] {
    [
        "metaData": mxMetaData(diagnostic.metaData),
        "applicationVersion": diagnostic.applicationVersion,
        "signpostData": {
            if #available(macOS 14.0, *) {
                return diagnostic.signpostData?.map(mxSignpostRecord) ?? []
            }
            return []
        }(),
    ]
}

func mxDiagnosticPayload(_ payload: MXDiagnosticPayload) -> [String: Any] {
    [
        "timeStampBegin": payload.timeStampBegin.timeIntervalSince1970,
        "timeStampEnd": payload.timeStampEnd.timeIntervalSince1970,
        "crashDiagnostics": payload.crashDiagnostics?.map(mxCrashDiagnostic) ?? [],
        "hangDiagnostics": payload.hangDiagnostics?.map(mxHangDiagnostic) ?? [],
        "cpuExceptionDiagnostics": payload.cpuExceptionDiagnostics?.map(mxCPUExceptionDiagnostic) ?? [],
        "diskWriteExceptionDiagnostics": payload.diskWriteExceptionDiagnostics?.map(mxDiskWriteExceptionDiagnostic) ?? [],
        "appLaunchDiagnostics": [],
    ]
}
