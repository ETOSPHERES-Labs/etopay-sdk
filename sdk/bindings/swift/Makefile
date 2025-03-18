# @file Makefile
# @author Team Lobster 🦞
# @copyright ETOSPHERES Labs GmbH

.PHONY = build_sdk lipo_bundle move_swift_files xcframework compile_swift_main

build_sdk_ios:
	# ios
	rustup target add aarch64-apple-ios
	# ios simulator
	rustup target add aarch64-apple-ios-sim
	rustup target add x86_64-apple-ios

	# ios
	@cargo build --lib --release --locked --target=aarch64-apple-ios
	# ios simulator
	@cargo build --lib --release --locked --target=aarch64-apple-ios-sim
	@cargo build --lib --release --locked --target=x86_64-apple-ios

build_sdk_mac:
	rustup target add aarch64-apple-darwin
	rustup target add x86_64-apple-darwin
	# macos
	@cargo build --lib --release --locked --target=aarch64-apple-darwin
	@cargo build --lib --release --locked --target=x86_64-apple-darwin

lipo_bundle_ios:
	# ios (basically a noop)
	@lipo \
	../../../target/aarch64-apple-ios/release/libwalletsdk_cabi.a -create -output \
	../../../target/aarch64-apple-ios/release/libwalletsdk_cabi.a
	# ios simulator
	@mkdir -p ../../../target/universal-sim/release
	@lipo \
	../../../target/aarch64-apple-ios-sim/release/libwalletsdk_cabi.a \
	../../../target/x86_64-apple-ios/release/libwalletsdk_cabi.a -create -output \
	../../../target/universal-sim/release/libwalletsdk_cabi.a

lipo_bundle_mac:
	# macos
	@mkdir -p ../../../target/universal-macos/release
	@lipo \
	../../../target/aarch64-apple-darwin/release/libwalletsdk_cabi.a \
	../../../target/x86_64-apple-darwin/release/libwalletsdk_cabi.a -create -output \
	../../../target/universal-macos/release/libwalletsdk_cabi.a

move_swift_files:
	find include/generated -name '*.swift' -exec sed -i '' '1s/^/import CawaenaSdkBin\n/' {} \; 
	find include/generated -name '*.swift' -exec mv {} CawaenaSdk/Sources/CawaenaSdk/ \;

xcframework:
	$(MAKE) build_sdk_ios
	$(MAKE) build_sdk_mac
	$(MAKE) lipo_bundle_ios
	$(MAKE) lipo_bundle_mac
	$(MAKE) move_swift_files
	@xcodebuild -create-xcframework \
	-library ../../../target/aarch64-apple-ios/release/libwalletsdk_cabi.a \
	-headers ./include/ \
	-library ../../../target/universal-sim/release/libwalletsdk_cabi.a \
	-headers ./include/ \
	-library ../../../target/universal-macos/release/libwalletsdk_cabi.a \
	-headers ./include/ \
	-output ../../../target/CawaenaSdkBin.xcframework
	cp -r ../../../target/CawaenaSdkBin.xcframework CawaenaSdk/

xcframework_mac:
	$(MAKE) build_sdk_mac
	$(MAKE) lipo_bundle_mac
	$(MAKE) move_swift_files
	@xcodebuild -create-xcframework \
	-library ../../../target/universal-macos/release/libwalletsdk_cabi.a \
	-headers ./include/ \
	-output ../../../target/CawaenaSdkBin.xcframework
	cp -r ../../../target/CawaenaSdkBin.xcframework CawaenaSdk/

compile_swift_main:
	cargo build --release
	xcrun swiftc -L ../../../target/release \
  		-lwalletsdk_cabi\
  		-import-objc-header ./include/bridging-header.h \
		-framework CoreFoundation -framework SystemConfiguration \
  		main.swift ./include/generated/SwiftBridgeCore.swift ./include/generated/cawaena-sdk-swift/cawaena-sdk-swift.swift 
	./main
