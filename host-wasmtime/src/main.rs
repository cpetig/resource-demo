use anyhow::{Result, anyhow};

use wasmtime::{
    self,
    component::{Component, Linker},
    Config, Engine, Store,
};

use crate::test::example::my_interface::MyObject;

wasmtime::component::bindgen!({ path: "../wit/simple.wit", world: "my-world" });

struct ObjectImpl {
    value: u32,
}

struct HostState {
    object_table: std::collections::HashMap<u32, ObjectImpl>,
}

impl test::example::my_interface::HostMyObject for HostState {
    // type Resource = ObjectImpl;

    fn new(
        store: &mut wasmtime::StoreContextMut<'_, Self>,
        a: u32,
    ) -> wasmtime::Result<wasmtime::component::Resource<ObjectImpl>>
    where
        Self: Sized,
    {
        let state = store.data_mut();

        let handle = wasmtime::component::Resource::<ObjectImpl>::new_own(
            state.object_table.len() as u32,
        );
        handle
            .object_table
            .insert(handle.rep(), ObjectImpl { value: a });
        Ok(handle)
    }

    fn set(
        &mut self,
        res: wasmtime::component::Resource<ObjectImpl>,
        v: u32,
    ) -> wasmtime::Result<()>
    where
        Self: Sized,
    {
        self.object_table
            .get_mut(&res.rep())
            .and_then(|o| o.value = v)?;
        Ok(())
    }

    fn get(&mut self, res: wasmtime::component::Resource<ObjectImpl>) -> wasmtime::Result<u32>
    where
        Self: Sized,
    {
        self.object_table
            .get(&res.rep())
            .map(|o| Ok(o.value))
            .ok_or(anyhow!(
                "tried to get a resource `{}` that doesn't exist",
                rep
            ))
    }

    fn drop(mut store: wasmtime::StoreContextMut<'_, Self>, rep: u32) -> wasmtime::Result<()> {
        let state = store.data_mut();

        state
            .object_table
            .remove(&rep)
            .and_then(|o| println!("Value at drop {}", o.value))
            .ok_or(anyhow!(
                "tried to drop a resource `{}` that doesn't exist",
                rep
            ))?;
        Ok(())
    }
}

// As guest, when you build the output is a wasm module, not a component in terms of wasmtime
// this allows us to do it in rust however as i'll note below it's preferred to do it through the wasm-tools cli
// and just import with Component::from_file
use wit_component::ComponentEncoder;

fn main() -> Result<()> {
    let mut config = Config::new();

    Config::wasm_component_model(&mut config, true);

    let engine = Engine::new(&config)?;
    let mut store = Store::new(&engine, 0);
    let linker = Linker::new(&engine);

    let wasm_module_path = "../guest.wasm";

    let module =
        std::fs::read(wasm_module_path).expect("WASM Module missing, did you build guest-rust?");

    let component = ComponentEncoder::default()
        .module(module.as_slice())?
        .validate(true)
        .encode()?;

    let component = Component::from_binary(&engine, &component)?;

    let start = component.get_typed_func::<(), ()>(&mut store, "_start")?;

    start.call(&mut store, ())?;

    Ok(())
}
