#!/usr/bin/env sh


watch target:
    @echo "Watching {{target}} for changes"
    @just "watch-{{target}}"

[working-directory: 'app']
@watch-android: 
    just android-watch

[working-directory: 'app']
@watch-ios:
    npm run tauri ios dev 'iphone 15'

[working-directory: 'app']
@watch-desktop:
    npm run tauri dev 

[working-directory: 'app']
@watch-frontend: 
    npm run dev 



[working-directory: 'server']
@watch-server: 
   @just server-watch
