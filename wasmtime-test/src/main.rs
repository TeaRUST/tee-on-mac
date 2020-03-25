use anyhow::Result;
use wasmtime::*;
use wasmtime_wasi::{Wasi, WasiCtxBuilder};
use serde::{Serialize, Deserialize};
use bincode;
//use wasi_common::{preopen_dir};
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Point {
    x : u8,
    y : u8,
}
// impl AsRef<[u8]> for Point{
//     fn as_ref(&self) -> &[u8] {
//         let encoded = bincode::serialize(&self).unwrap();
//             // &encoded.to_owned()
//         &[0,1]
//     }
// } 

fn main() -> Result<()> {
    let store = Store::default();
    let module = Module::from_file(&store, "demo/target/wasm32-wasi/debug/demo.wasm")?;
    //println!("encoded is {:?} ", encoded);
    //let bytes_point = new_point.serialize::<Bytes>(new_point);
    println!("{:?}", std::fs::File::open("."));
    let wcb = {
        WasiCtxBuilder::new()
        .env("HOME", "DIR")
        .preopened_dir(std::fs::File::open("asset")?, "root")
        .inherit_stdio()
        //.inherit_env()
        //.inherit_args()
        .arg(bincode::serialize(&Point{x:1,y:2}).unwrap())
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
    for ex in instance.exports(){
        
        //println!("ex is a {:?}", ex.ty());
        if let Some(func) = ex.func(){
            //println!("exports type is {:?}", ex.ty());
            //println!("ex is a func {:?}", func);
        }
        if let Some(global) = ex.global(){
        }
        if let Some(table) = ex.table(){
        }
        if let Some(mem) = ex.memory(){ 
            unsafe{
                //println!("unsafe data {:?}", mem.data_unchecked());
            }
            println!("ex memory data_ptr, data_size size is {:?}, {:?}, {:?}", mem.data_ptr(), mem.data_size(), mem.size());
        }
    }
    let start = instance
        .get_export("orig")
        .and_then(|e| e.func())
        .unwrap();
    let orig = start.get1::<i32,i32>()?;
    let r = orig(1)? as usize;
    println!("output of orig: {}", r);
    for ex in instance.exports(){
        if let Some(mem) = ex.memory(){
            unsafe{
                let mem_array: &[u8] = mem.data_unchecked();
                println!("first 3 cell in u8 array are: {}, {}, {}", mem_array[r], mem_array[r + 1], mem_array[r + 2]);
            }
            println!("ex memory data_ptr, data_size size is {:?}, {:?}, {:?}", mem.data_ptr(), mem.data_size(), mem.size());
        }
    }
    let start = instance
        .get_export("add")
        .and_then(|e| e.func())
        .unwrap();
    let add = start.get1::<i32,i32>()?;
    let r = add(1)? as usize;
    println!("output of add: {}", r);
    for ex in instance.exports(){
        if let Some(mem) = ex.memory(){
            unsafe{
                let mem_array: &[u8] = mem.data_unchecked();
                println!("first 3 cell in u8 array are: {}, {}, {}", mem_array[r], mem_array[r + 1], mem_array[r + 2]);
            }
            println!("ex memory data_ptr, data_size size is {:?}, {:?}, {:?}", mem.data_ptr(), mem.data_size(), mem.size());
        }
    }
    Ok(())
}
