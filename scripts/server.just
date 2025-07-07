alias sw:= server-watch
alias sk:= server-kill
alias sb:= server-build
alias sl:= server-logs
alias sr:= server-restart


set dotenv-required
set dotenv-load := true
set dotenv-path := "server/.env"
set export :=  true

#>> the root Server Commands, it abstarcts other commands like watch-server, build-server, kill-server, e.t.c
@server target:
    @echo "Building {{target}} for production"
    @just "server-{{target}}"

@server-default: 
    @just --list --list-heading $'Available commands\n'

@server-lint:
    cargo fmt && cargo fix 

@server-watch: 
    docker compose up -d 
    @just l

@server-logs:
    docker compose logs -f --tail='30' app

@server-build:
    cargo run build --release 

@server-kill:  
    docker compose down -v 

@server-restart:
    @just kill
    @just watch