FROM ubuntu:22.04

RUN apt update && apt upgrade -y
RUN apt install -y curl vim build-essential

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

ENV PATH=/root/.cargo/bin:$PATH

RUN rustup install 1.71.0-x86_64-unknown-linux-gnu
RUN rustup default 1.71.0-x86_64-unknown-linux-gnu
RUN rustup target add wasm32-wasi
RUN cargo install wasm-tools
RUN cargo install cargo-expand

COPY . /wasi-component
WORKDIR /wasi-component

RUN ./build.sh
