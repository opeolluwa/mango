alias sw:= server-watch
alias sk:= server-kill
alias sb:= server-build
alias sl:= server-logs
alias sr:= server-restart


set dotenv-required
set dotenv-load := true
set dotenv-path := "server/.env"
set export :=  true
# set working-directory :="server"

#>> the root Server Commands, it abstarcts other commands like watch-server, build-server, kill-server, e.t.c
[working-directory: 'server']
@server target:
    @echo "Building {{target}} for production"
    @just "server-{{target}}"

@server-default: 
    @just --list --list-heading $'Available commands\n'

@server-lint:
    cargo fmt && cargo fix 

[working-directory: 'server']
@server-watch: 
    docker compose up -d 
    @just l

[working-directory: 'server']
@server-logs:
    docker compose logs -f --tail='30' app

[working-directory: 'server']
@server-build:
    cargo run build --release 

[working-directory: 'server']
@server-kill:  
    docker compose down -v 

[working-directory: 'server']
@server-restart:
    @just kill
    @just watch