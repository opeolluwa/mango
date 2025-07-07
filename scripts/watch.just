#!/usr/bin/env sh


watch target:
    @echo "Watching {{target}} for changes"
    @just "watch-{{target}}"

[working-directory: 'app']
@watch-android: 
    #!/usr/bin/env sh
    export JAVA_HOME="/Applications/Android Studio.app/Contents/jbr/Contents/Home"
    export ANDROID_HOME="$HOME/Library/Android/sdk"
    export NDK_VERSION=$(ls -1 "$ANDROID_HOME/ndk" | tail -n 1)
    export NDK_HOME="$ANDROID_HOME/ndk/$NDK_VERSION"

    npm run tauri android dev

[working-directory: 'app']
@watch-ios:
    npm run tauri ios dev 'iphone 15'

[working-directory: 'app']
@watch-desktop:
    npm run tauri dev 
