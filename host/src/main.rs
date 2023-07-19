use anyhow::Result;
use futures::executor::block_on;
use wasmtime::{component::Linker, Config, Engine, Store};
use wasmtime_wasi::preview2::{
    pipe::WritePipe, wasi::command::add_to_linker, Table, WasiCtx, WasiCtxBuilder, WasiView,
};

lazy_static::lazy_static! {
    static ref ENGINE: Engine = {
        let mut config = Config::new();
        config.wasm_backtrace_details(wasmtime::WasmBacktraceDetails::Enable);
        config.wasm_component_model(true);
        config.async_support(true);

        let engine = Engine::new(&config).unwrap();
        engine
    };
}

wasmtime::component::bindgen!({
    path: "../guest/wit",
    world: "host",
    async: true,
});

fn main() {
    let res = block_on(run("./guest_component.wasm", false)).unwrap();
    println!("res={:?}", res);
}
struct Ctx {
    wasi: WasiCtx,
    table: Table,
}
impl WasiView for Ctx {
    fn ctx(&self) -> &WasiCtx {
        &self.wasi
    }
    fn ctx_mut(&mut self) -> &mut WasiCtx {
        &mut self.wasi
    }
    fn table(&self) -> &Table {
        &self.table
    }
    fn table_mut(&mut self) -> &mut Table {
        &mut self.table
    }
}

#[async_trait::async_trait]
impl Host_Imports for Ctx {
    async fn print(&mut self, _msg: String) -> wasmtime::Result<()> {
        Ok(())
    }
}

async fn run(name: &str, inherit_stdio: bool) -> Result<()> {
    let stdout = WritePipe::new_in_memory();
    let stderr = WritePipe::new_in_memory();
    {
        let mut linker = Linker::new(&ENGINE);
        add_to_linker(&mut linker)?;

        // Create our wasi context.
        // Additionally register any preopened directories if we have them.
        let mut builder = WasiCtxBuilder::new();

        if inherit_stdio {
            builder = builder.inherit_stdio();
        } else {
            builder = builder
                .set_stdout(stdout.clone())
                .set_stderr(stderr.clone());
        }
        builder = builder.set_args(&[name, "."]);

        let mut table = Table::new();
        let wasi = builder.build(&mut table)?;

        Host_::add_to_linker(&mut linker, |ctx: &mut Ctx| ctx)?;

        let ctx = Ctx { wasi, table };
        let mut store = Store::new(&ENGINE, ctx);
        let component = wasmtime::component::Component::from_file(&ENGINE, name)?;
        let (h, _inst) = Host_::instantiate_async(&mut store, &component, &linker)
            .await
            .unwrap();
        h.call_run(&mut store).await?;
    };
    let stdout = stdout
        .try_into_inner()
        .expect("sole ref to stdout")
        .into_inner();
    if !stdout.is_empty() {
        println!("guest stdout:\n{}\n===", String::from_utf8_lossy(&stdout));
    }
    let stderr = stderr
        .try_into_inner()
        .expect("sole ref to stderr")
        .into_inner();
    if !stderr.is_empty() {
        println!("guest stderr:\n{}\n===", String::from_utf8_lossy(&stderr));
    }
    Ok(())
}
