use anyhow::{anyhow, Result};

use test::example::my_interface::Host;
use wasmtime::{
    self,
    component::{Component, Linker, Resource},
    Config, Engine, Store,
};

use crate::test::example::my_interface::MyObject;

wasmtime::component::bindgen!({
    path: "../wit/simple.wit",
    world: "my-world",
});

struct ObjectImpl {
    value: u32,
}

#[derive(Default)]
struct HostState {
    object_table: std::collections::HashMap<u32, ObjectImpl>,
}

impl Host for HostState {}

impl test::example::my_interface::HostMyObject for HostState {
    fn new(&mut self, a: u32) -> wasmtime::Result<Resource<MyObject>> {
        let handle = Resource::<MyObject>::new_own(self.object_table.len() as u32);
        self.object_table
            .insert(handle.rep(), ObjectImpl { value: a });
        Ok(handle)
    }

    fn set(&mut self, res: Resource<MyObject>, v: u32) -> wasmtime::Result<()> {
        self.object_table
            .get_mut(&res.rep())
            .map(|o| o.value = v)
            .ok_or(anyhow!(
                "tried to set a resource `{}` that doesn't exist",
                res.rep()
            ))
    }

    fn get(&mut self, res: Resource<MyObject>) -> wasmtime::Result<u32> {
        self.object_table
            .get(&res.rep())
            .map(|o| o.value)
            .ok_or(anyhow!(
                "tried to get a resource `{}` that doesn't exist",
                res.rep()
            ))
    }

    fn drop(&mut self, res: Resource<MyObject>) -> wasmtime::Result<()> {
        self.object_table
            .remove(&res.rep())
            .map(|o| println!("Value at drop {}", o.value))
            .ok_or(anyhow!(
                "tried to drop a resource `{}` that doesn't exist",
                res.rep()
            ))
    }
}

// As guest, when you build the output is a wasm module, not a component in terms of wasmtime
// this allows us to do it in rust however as i'll note below it's preferred to do it through the wasm-tools cli
// and just import with Component::from_file
use wit_component::ComponentEncoder;

fn main() -> Result<()> {
    let mut config = Config::new();
    config.wasm_component_model(true);

    let engine = Engine::new(&config)?;
    let mut store = Store::new(&engine, HostState::default());
    let mut linker = Linker::new(&engine);

    let wasm_module_path = "../guest.wasm";

    let module =
        std::fs::read(wasm_module_path).expect("WASM Module missing, did you build guest-rust?");

    let component = ComponentEncoder::default()
        .module(module.as_slice())?
        .validate(true)
        .encode()?;

    let component = Component::from_binary(&engine, &component)?;

    crate::test::example::my_interface::add_to_linker(&mut linker, |s| s)?;

    let (bindings, instance) = MyWorld::instantiate(&mut store, &component, &linker).unwrap();

    // let result = bindings.
    // instance.get_func(store, name)

    let start = instance.get_typed_func::<(), ()>(&mut store, "_start")?;

    start.call(&mut store, ())?;

    Ok(())
}
