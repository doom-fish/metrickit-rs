import Foundation
import MetricKit

func mxMetaData(_ metaData: MXMetaData) -> [String: Any] {
    [
        "regionFormat": metaData.regionFormat,
        "osVersion": metaData.osVersion,
        "deviceType": metaData.deviceType,
        "applicationBuildVersion": metaData.applicationBuildVersion,
        "platformArchitecture": metaData.platformArchitecture,
        "lowPowerModeEnabled": {
            if #available(macOS 14.0, *) {
                return metaData.lowPowerModeEnabled
            }
            return NSNull()
        }(),
        "isTestFlightApp": {
            if #available(macOS 14.0, *) {
                return metaData.isTestFlightApp
            }
            return NSNull()
        }(),
        "pid": {
            if #available(macOS 14.0, *) {
                return metaData.pid
            }
            return NSNull()
        }(),
        "bundleIdentifier": {
            if #available(macOS 26.0, *) {
                return metaData.bundleIdentifier
            }
            return NSNull()
        }(),
    ]
}
