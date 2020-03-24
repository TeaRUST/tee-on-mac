use anyhow::Result;
use wasmtime::*;
use wasmtime_wasi::{Wasi, WasiCtx, WasiCtxBuilder};
use wasi_common::{preopen_dir};

fn main() -> Result<()> {
    let store = Store::default();
    let module = Module::from_file(&store, "demo/target/wasm32-wasi/debug/demo.wasm")?;
    
    println!("{:?}", std::fs::File::open("."));
    let wcb = {
        WasiCtxBuilder::new()
        .arg("jacky")
        .env("HOME", "DIR")
        .preopened_dir(std::fs::File::open("asset")?, "root")
        .inherit_stdio()
        .build().expect("error here")
    };

    // let wasi = Wasi::new(&store, WasiCtx::new(std::env::args())?);
    let wasi = Wasi::new(&store, wcb);

    let mut imports = Vec::new();
    for import in module.imports() {
        println!("{} - {}", import.module(), import.name());

        if import.module() == "wasi_snapshot_preview1" {
            if let Some(export) = wasi.get_export(import.name()) {
                imports.push(Extern::from(export.clone()));
                continue;
            }
        }
        panic!(
            "couldn't find import for `{}::{}`",
            import.module(),
            import.name()
        );
    }

    // println!("{:?}", &imports[0].global());

    // Instance our module with the imports we've created, then we can run the
    // standard wasi `_start` function.
    let instance = Instance::new(&module, &imports)?;
    let start = instance
        .get_export("_start")
        .and_then(|e| e.func())
        .unwrap();
    let start = start.get0::<()>()?;
    start()?;
    Ok(())
}
