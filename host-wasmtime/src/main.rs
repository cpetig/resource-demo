use anyhow::Result;

use test::example::my_interface::{self, Host};
use wasmtime::{
    self,
    component::{Component, Linker, Resource},
    Config, Engine, Store,
};
use wasmtime_wasi::{self, bindings::Command, ResourceTable, WasiCtx, WasiCtxBuilder, WasiView};

wasmtime::component::bindgen!({
    path: "../wit/simple.wit",
    world: "my-world",
    async: true,
    with: {
        "test:example/my-interface/my-object": ObjectImpl,
    }
});

pub struct ObjectImpl {
    value: u32,
}

struct HostState {
    table: ResourceTable,
    wasi: WasiCtx,
}

impl Default for HostState {
    fn default() -> Self {
        let table = ResourceTable::new();
        let wasi = WasiCtxBuilder::new().build();
        Self { table, wasi }
    }
}

impl Host for HostState {}

impl my_interface::HostMyObject for HostState {
    async fn new(&mut self, a: u32) -> Resource<ObjectImpl> {
        println!("New {a}");
        self.table.push(ObjectImpl { value: a }).unwrap()
    }

    async fn set(&mut self, res: Resource<ObjectImpl>, v: u32) {
        self.table.get_mut(&res).map(|o| o.value = v).unwrap()
    }

    async fn get(&mut self, res: Resource<ObjectImpl>) -> u32 {
        self.table.get(&res).map(|o| o.value).unwrap()
    }

    async fn drop(&mut self, res: Resource<ObjectImpl>) -> wasmtime::Result<()> {
        Ok(self
            .table
            .delete(res)
            .map(|o| println!("Value at drop {}", o.value))?)
    }
}

impl WasiView for HostState {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.wasi
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut config = Config::new();
    config.wasm_component_model(true).async_support(true);

    let engine = Engine::new(&config)?;
    let mut store = Store::new(&engine, HostState::default());

    let wasm_module_path = "component.wasm";
    let component = Component::from_file(&engine, wasm_module_path)?;

    let mut linker = Linker::new(&engine);
    my_interface::add_to_linker(&mut linker, |s| s)?;
    wasmtime_wasi::add_to_linker_async(&mut linker)?;

    let command = Command::instantiate_async(&mut store, &component, &linker).await?;

    command.wasi_cli_run().call_run(&mut store).await?.ok();

    Ok(())
}
