FROM rust:1.86.0-slim-bullseye

ENV CARGO_TARGET_DIR=/cargo-target 

RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev build-essential && \
    rm -rf /var/lib/apt/lists/*
    
# RUN apt update && apt install libmp3lame0
# RUN lame --version 

WORKDIR /app

COPY . .

RUN cargo install cargo-watch

RUN cargo build 

VOLUME ["/app", "/cargo-target", "/usr/local/cargo/registry"]

CMD ["cargo", "watch", \
     "--why", \
     "--ignore", ".git/", \
     "--ignore", "target/", \
     "-w", "src", \
     "-w", "Cargo.toml", \
     "-w", "migrations", \
     "-x", "check", \
     "-x", "run"]
