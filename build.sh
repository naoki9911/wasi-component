#!/bin/bash

ADAPT_WASM="wasi_snapshot_preview1.wasm"
if [ ! -e $ADAPT_WASM ]; then
    wget https://github.com/bytecodealliance/wasmtime/releases/download/v10.0.1/wasi_snapshot_preview1.reactor.wasm -O $ADAPT_WASM
fi

make build
