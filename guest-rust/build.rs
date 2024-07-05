fn main() {
    // Till we can set default-targets per crate
    assert_eq!(
        &std::env::var("TARGET").unwrap()[..11],
        "wasm32-wasi",
        "\nSpecify --target wasm32-wasip1 for this package\n cargo build --target wasm32-wasip1"
    );
}
