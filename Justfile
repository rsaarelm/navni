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

update-flake:
    rm -rf .direnv/
    nix flake update

# Download and minify JS shims
generate-minified-js:
    #!/bin/sh
    OUT=$(pwd)/web
    TMPDIR=$(mktemp -d)
    cd $TMPDIR

    wget https://raw.githubusercontent.com/not-fl3/miniquad/master/js/gl.js
    wget https://raw.githubusercontent.com/optozorax/quad-storage/master/js/quad-storage.js
    wget https://raw.githubusercontent.com/not-fl3/sapp-jsutils/master/js/sapp_jsutils.js

    minify gl.js > $OUT/gl.js
    minify quad-storage.js > $OUT/quad-storage.js
    minify sapp_jsutils.js > $OUT/sapp_jsutils.js
