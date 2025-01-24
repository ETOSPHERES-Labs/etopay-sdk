// swift-tools-version:5.5.0
import PackageDescription
let package = Package(
	name: "CryptpaySdk",
	platforms: [
    	.iOS(.v13),
    	.macOS(.v11),
  	],
	products: [
		.library(
			name: "CryptpaySdk",
			targets: ["CryptpaySdk"]),
	],
	dependencies: [],
	targets: [
		.binaryTarget(
			name: "CryptpaySdkBin",
			path: "CryptpaySdkBin.xcframework"
		),
		.target(
			name: "CryptpaySdk",
			dependencies: ["CryptpaySdkBin"])
	]
)