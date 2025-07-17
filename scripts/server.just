alias sw:= server-watch
alias sk:= server-kill
alias sb:= server-build
alias sl:= server-logs
alias sr:= server-restart
alias srb:= server-restart
alias sp := server-build-production


set dotenv-required
set dotenv-load := true
set dotenv-path := "server/.env"
set export :=  true
# set working-directory :="server"

DOCKER_CMD := "docker compose -f server/docker-compose.yaml"
#>> the root Server Commands, it abstarcts other commands like watch-server, build-server, kill-server, e.t.c
[working-directory: 'server']
@server target:
    @echo "Building {{target}} for production"
    @just "server-{{target}}"

@server-default: 
    @just --list --list-heading $'Available commands\n'

@server-lint:
    cargo fmt && cargo fix 

@server-watch: 
    {{DOCKER_CMD}} up -d 
    @just sl

@server-logs:
    {{DOCKER_CMD}} logs -f --tail='30' app

@server-build:
    cargo run build --release 

@server-kill:  
    {{DOCKER_CMD}} down -v 

@server-restart:
    @just server-kill
    @just server-watch

@server-rebuild:
    docker compose -f server/docker-compose.yaml up --build 

@server-build-production target:
    docker build . -t aers-0.6.8 --file docker/prod/Dockerfile
    docker run aers-{{target}}