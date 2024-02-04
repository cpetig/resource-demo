use anyhow::Result;

use test::example::my_interface::{self, Host};
use wasmtime::{
    self,
    component::{Component, Linker, Resource},
    Config, Engine, Store,
};
use wasmtime_wasi::preview2::{self, ResourceTable, WasiCtx, WasiCtxBuilder, WasiView};

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
        Self {
            table,
            wasi,
        }
    }
}

impl Host for HostState {}

#[wasmtime::component::__internal::async_trait]
impl my_interface::HostMyObject for HostState {
    async fn new(&mut self, a: u32) -> wasmtime::Result<Resource<ObjectImpl>> {
        println!("New {a}");
        Ok(self.table.push(ObjectImpl { value: a })?)
    }

    async fn set(&mut self, res: Resource<ObjectImpl>, v: u32) -> wasmtime::Result<()> {
        Ok(self.table.get_mut(&res).map(|o| o.value = v)?)
    }

    async fn get(&mut self, res: Resource<ObjectImpl>) -> wasmtime::Result<u32> {
        Ok(self.table.get(&res).map(|o| o.value)?)
    }

    fn drop(&mut self, res: Resource<ObjectImpl>) -> wasmtime::Result<()> {
        Ok(self
            .table
            .delete(res)
            .map(|o| println!("Value at drop {}", o.value))?)
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

    let wasm_module_path = "component.wasm";
    let component = Component::from_file(&engine, wasm_module_path)?;

    let mut linker = Linker::new(&engine);
    my_interface::add_to_linker(&mut linker, |s| s)?;
    preview2::command::add_to_linker(&mut linker)?;

    let (command, _instance) = preview2::command::Command::instantiate_async(
        &mut store, &component, &linker,
    )
    .await?;

    command.wasi_cli_run().call_run(&mut store).await?.ok();

    Ok(())
}
