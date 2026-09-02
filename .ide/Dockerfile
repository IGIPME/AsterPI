FROM cnbcool/default-dev-env:latest

RUN apt-get update && apt-get install build-essential clang lld -y

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable \
    && . $HOME/.cargo/env \
    && rustup target add wasm32-unknown-unknown \
    && cargo install cargo-leptos \
    && cargo install just \
    && cargo install volo-cli

RUN npm install -g pnpm

ENV PATH="/root/.cargo/bin:${PATH}"