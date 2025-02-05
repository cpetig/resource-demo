use std::io::{Read, Write};

use wasmtime::{Config, Engine};

fn main() -> wasmtime::Result<()> {
    let mut config = Config::new();
    let mut f1 = std::fs::File::open("../../guest.wasm")?;
    let mut bytes = Vec::new();
    f1.read_to_end(&mut bytes)?;
    drop(f1);
    if cfg!(target_pointer_width = "64") {
        config.target("pulley64")?;
    } else {
        config.target("pulley32")?;
    }
    let engine = Engine::new(&config)?;
    let precompiled = engine.precompile_module(&bytes)?;
    let mut f2 = std::fs::File::create("../guest.cwasm")?;
    f2.write_all(&precompiled)?;
    Ok(())
}
