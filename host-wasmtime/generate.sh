#!/bin/sh
ln -s ../guest-rust/target/wasm32-wasi/release/guest-rust.wasm
wasm-tools component new guest-rust.wasm -o component.wasm --adapt wasi_snapshot_preview1.wasm 
