# @file Makefile
# @author Sharang Parnerkar
#
# @copyright ETOSPHERES Labs GmbH
#

.PHONY = clean build_sdk bundle lib_example run_example build_javadoc

build_sdk:
	@cargo ndk -o ./jniLibs -t armeabi-v7a -t arm64-v8a -t x86 -t x86_64 build --release --locked

bundle:build_sdk
	cd jar/java/com/cawaena && javac Wallet.java
	cd jar && jar cf Wallet.jar -C java/ .

clean:
	@cargo clean
	@rm -rf ./jar
	@echo done.


lib_example:
	cargo build

run_example: lib_example
	cd examples && ./gradlew run --args $(example)

build_javadoc:
	javadoc -d javadoc -sourcepath ./tests/src/main/java com.cawaena
