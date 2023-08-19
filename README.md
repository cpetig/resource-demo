# A demo showing WASM component model resources in various environments

## Prerequisites

Rust and target wasm32-unknown-unknown

CMake

## Compiling and Running
```bash
cd guest-rust
cargo build --target wasm32-unknown-unknown --release
cd ../host-wamr
mkdir build
cd build
cmake ..
make

```

## Regenerating C++ host bindings

```bash
git clone https://github.com/cpetig/wit-bindgen.git
cd wit-bindgen
cargo install --path .
cd ../host-wamr
wit-bindgen cpp-host ../wit/simple.wit
```
