FROM rust:1.98-slim-bookworm AS builder

ENV CARGO_BUILD_JOBS=1
ENV CARGO_HTTP_MULTIPLEXING=true
ENV RUST_BACKTRACE=1

RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    curl \
    ca-certificates \
    build-essential \
    clang \
    lld \
    && rm -rf /var/lib/apt/lists/*

RUN rustup toolchain install 1.98.0 --profile minimal \
        -c rust-src \
        -t wasm32-unknown-unknown,x86_64-unknown-linux-gnu && \
    rustup default 1.98.0

ARG CARGO_LEPTOS_VERSION=v0.3.7
RUN ARCH=$(uname -m) && \
    cargo install cargo-leptos && \
    cargo leptos --version

WORKDIR /build

COPY Cargo.toml Cargo.lock rust-toolchain.toml ./
COPY .cargo ./.cargo/
COPY crates/app/Cargo.toml ./crates/app/
COPY crates/frontend/Cargo.toml ./crates/frontend/
COPY crates/server/Cargo.toml ./crates/server/

COPY . .