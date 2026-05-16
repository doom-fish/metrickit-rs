import Foundation
import MetricKit

public typealias MXMetricEventCallback =
    @convention(c) (UnsafeMutableRawPointer?, UnsafePointer<CChar>?) -> Void

private final class MXRustSubscriber: NSObject, MXMetricManagerSubscriber {
    let callback: MXMetricEventCallback
    let userInfo: UnsafeMutableRawPointer?

    init(callback: @escaping MXMetricEventCallback, userInfo: UnsafeMutableRawPointer?) {
        self.callback = callback
        self.userInfo = userInfo
        super.init()
    }

    private func emit(_ object: [String: Any]) {
        let json = mxJSONString(object)
        json.withCString { callback(userInfo, $0) }
    }

    func didReceive(_ payloads: [MXMetricPayload]) {
        emit([
            "event": "didReceiveMetricPayloads",
            "metricPayloads": payloads.map(mxMetricPayload),
        ])
    }

    func didReceive(_ payloads: [MXDiagnosticPayload]) {
        emit([
            "event": "didReceiveDiagnosticPayloads",
            "diagnosticPayloads": payloads.map(mxDiagnosticPayload),
        ])
    }
}

@_cdecl("mx_metric_manager_add_subscriber")
public func mx_metric_manager_add_subscriber(
    _ callback: MXMetricEventCallback?,
    _ userInfo: UnsafeMutableRawPointer?,
    _ outHandle: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    outHandle.pointee = nil
    guard let callback else {
        mxWriteError(errorOut, "missing MetricKit subscriber callback")
        return MX_INVALID_ARGUMENT
    }

    let subscriber = MXRustSubscriber(callback: callback, userInfo: userInfo)
    MXMetricManager.shared.add(subscriber)
    outHandle.pointee = mxRetain(subscriber)
    _ = errorOut
    return MX_OK
}

@_cdecl("mx_metric_manager_remove_subscriber")
public func mx_metric_manager_remove_subscriber(_ handle: UnsafeMutableRawPointer?) {
    guard let handle else { return }
    let subscriber: MXRustSubscriber = mxBorrow(handle)
    MXMetricManager.shared.remove(subscriber)
}

@_cdecl("mx_metric_manager_past_payloads_json")
public func mx_metric_manager_past_payloads_json() -> UnsafeMutablePointer<CChar>? {
    mxCString(mxJSONString(MXMetricManager.shared.pastPayloads.map(mxMetricPayload)))
}

@_cdecl("mx_metric_manager_past_diagnostic_payloads_json")
public func mx_metric_manager_past_diagnostic_payloads_json() -> UnsafeMutablePointer<CChar>? {
    mxCString(mxJSONString(MXMetricManager.shared.pastDiagnosticPayloads.map(mxDiagnosticPayload)))
}
