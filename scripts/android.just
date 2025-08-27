#!/usr/bin/env sh

# Android-related tasks centralized here

@android target:
	@echo "Android: running {{target}}"
	@just "android-{{target}}"

[working-directory:'piper-rs']
@android-build-onnxruntime:
	export JAVA_HOME="/Applications/Android Studio.app/Contents/jbr/Contents/Home"
	export ANDROID_HOME="$HOME/Library/Android/sdk"
	export NDK_HOME="$ANDROID_HOME/ndk/$(ls -1 $ANDROID_HOME/ndk | head -n 1)"
	./onnxruntime/build.sh --android --android_sdk_path "$ANDROID_HOME" --android_ndk_path "$NDK_HOME" --android_abi arm64-v8a --android_api 27

[working-directory:'piper-rs']
@android-build-piper:
	export JAVA_HOME="/Applications/Android Studio.app/Contents/jbr/Contents/Home"
	export ANDROID_HOME="$HOME/Library/Android/sdk"
	export BINDGEN_EXTRA_CLANG_ARGS="--sysroot=$NDK_HOME/toolchains/llvm/prebuilt/aarch64-apple-darwin/sysroot"
	export NDK_HOME="$ANDROID_HOME/ndk/$(ls -1 $ANDROID_HOME/ndk | head -n 1)"
	ORT_LIB_LOCATION=../onnxruntime/build/Android/Debug
	cargo build --target aarch64-linux-android

[working-directory: 'app']
@android-build-apk:
	npm run tauri android build -- --apk

[working-directory: 'app']
@android-build-aab:
	npm run tauri android build -- --aab

[working-directory: 'app']
@android-watch:
	#!/usr/bin/env sh
	export JAVA_HOME="/Applications/Android Studio.app/Contents/jbr/Contents/Home"
	export ANDROID_HOME="$HOME/Library/Android/sdk"
	export NDK_VERSION=$(ls -1 "$ANDROID_HOME/ndk" | tail -n 1)
	export NDK_HOME="$ANDROID_HOME/ndk/$NDK_VERSION"
	export NODE_ENV="development"
	npm run tauri android dev


[working-directory: 'app']
@android-init:
	#!/usr/bin/env sh
	export JAVA_HOME="/Applications/Android Studio.app/Contents/jbr/Contents/Home"
	export ANDROID_HOME="$HOME/Library/Android/sdk"
	export NDK_VERSION=$(ls -1 "$ANDROID_HOME/ndk" | tail -n 1)
	export NDK_HOME="$ANDROID_HOME/ndk/$NDK_VERSION"
	export NODE_ENV="development"
	npm run tauri android init


