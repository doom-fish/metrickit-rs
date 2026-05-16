// swift-tools-version:5.9
import PackageDescription

let package = Package(
    name: "MetricKitBridge",
    platforms: [
        .macOS(.v12)
    ],
    products: [
        .library(
            name: "MetricKitBridge",
            type: .static,
            targets: ["MetricKitBridge"]
        )
    ],
    targets: [
        .target(
            name: "MetricKitSignpostC",
            path: "Sources/MetricKitSignpostC",
            publicHeadersPath: "include"
        ),
        .target(
            name: "MetricKitBridge",
            dependencies: ["MetricKitSignpostC"],
            path: "Sources/MetricKitBridge",
            publicHeadersPath: "include"
        )
    ]
)
