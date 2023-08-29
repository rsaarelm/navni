# Run game in TTY terminal mode.
test:
    @cargo test --features=gui,tty --all

# Spin up a test web server to run the WASM binary
run-wasm: build-wasm
    @cargo install basic-http-server
    @echo Starting WASM game server at http://localhost:4000/
    ~/.cargo/bin/basic-http-server web/

# Build a WASM version
build-wasm:
    cargo build --target=wasm32-unknown-unknown --features=gui --release --example demo
    cp target/wasm32-unknown-unknown/release/examples/demo.wasm web/
