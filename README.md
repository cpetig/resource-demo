# A demo showing WASM component model resources in various environments

## Prerequisites

Rust with target wasm32-wasi

CMake

## Compiling and Running

```bash
cd guest-rust
cargo build --target wasm32-wasi --release
cd ../host-wamr
mkdir build
cd build
cmake ..
make
./executor
```

Expected Output:

`drop with value 34`

### C guest

Prerequisite: wasi-SDK

```bash
cd guest-c
make
cd ..
ln -sf guest-c/guest-c.wasm guest.wasm 
```

## Regenerating C++ host bindings

```bash
git clone https://github.com/cpetig/wit-bindgen.git
cd wit-bindgen
cargo install --path .
cd ../host-wamr
wit-bindgen cpp-host ../wit/simple.wit
```

## Other environments

### Host languages

- Rust:

    Wasmtime host doesn't support generating bindings for resources.
    https://github.com/bytecodealliance/wasmtime/issues/6722


    Wasmer bindgen only supports the old witx, not the newer wit format. You could write a host binding generator within wit-bindgen though.

- JavaScript: JCO doesn't support resources yet, IIRC.

- C: Currently I generate C++ host bindings for WAMR, C would be feasible.

- Python: https://pypi.org/project/componentize-py/ (?)

### Guest languages

- JavaScript: https://github.com/bytecodealliance/javy, you will need another tool to generate guest bindings for imported functions, IIRC. A path using C bindings with yet to be written JavaScript wrapping should be viable.

- Python: https://pypi.org/project/componentize-py/ 

- Go: WIT bindgen support is being worked on https://github.com/bytecodealliance/SIG-Guest-Languages/blob/main/docs/subgroups.md
