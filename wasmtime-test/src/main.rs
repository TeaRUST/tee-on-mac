use anyhow::Result;
use wasmtime::*;
use wasmtime_wasi::{Wasi, WasiCtxBuilder};
use serde::{Serialize, Deserialize};
use bincode;
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Point {
    x : u8,
    y : u8,
}

fn main() -> Result<()> {
    let store = Store::default();
    let module = Module::from_file(&store, "demo/target/wasm32-wasi/debug/demo.wasm")?;
    println!("{:?}", std::fs::File::open("."));
    //debug_get_module_import_export_list(&module);
    let wcb = {
        WasiCtxBuilder::new()
        .env("HOME", "DIR")
        .preopened_dir(std::fs::File::open("asset")?, "root")
        .inherit_stdio()
        .arg(bincode::serialize(&Point{x:1,y:2}).unwrap())
        .build().expect("error here")
    };

    let wasi = Wasi::new(&store, wcb);

    let mut imports = Vec::new();
    for import in module.imports() {
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

    // Instance our module with the imports we've created, then we can run the
    // standard wasi `_start` function.
    let instance = Instance::new(&module, &imports)?;
    let start = instance
        .get_export("orig")
        .and_then(|e| e.func())
        .unwrap();
    let orig = start.get1::<i32,i32>()?;
    let r = orig(1)? as usize;
    println!("output of orig: {}", r);
    let mem = instance.get_export("memory").unwrap().memory().unwrap();

    let mut mem_array: &[u8];
    unsafe{
        mem_array = mem.data_unchecked();
    }
    println!("first 3 cell in u8 array are: {}, {}, {}", mem_array[r], mem_array[r + 1], mem_array[r + 2]);
    println!("ex memory data_ptr, data_size size is {:?}, {:?}, {:?}", mem.data_ptr(), mem.data_size(), mem.size());
    let start = instance
        .get_export("add")
        .and_then(|e| e.func())
        .unwrap();
    let add = start.get1::<i32,i32>()?;
    let r = add(1)? as usize;
    println!("output of add: {}", r);
    unsafe{
        mem_array = mem.data_unchecked();
    }
    println!("first 3 cell in u8 array are: {}, {}, {}", mem_array[r], mem_array[r + 1], mem_array[r + 2]);
    println!("ex memory data_ptr, data_size size is {:?}, {:?}, {:?}", mem.data_ptr(), mem.data_size(), mem.size());

    Ok(())
}
fn debug_get_module_import_export_list(module: &Module){

    for import in module.imports(){
        println!("in module importType.name: {:?}", import.name());
    }
    for export in module.exports(){
        println!("in module exportType.name : {:?}", export.name());
    }
}