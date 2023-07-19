.PHONY: build
build:
	cargo build --manifest-path ./guest/Cargo.toml --target wasm32-wasi --release
	wasm-tools component new ./target/wasm32-wasi/release/guest.wasm -o guest_component.wasm --adapt ./wasi_snapshot_preview1.wasm
	cargo build --manifest-path ./host/Cargo.toml --release

.PHONY: run
run:
	cargo run --manifest-path ./host/Cargo.toml --release
