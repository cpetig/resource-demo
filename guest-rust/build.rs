fn main() {
    // Till we can set default-targets per crate
    assert_eq!(
        std::env::var("TARGET").unwrap(),
        "wasm32-wasi",
        "\nSpecify --target wasm32-wasi for this package\n cargo build -p guest --target wasm32-unknown-unknown"
    );
}
