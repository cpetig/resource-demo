use anyhow::{anyhow, Result};

use test::example::my_interface::Host;
use wasmtime::{
    self,
    component::{Component, Linker, Resource},
    Config, Engine, Store,
};
use wasmtime_wasi::preview2::{ResourceTable, WasiCtx, WasiView};

use crate::test::example::my_interface::MyObject;

wasmtime::component::bindgen!({
    path: "../wit/simple.wit",
    world: "my-world",
    async: true,
});

struct ObjectImpl {
    value: u32,
}

//derive(Default)]
struct HostState {
    object_table: std::collections::HashMap<u32, ObjectImpl>,
    table: ResourceTable,
    wasi: WasiCtx,
}

impl Default for HostState {
    fn default() -> Self {
        let table = ResourceTable::new();
        let wasi = wasmtime_wasi::preview2::WasiCtxBuilder::new().build();
        Self {
            object_table: Default::default(),
            table,
            wasi,
        }
    }
}

impl Host for HostState {}

#[wasmtime::component::__internal::async_trait]
impl test::example::my_interface::HostMyObject for HostState {
    async fn new(&mut self, a: u32) -> wasmtime::Result<Resource<MyObject>> {
        println!("New {a}");
        let handle = Resource::<MyObject>::new_own(self.object_table.len() as u32);
        self.object_table
            .insert(handle.rep(), ObjectImpl { value: a });
        Ok(handle)
    }

    async fn set(&mut self, res: Resource<MyObject>, v: u32) -> wasmtime::Result<()> {
        self.object_table
            .get_mut(&res.rep())
            .map(|o| o.value = v)
            .ok_or(anyhow!(
                "tried to set a resource `{}` that doesn't exist",
                res.rep()
            ))
    }

    async fn get(&mut self, res: Resource<MyObject>) -> wasmtime::Result<u32> {
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

impl WasiView for HostState {
    fn table(&self) -> &ResourceTable {
        &self.table
    }
    fn table_mut(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
    fn ctx(&self) -> &WasiCtx {
        &self.wasi
    }
    fn ctx_mut(&mut self) -> &mut WasiCtx {
        &mut self.wasi
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut config = Config::new();
    config.wasm_component_model(true).async_support(true);

    let engine = Engine::new(&config)?;
    let mut store = Store::new(&engine, HostState::default());
    let mut linker = Linker::new(&engine);

    let wasm_module_path = "component.wasm";

    let component = Component::from_file(&engine, wasm_module_path)?;

    crate::test::example::my_interface::add_to_linker(&mut linker, |s| s)?;
    wasmtime_wasi::preview2::command::add_to_linker(&mut linker)?;

    let (command, _instance) = wasmtime_wasi::preview2::command::Command::instantiate_async(
        &mut store, &component, &linker,
    )
    .await?;

    command.wasi_cli_run().call_run(&mut store).await?.ok();

    Ok(())
}
