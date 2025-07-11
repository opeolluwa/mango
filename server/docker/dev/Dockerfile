FROM rust:1.86.0-slim

ENV CARGO_TARGET_DIR=/cargo-target 

RUN set -e && \
    apt-get update -q && \
    apt-get install -yq \
        libgcc-s1 \
        libssl-dev \
        pkg-config \
        clang \
        libclang-dev \
        llvm-dev \
        cmake \
        git \
        zlib1g-dev \
        build-essential && \
    rm -rf /var/lib/apt/lists/*

RUN rustup component add rustfmt
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
