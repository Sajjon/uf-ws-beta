// swift-tools-version: 5.9
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let binaryTargetName = "DemoCoreRS"
let binaryTarget = Target.binaryTarget(
  name: binaryTargetName,
  path: "./target/swift/libtwo-rs.xcframework"
)

let package = Package(
  name: "DemoPackage",
  platforms: [
    .iOS(.v16), .macOS(.v13),
  ],
  products: [
    .library(
      name: "DemoUniFFI",
      targets: ["DemoUniFFI"]
    )
  ],
  dependencies: [],
  targets: [
    binaryTarget,
    .target(
      name: "DemoUniFFI",
      dependencies: [.target(name: binaryTargetName)],
      path: "swift/Sources/UniFFI"
    ),
    .testTarget(
      name: "DemoUniFFITests",
      dependencies: [
        .target(name: "DemoUniFFI")
      ],
      path: "swift/Tests"
    ),
  ]
)
