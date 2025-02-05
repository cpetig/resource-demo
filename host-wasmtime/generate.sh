#!/bin/sh
ln -s ../guest-rust/target/wasm32-wasip1/release/guest-rust.wasm
wasm-tools component new guest-rust.wasm -o component.wasm --adapt wasi_snapshot_preview1.command.wasm 
