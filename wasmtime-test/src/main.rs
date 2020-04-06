use anyhow::Result;
use wasmtime::*;
use wasmtime_wasi::{Wasi, WasiCtxBuilder};
use binio;
use serde::{Serialize, Deserialize};

mod tpm_cmd;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Copy)] 
pub struct Point {
    x: i32,
    y: i32,
    name: &'static str
}
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Copy)] 
pub struct React {
    left: i32,
    right: i32,
    top: i32,
    bottom: i32,
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

    let point1 = Point{x:2, y:3, name: "jacky"};
    let point2 = Point{x:8, y:9, name: "kevin"};

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
                    "abc" => {
                        let wasm_binio_serilaize_function : Func = Func::wrap(&module.store(), move || -> i64 {

                            let price = get_btc_price().unwrap();
                            println!(" ==== {:?}, {:?}", price, point1);

                            price

                        });
                        imports.push(Extern::from(wasm_binio_serilaize_function));
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

    

    let instance = Instance::new(&module, &imports)?;
    let result: React= binio::call_stub(&instance, &(point1, point2), "do_compute");
    println!("return React {:?}", result);
    
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

fn get_btc_price() -> Result<i64> {

    Ok(5000)
}

