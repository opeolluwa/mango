clean target:
    #!/usr/bin/env sh
    echo "Cleaning {{target}} build assets"
    if [ "{{target}}" = "web" ]; then \
        rm -rf node_modules dist
    elif [ "{{target}}" = "app" ]; then \
        cd "{{APP_DIR}}/src-tauri" && cargo clean
    elif [ "{{target}}" = "server" ]; then \
        cd "{{SERVER_DIR}}" && cargo clean
    elif [ "{{target}}" = "all" ]; then \
        just clean web
        just clean server
        just clean app 
    else 
        echo "Invalid {{target}} use one of app,piper,lib"
    fi
