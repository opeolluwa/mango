FROM rust:1.86.0-slim AS build

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

WORKDIR /app

COPY . .

RUN cargo build --release

RUN ls 

# RUN ls -lh target/release && file target/release/aers

FROM debian:bullseye-slim AS final

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

COPY --from=build /app/target/release/aers ./aers
COPY --from=build /app/resources /resources
COPY --from=build /app/migrations /migrations


EXPOSE 8080

CMD ["aers"]
