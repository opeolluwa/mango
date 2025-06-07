# Alias
alias install := install-dependencies

# Constants
LAME_PATH := "app/src-tauri/sidecar/lame/bin/lame"
SIDECAR_PATH := "app/src-tauri/sidecar/binaries"
HOST_TRIPLE := "$(rustc -Vv | grep host | cut -f2 -d' ')"

# Default
default:
    @just --list --list-heading $'Available commands\n'

#>> Dependency Setup
[doc('Install the application dependencies')]
install-dependencies:
    echo "Installing dependencies"
    cargo install --git https://github.com/cpg314/cargo-group-imports
    cargo install cargo-sort

#>> Test Commands
[working-directory: 'lib']
@test-audio-synthesis:
    cargo run --example audio

#>> Watch Commands
[working-directory: 'app']
@watch-android:
    just w android

[working-directory: 'app']
@watch-ios:
    just w ios

[working-directory: 'app']
@watch-desktop:
    just w desktop

#>> Build Commands
build-onnxruntime:
    export JAVA_HOME="/Applications/Android Studio.app/Contents/jbr/Contents/Home"
    export ANDROID_HOME="$HOME/Library/Android/sdk"
    export NDK_HOME="$ANDROID_HOME/ndk/$(ls -1 $ANDROID_HOME/ndk | head -n 1)"
    ./onnxruntime/build.sh --android --android_sdk_path "$ANDROID_HOME" --android_ndk_path "$NDK_HOME" --android_abi arm64-v8a --android_api 27

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
    if [ "{{os()}}" = "windows" ]; then \
        echo "Building for Windows in $PWD"
        mkdir -p "../app/src-tauri/sidecar/lame/bin"
        mkdir -p "../app/src-tauri/sidecar/binaries"
        cp "./archive/lame.exe" "../app/src-tauri/sidecar/lame/bin/"
    fi

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
    just build-lame

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
