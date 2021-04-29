// swift-tools-version:5.4.0

import PackageDescription

let package = Package(
    name: "WordCount",
    platforms: [
        .macOS(.v11)
    ],
    products: [
        .library(
            name: "WordCount",
            targets: ["WordCount"]),
    ],
    dependencies: [],
    targets: [
        .target(
            name: "WordCount",
            dependencies: []),
        .testTarget(
            name: "WordCountTests",
            dependencies: ["WordCount"]),
    ]
)
