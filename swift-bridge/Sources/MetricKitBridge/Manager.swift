import Foundation
import MetricKit

public typealias MXMetricEventCallback =
    @convention(c) (UnsafeMutableRawPointer?, UnsafePointer<CChar>?) -> Void
public typealias MXMetricEventRelease = @convention(c) (UnsafeMutableRawPointer?) -> Void

private final class MXRustSubscriber: NSObject, MXMetricManagerSubscriber {
    let callback: MXMetricEventCallback
    let userInfo: UnsafeMutableRawPointer?
    let releaseUserInfo: MXMetricEventRelease?

    init(
        callback: @escaping MXMetricEventCallback,
        userInfo: UnsafeMutableRawPointer?,
        releaseUserInfo: MXMetricEventRelease?
    ) {
        self.callback = callback
        self.userInfo = userInfo
        self.releaseUserInfo = releaseUserInfo
        super.init()
    }

    deinit {
        releaseUserInfo?(userInfo)
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
    _ releaseUserInfo: MXMetricEventRelease?,
    _ outHandle: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    outHandle.pointee = nil
    guard let callback else {
        mxWriteError(errorOut, "missing MetricKit subscriber callback")
        return MX_INVALID_ARGUMENT
    }

    let subscriber = MXRustSubscriber(
        callback: callback,
        userInfo: userInfo,
        releaseUserInfo: releaseUserInfo
    )
    MXMetricManager.shared.add(subscriber)
    outHandle.pointee = mxRetain(subscriber)
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

@_cdecl("mx_metric_manager_extend_launch_measurement")
public func mx_metric_manager_extend_launch_measurement(
    _ taskID: UnsafePointer<CChar>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let taskID else {
        mxWriteError(errorOut, "missing MetricKit launch task identifier")
        return MX_INVALID_ARGUMENT
    }

    let taskIDString = String(cString: taskID)
    guard !taskIDString.isEmpty else {
        mxWriteError(errorOut, "MetricKit launch task identifier cannot be empty")
        return MX_INVALID_ARGUMENT
    }

    guard Thread.isMainThread else {
        mxWriteError(errorOut, "MXMetricManager extended launch measurement must be called on the main thread")
        return MX_MAIN_THREAD_REQUIRED
    }

    guard #available(macOS 13.0, *) else {
        mxWriteError(errorOut, "MXMetricManager extended launch measurement requires macOS 13.0")
        return MX_FRAMEWORK_ERROR
    }

    do {
        try MXMetricManager.extendLaunchMeasurement(forTaskID: MXLaunchTaskID(taskIDString))
        return MX_OK
    } catch {
        mxWriteError(errorOut, (error as NSError).localizedDescription)
        return MX_FRAMEWORK_ERROR
    }
}

@_cdecl("mx_metric_manager_finish_extended_launch_measurement")
public func mx_metric_manager_finish_extended_launch_measurement(
    _ taskID: UnsafePointer<CChar>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let taskID else {
        mxWriteError(errorOut, "missing MetricKit launch task identifier")
        return MX_INVALID_ARGUMENT
    }

    let taskIDString = String(cString: taskID)
    guard !taskIDString.isEmpty else {
        mxWriteError(errorOut, "MetricKit launch task identifier cannot be empty")
        return MX_INVALID_ARGUMENT
    }

    guard Thread.isMainThread else {
        mxWriteError(errorOut, "MXMetricManager extended launch measurement must be called on the main thread")
        return MX_MAIN_THREAD_REQUIRED
    }

    guard #available(macOS 13.0, *) else {
        mxWriteError(errorOut, "MXMetricManager extended launch measurement requires macOS 13.0")
        return MX_FRAMEWORK_ERROR
    }

    do {
        try MXMetricManager.finishExtendedLaunchMeasurement(forTaskID: MXLaunchTaskID(taskIDString))
        return MX_OK
    } catch {
        mxWriteError(errorOut, (error as NSError).localizedDescription)
        return MX_FRAMEWORK_ERROR
    }
}
