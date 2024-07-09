#!/usr/bin/env zsh

set -e # Exit immediately if a command exits with a non-zero status.
set -u # Treat unset variables as an error when substituting.

function build_swift() {

	local NAME=$1
	local TARGET="aarch64-apple-darwin"
	local OUT=target/uf-xcframework
	local SWIFT_UF=swift/Sources/UniFFI
	local BIN=target/swift
	local LIB_BASE=target/$TARGET/release/lib$NAME
	echo "📦 LIB_BASE: $LIB_BASE"
	echo "📦 headers: $OUT"

	# Build Rust
	cargo build --lib --release --target $TARGET

	# Bindgen
	cargo run -p demo-bindgen generate  \
		--library $LIB_BASE.dylib \
		--language swift --out-dir $OUT

	# Create Xcframework
	rm -rf $SWIFT_UF
	mkdir -p $SWIFT_UF
	mv $OUT/*.swift $SWIFT_UF
	mv $OUT/${NAME}FFI.modulemap $OUT/module.modulemap  

	rm -rf $BIN
	XCFRAME_PATH=$BIN/lib${NAME}-rs.xcframework

	xcodebuild -create-xcframework \
		-library ${LIB_BASE}.a -headers $OUT \
		-output $XCFRAME_PATH
}


# Test
cargo test

# Build Swift
build_swift two
# Test Swift
swift test