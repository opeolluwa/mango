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

RUN find target/release/build -name "libtika_native.so" -exec cp {} /usr/local/lib/ \;

# Final stage
FROM debian:bullseye-slim AS final

RUN apt-get update && \
    apt-get install -y libssl-dev && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=build /app/target/release/aers ./aers
COPY --from=build /app/resources ./resources
COPY --from=build /app/migrations ./migrations
COPY --from=build /usr/local/lib/libtika_native.so /usr/local/lib

EXPOSE 8080

CMD ["./aers"]
