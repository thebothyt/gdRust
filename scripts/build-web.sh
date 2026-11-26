#!/usr/bin/env sh
# rebuilds web/gdrust.wasm, commit the result since vercel won't build rust for you
set -e
rustup target add wasm32-unknown-unknown
cargo build -p gd_rust_wasm --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/gd_rust_wasm.wasm web/gdrust.wasm
echo "web/gdrust.wasm updated"
