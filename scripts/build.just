#!/usr/bin/env sh


# build target:
#     @echo "building {{target}}"
#     @just "build-{{target}}"

# @build-web:





#>> the root Watch Commands, it abstarcts other commands like watch-lib watch-app, watch-android
@build target:
    @echo "Building {{target}} for production"
    @just "build-{{target}}"

#>> Build Commands
@build-onnxruntime:
    export JAVA_HOME="/Applications/Android Studio.app/Contents/jbr/Contents/Home"
    export ANDROID_HOME="$HOME/Library/Android/sdk"
    export NDK_HOME="$ANDROID_HOME/ndk/$(ls -1 $ANDROID_HOME/ndk | head -n 1)"
    ./onnxruntime/build.sh --android --android_sdk_path "$ANDROID_HOME" --android_ndk_path "$NDK_HOME" --android_abi arm64-v8a --android_api 27

[working-directory:'piper-rs']
@build-piper-android:
    #!/usr/bin/env sh
    export JAVA_HOME="/Applications/Android Studio.app/Contents/jbr/Contents/Home"
    export ANDROID_HOME="$HOME/Library/Android/sdk"
    export BINDGEN_EXTRA_CLANG_ARGS="--sysroot=$NDK_HOME/toolchains/llvm/prebuilt/aarch64-apple-darwin/sysroot"
    export NDK_HOME="$ANDROID_HOME/ndk/$(ls -1 $ANDROID_HOME/ndk | head -n 1)"
    ORT_LIB_LOCATION=../onnxruntime/build/Android/Debug
    cargo build --target aarch64-linux-android

[working-directory: 'app']
@build-android:
    npm run tauri android build -- --apk

[working-directory: 'app']
@build-ios:
    npm run tauri android build -- --aab

[working-directory: 'app']
@build-desktop:
    just b desktop

#>> Build LAME
[working-directory: 'lame']
@build-lame:
    #!/bin/bash
    # if [ "{{os()}}" = "windows" ]; then \
    #     echo "Building for Windows in $PWD"
    #     mkdir -p "../app/src-tauri/sidecar/lame/bin"
    #     mkdir -p "../app/src-tauri/sidecar/binaries"
    #     cp "./archive/lame.exe" "../app/src-tauri/sidecar/lame/bin/"
    # fi

    if [ ! -f {{LAME_PATH}} ]; then \
        ./configure --disable-shared --enable-static --enable-nasm --prefix="$(pwd)/../app/src-tauri/sidecar/lame" 
        make 
        make install
    else
        echo "LAME already exists, skipping..."
    fi

#>> Build everything for current platform
[doc('Build the LAME project and prepare sidecar for the target platform')]
@prebuild:
    #!/bin/bash
    cd app && npm run build && cd ..
    mkdir -p "app/src-tauri/sidecar"
    mkdir -p "app/src-tauri/sidecar/binaries"
    # just build-lame

    if [ "{{HOST_TRIPLE}}" = "x86_64-pc-windows-gnu" ]; then
        cp "{{LAME_PATH}}.exe" "{{SIDECAR_PATH}}/lame.exe-{{HOST_TRIPLE}}"
    elif [ "{{HOST_TRIPLE}}" = "aarch64-apple-darwin" ]; then
        cp "{{LAME_PATH}}" "{{SIDECAR_PATH}}/lame-x86_64-apple-darwin"
        cp "{{LAME_PATH}}" "{{SIDECAR_PATH}}/lame-{{HOST_TRIPLE}}"
    else
        cp "{{LAME_PATH}}" "{{SIDECAR_PATH}}/lame-{{HOST_TRIPLE}}"
    fi

    echo "Sidecar copied to {{SIDECAR_PATH}}/lame-{{HOST_TRIPLE}}"
