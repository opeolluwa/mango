alias install := install-dependencies

LAME_PATH:= "app/src-tauri/sidecar/lame/bin/lame"
SIDECAR_PATH :="app/src-tauri/sidecar/binaries"

default: 
    @just --list --list-heading $'Available commands\n'

[doc('Install the application dependencies')]
install-dependencies: 
    echo "Installing dependencies"
    cargo install --git https://github.com/cpg314/cargo-group-imports
    cargo install cargo-sort
    


#>> test commands 
[working-directory: 'lib']
@test-audio-synthensis:
    cargo run --example audio

#>> watch commands 
[working-directory: 'app']
@watch-android:
    just w android 

[working-directory: 'app']
@watch-io:
    just w ios

[working-directory: 'app']
@watch-desktop:
    just w desktop 

#>> build commands 
build-onnxruntime:
    export JAVA_HOME="/Applications/Android Studio.app/Contents/jbr/Contents/Home"
    export ANDROID_HOME="$HOME/Library/Android/sdk"
    export NDK_HOME="$ANDROID_HOME/ndk/$(ls -1 $ANDROID_HOME/ndk)"

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


[working-directory:'lame']
@build-lame:
    #!/bin/bash
    # set -e
    if [ {{os()}} = "windows" ]; then \
        cp "archive\\lame.exe" "{{LAME_PATH}}.exe"; \
    fi

    if [ ! -f "{{LAME_PATH}}" ]; then
        ./configure --disable-shared --enable-static --enable-nasm --prefix=$(pwd)/../app/src-tauri/sidecar/lame
        make
        make install
    else
        echo "LAME already exists, skipping..."
    fi




#TODO: build for all platforms in CI
[doc('build the lame project for the target platform')]
@prebuild:
    @cd app && npm run build  && cd ..
    @mkdir  -p "app/src-tauri/sidecar"
    @mkdir  -p "app/src-tauri/sidecar/binaries"
    @just build-lame
    @cp "{{LAME_PATH}}" "{{SIDECAR_PATH}}/lame-$(rustc -Vv | grep host | cut -f2 -d' ')"
    @echo "sidecar copied to {{SIDECAR_PATH}}/lame-$(rustc -Vv | grep host | cut -f2 -d' ')"

@convert-local:
    #!/bin/bash

    SOURCE_DIR="/Users/USER/Music/audify"   
    OUTPUT_DIR="$SOURCE_DIR"

    for wav_file in "$SOURCE_DIR"/*.wav; do
        base_name=$(basename "$wav_file" .wav)
        mp3_file="$OUTPUT_DIR/$base_name.mp3"
        lame "$wav_file" "$mp3_file"
    done

    rm "/Users/USER/Music/audify/*.wav"
