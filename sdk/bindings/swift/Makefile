# @file Makefile
# @author Team Lobster 🦞
# @copyright ETO GRUPPE TECHNOLOGIES GmbH

.PHONY = ensure_rustup_targets build_sdk lipo_bundle move_swift_files xcframework compile_swift_main

ensure_rustup_targets:
	# ios
	rustup target add aarch64-apple-ios
	# ios simulator
	rustup target add aarch64-apple-ios-sim
	rustup target add x86_64-apple-ios
	# macos
	rustup target add aarch64-apple-darwin
	rustup target add x86_64-apple-darwin

build_sdk:
	$(MAKE) ensure_rustup_targets
	# ios
	@cargo build --lib --release --locked --target=aarch64-apple-ios
	# ios simulator
	@cargo build --lib --release --locked --target=aarch64-apple-ios-sim
	@cargo build --lib --release --locked --target=x86_64-apple-ios
	# macos
	@cargo build --lib --release --locked --target=aarch64-apple-darwin
	@cargo build --lib --release --locked --target=x86_64-apple-darwin

lipo_bundle:
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
	# macos
	@mkdir -p ../../../target/universal-macos/release
	@lipo \
	../../../target/aarch64-apple-darwin/release/libwalletsdk_cabi.a \
	../../../target/x86_64-apple-darwin/release/libwalletsdk_cabi.a -create -output \
	../../../target/universal-macos/release/libwalletsdk_cabi.a

move_swift_files:
	find include/generated -name '*.swift' -exec sed -i '' '1s/^/import CryptpaySdkBin\n/' {} \; 
	find include/generated -name '*.swift' -exec mv {} CryptpaySdk/Sources/CryptpaySdk/ \;

xcframework:
	$(MAKE) build_sdk
	$(MAKE) lipo_bundle
	$(MAKE) move_swift_files
	@xcodebuild -create-xcframework \
	-library ../../../target/aarch64-apple-ios/release/libwalletsdk_cabi.a \
	-headers ./include/ \
	-library ../../../target/universal-sim/release/libwalletsdk_cabi.a \
	-headers ./include/ \
	-library ../../../target/universal-macos/release/libwalletsdk_cabi.a \
	-headers ./include/ \
	-output ../../../target/CryptpaySdkBin.xcframework
	cp -r ../../../target/CryptpaySdkBin.xcframework CryptpaySdk/

compile_swift_main:
	cargo build --release
	swiftc -L ../../../target/release \
  		-lwalletsdk_cabi\
  		-import-objc-header ./include/bridging-header.h \
		-framework CoreFoundation -framework SystemConfiguration \
  		main.swift ./include/generated/SwiftBridgeCore.swift ./include/generated/cryptpay-sdk-swift/cryptpay-sdk-swift.swift 
	./main
