#!/usr/bin/env sh

package target:
    @echo "Packaging {{target}} "
    @just "package-{{target}}"

@package-web:

@package-server:
    cargo install css-inline
    cargo install mailtutan

[working-directory :'app']
@package-android:
    mkdir -p build
    cp  /Users/USER/Desktop/projects/audify/app/src-tauri/gen/android/app/build/outputs/apk/universal/release/*apk build/eckko-mobile.apk