alias install := install-dependencies

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
    ./configure --disable-shared --enable-static --enable-nasm --prefix=$(pwd)/../app/src-tauri/sidecar/lame
    make
    make install

