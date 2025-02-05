use std::io::Read;

use wasmtime::{Config, Engine, Linker, Module, Store};

fn main() -> wasmtime::Result<()> {
    let mut config = Config::new();
    let mut f1 = std::fs::File::open("guest.cwasm")?;
    let mut bytes = Vec::new();
    f1.read_to_end(&mut bytes)?;
    drop(f1);
    if cfg!(target_pointer_width = "64") {
        config.target("pulley64")?;
    } else {
        config.target("pulley32")?;
    }
    let engine = Engine::new(&config)?;
    let module = unsafe { Module::deserialize(&engine, &bytes) }?;
    let mut linker: Linker<u32> = Linker::new(&engine);
    linker.define_unknown_imports_as_traps(&module)?;
    let mut store = Store::new(&engine, 42);
    let instance = linker.instantiate(&mut store, &module)?;
    let func = instance.get_typed_func::<(), ()>(&mut store, "_start")?;
    func.call(&mut store, ())?;
    Ok(())
}
