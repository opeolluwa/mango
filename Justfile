# Alias
alias install := install-dependencies
alias config:= configure
alias w := watch
alias b := build

# Constants
LAME_PATH := "app/src-tauri/sidecar/lame/bin/lame"
SIDECAR_PATH := "app/src-tauri/sidecar/binaries"
HOST_TRIPLE := "$(rustc -Vv | grep host | cut -f2 -d' ')"

# projects directories 
PIPER_DIR:='piper'
APP_DIR:='app'
LIB_DIR :='lib'

#>>  Default Shows the default commands 
default:
    @just --list --list-heading $'Available commands\n'

#>> execute all initial setup after cloning the project
configure:
    @just configure-piper-rs
    @just install-dependencies

#>> delete rust target dir
clean target:
    #!/usr/bin/env sh
    echo "Cleaning {{target}} build assets"
    if [ "{{target}}" = "lib" ]; then \
        cd {{LIB_DIR}} && cargo clean 
    elif [ "{{target}}" = "piper" ]; then \
        cd {{PIPER_DIR}} && cargo clean
    elif [ "{{target}}" = "app" ]; then \
        cd "{{APP_DIR}}/src-tauri" && cargo clean
    elif [ "{{target}}" = "all" ]; then \
        just clean lib 
        just clean piper 
        just clean app 
    else 
        echo "Invalid {{target}} use one of app,piper,lib"
    fi
        
#>> download piper-rs dependnecies 
[working-directory:'piper']
configure-piper-rs:
    @echo "Configuring piper-rs ...\n"
    git submodule update --init

#>> Dependency Setup
[doc('Install the application dependencies')]
install-dependencies:
    @echo "Installing dependencies"
    cargo install --git https://github.com/cpg314/cargo-group-imports
    cargo install cargo-sort

#>> Test Commands
[working-directory: 'lib']
@test-audio-synthesis:
    cargo run --example audio

#>> the root Watch Commands, it abstarcts other commands like watch-lib watch-app, watch-android
watch target:
    @echo "Watching {{target}} for changes"
    @just "watch-{{target}}"

[working-directory: 'app']
@watch-android:
    #!/usr/bin/env sh
    export BINDGEN_EXTRA_CLANG_ARGS="--sysroot=$NDK_HOME/toolchains/llvm/prebuilt/aarch64-apple-darwin/sysroot"
    just w android

[working-directory: 'app']
@watch-ios:
    just w ios

[working-directory: 'app']
@watch-desktop:
    just w desktop


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
    just b android

[working-directory: 'app']
@build-ios:
    just b ios

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

#>> Local audio conversion using LAME
@convert-local:
    #!/bin/bash
    SOURCE_DIR="/Users/USER/Music/audify"
    OUTPUT_DIR="$SOURCE_DIR"

    for wav_file in "$SOURCE_DIR"/*.wav; do
        base_name=$(basename "$wav_file" .wav)
        mp3_file="$OUTPUT_DIR/$base_name.mp3"
        lame "$wav_file" "$mp3_file"
    done

    rm "$SOURCE_DIR"/*.wav
