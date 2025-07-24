import 'scripts/watch.just'
import 'scripts/build.just'
import 'scripts/package.just'
import 'scripts/clean.just'
import 'scripts/server.just'
import 'scripts/lint.just'



# Alias
alias install := install-dependencies
alias config:= configure
alias w := watch
alias b := build
alias c := clean
alias p := package

# Constants
LAME_PATH := "app/src-tauri/sidecar/lame/bin/lame"
SIDECAR_PATH := "app/src-tauri/sidecar/binaries"
HOST_TRIPLE := "$(rustc -Vv | grep host | cut -f2 -d' ')"

# projects directories 
PIPER_DIR:='piper'
APP_DIR:='app'
LIB_DIR :='lib'
WEB_DIR :='web'
SERVER_DIR :='server'

#>>  Default Shows the default commands 
default:
    @just --list --list-heading $'Available commands\n'

#>> execute all initial setup after cloning the project
configure:
    # @just configure-piper-rs
    # @just install-dependencies
    rustup target add aarch64-unknown-linux-gnu

        
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
    brew install coreutils gnu-sed
