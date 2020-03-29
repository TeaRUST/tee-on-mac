use anyhow::Result;
use wasmtime::*;
use wasmtime_wasi::{Wasi, WasiCtxBuilder};
use binio;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, PartialEq, Debug)] 
pub struct Point {
    x: i32,
    y: i32,
}
fn main() -> Result<()> {
    let store = Store::default();
    let module = Module::from_file(&store, "demo/target/wasm32-wasi/release/demo.wasm")?;
    #[cfg(feature = "verbose")]
    println!("{:?}", std::fs::File::open("."));
    #[cfg(feature = "verbose")]
    _debug_get_module_import_export_list(&module);
    let wcb = {
        WasiCtxBuilder::new()
        .env("HOME", "DIR")
        .preopened_dir(std::fs::File::open("asset")?, "root")
        .inherit_stdio()
        .build().expect("error here")
    };

    let wasi = Wasi::new(&store, wcb);
    let mut imports = Vec::new();
    for import in module.imports() {
        match import.module(){
            "wasi_snapshot_preview1" => {
                if let Some(export) = wasi.get_export(import.name()) {
                    imports.push(Extern::from(export.clone()));
                }
                else{
                    println!(
                        "couldn't find import for `{}::{}`",
                        import.module(),
                        import.name()
                    );
                }
            },
            "env" => {
                match import.name(){
                    "wasm_binio_serilaize" => { 
                        let func_type : FuncType = FuncType::new(
                            Box::new([ValType::I32, ValType::I32]),
                            Box::new([ValType::I32])
                        );
                        let wasm_binio_serilaize_function : Func = Func::wrap(&module.store(), |caller: Caller, ptr: i32, len: i32 | -> i32 {
                            println!("inside");
                            1
                        });
                        imports.push(Extern::from(wasm_binio_serilaize_function));
                    },
                    "wasm_binio_deserialize" => {
                        imports.push(Extern::from(Func::wrap(&module.store(), | caller: Caller, cptr: i32, len:i32|->i32 {0})));
                    },
                    _default => {
                        panic!("Found unresolved import function! {}:{}", import.module(), import.name());
                    },
                }
            },
            _default => {
                
                println!(
                    "couldn't find import for `{}::{}`",
                    import.module(),
                    import.name()
                );
            },
        }
    }

    let point = Point{x:2, y:3};

    let instance = Instance::new(&module, &imports)?;
    let (ptr, buffer_size) = binio::reserve_wasm_memory_buffer(&point, &instance);
    println!("prepare_buffer ptr {} and buffer size {}", ptr, buffer_size);
    binio::fill_buffer(&point, &instance, ptr, buffer_size).expect("error in fillling in buffer {}");
    
    let do_compute = instance
        .get_export("do_compute")
        .and_then(|e| e.func())
        .unwrap()
        .get2::<i32, i32, i32>().unwrap();
    
    let result = prepare_buffer_func(buffer_size).unwrap();
    
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
