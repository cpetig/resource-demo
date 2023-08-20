#!/bin/sh
ln -s ../guest-rust/target/wasm32-wasi/release/guest-rust.wasm
wasm-tools component new guest-rust.wasm -o component.wasm --adapt ~/wasi_snapshot_preview1.wasm 
jco transpile component.wasm -o html --no-typescript \
    --map wasi:filesystem/*=./bytecodealliance/preview2-shim/filesystem.js \
    --map wasi:cli/*=./bytecodealliance/preview2-shim/cli.js \
    --map wasi:io/*=./bytecodealliance/preview2-shim/io.js \
    --map test:example/my-interface=./test_example/my-interface.js
echo Manually edit html/component.js to refer to ./test_example/my-interface.js
echo   Perhaps --map can handle this somehow
echo 'Manually call run() at the end of html/component.js'
echo 'simple-http-server --cors --coop --coep --nocache -i html'
