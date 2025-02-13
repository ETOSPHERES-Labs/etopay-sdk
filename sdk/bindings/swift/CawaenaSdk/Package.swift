// swift-tools-version:5.5.0
import PackageDescription

let package = Package(
	name: "CawaenaSdk",
	platforms: [
		.iOS(.v13),
		.macOS(.v11),
	],
	products: [
		.library(
			name: "CawaenaSdk",
			targets: ["CawaenaSdk"])
	],
	dependencies: [],
	targets: [
		.binaryTarget(
			name: "CawaenaSdkBin",
			path: "CawaenaSdkBin.xcframework"
		),
		.target(
			name: "CawaenaSdk",
			dependencies: ["CawaenaSdkBin"]),
	]
)
