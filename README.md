# WASI with Component Model example
Minimum example for WASI module with Component Model

## with Docker
```console
$ docker build -t wasi-component .
$ docker run -it --rm wasi-component /bin/bash
$ make run
```

## without Docker
```console
$ rustup install 1.71.0-x86_64-unknown-linux-gnu
$ rustup default 1.71.0-x86_64-unknown-linux-gnu
$ rustup target add wasm32-wasi
$ cargo install wasm-tools
$ ./build.sh && make run
```
