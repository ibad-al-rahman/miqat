// swift-tools-version: 6.0
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

// Never push to remote with this flag set to true
let useLocalFramework = false
let releaseTag = "0.6.1"
let releaseChecksum = "8e27c8a1e10a04c5b904a169e824771606ab19fca568fde62a2767b06e78020d"

let binaryTarget: Target = if useLocalFramework {
    .binaryTarget(
        name: "MiqatFFI",
        path: "./target/ios/libmiqat-rs.xcframework"
    )
} else {
    .binaryTarget(
        name: "MiqatFFI",
        url: "https://github.com/ibad-al-rahman/miqat/releases/download/\(releaseTag)/libmiqat-rs.xcframework.zip",
        checksum: releaseChecksum
    )
}

let package = Package(
    name: "Miqat",
    platforms: [.iOS(.v13)],
    products: [
        .library(name: "Miqat", targets: ["Miqat"]),
    ],
    targets: [
        binaryTarget,
        .target(
            name: "Miqat",
            dependencies: [.target(name: "MiqatFFI")],
            path: "apple/Sources/Miqat",
            resources: [
                .process("Resources/PrivacyInfo.xcprivacy")
            ]
        )
    ]
)
