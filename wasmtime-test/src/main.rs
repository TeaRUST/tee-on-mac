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
    let module = Module::from_file(&store, "demo/target/wasm32-wasi/release/demo.wasm")?;
    #[cfg(feature = "verbose")]
    println!("{:?}", std::fs::File::open("."));
    #[cfg(feature = "verbose")]
    debug_get_module_import_export_list(&module);
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
    let begin_transfer_into_wasm = instance
        .get_export("begin_transfer_into_wasm")
        .and_then(|e| e.func())
        .unwrap()
        .get0::<i32>()?;
    
    let mem = instance.get_export("memory").unwrap().memory().unwrap();
    let mut mem_array: &mut [u8];
    unsafe{
        mem_array = mem.data_unchecked_mut();
    }
    let wasm_in_mem_buffer_offset = begin_transfer_into_wasm().unwrap();
    let point = Point{x:3,y:4};
    let serialzied_size = bincode::serialized_size(&point).unwrap() as u32;
    let serialized_array = bincode::serialize(&point).unwrap();
    unsafe{
        for i in 0..serialzied_size{
          mem_array[wasm_in_mem_buffer_offset as usize + i as usize ] = serialized_array[i as usize];
        }
    }
    
    let end_transfer_into_wasm = instance
    .get_export("end_transfer_into_wasm")
    .and_then(|e| e.func())
    .unwrap()
    .get1::<i32, i32>()?;
    
    end_transfer_into_wasm(serialzied_size as i32);

    let do_compute = instance
        .get_export("do_compute")
        .and_then(|e| e.func())
        .unwrap()
        .get0::<i32>()?;
    do_compute();

    let mem = instance.get_export("memory").unwrap().memory().unwrap();

    let transfer_out = instance
        .get_export("transfer_out_from_wasm")
        .and_then(|e| e.func())
        .unwrap()
        .get0::<i32>()?;
    let r = transfer_out()? as usize;
    println!("output of trasnfer out: {}", r);
    let mut mem_array: &[u8];
    unsafe{
        mem_array = mem.data_unchecked();
    }
    let point_from_wasm: Point = bincode::deserialize(&mem_array[r..]).unwrap();
    println!("point from wasm is {:?}", point_from_wasm);
    Ok(())
}
#[cfg(feature = "verbose")]
fn _debug_get_module_import_export_list(module: &Module){
    for import in module.imports(){
        println!(":wasi_commonin module importType.name: {:?}", import.name());
    }
    for export in module.exports(){
        println!("in module exportType.name : {:?}", export.name());
    }
}