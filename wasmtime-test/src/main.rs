use anyhow::Result;
use wasmtime::*;
use wasmtime_wasi::{Wasi, WasiCtxBuilder, WasiCtx};
use serde::{Serialize, Deserialize};
use bincode;
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Point {
    x : u8,
    y : u8,
}

fn main() -> Result<()> {
    let point = Point{x:3,y:4};
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
    let mut binio = Wasm_binio_serilaize{instance : None}; 
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
                        let wasm_binio_serilaize_function : Func = Func::new(&module.store(), func_type, std::rc::Rc::new(binio));
                        imports.push(Extern::from(wasm_binio_serilaize_function));
                    },
                    "wasm_binio_deserialize" => {
                        imports.push(Extern::from(Func::wrap2(&module.store(), | ptr: i32, len:i32| wasm_binio_deserilaize(ptr, len))));
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
    // let begin_transfer_into_wasm = instance
    //     .get_export("begin_transfer_into_wasm")
    //     .and_then(|e| e.func())
    //     .unwrap()
    //     .get0::<i32>()?;
    
    // let mem = instance.get_export("memory").unwrap().memory().unwrap();
    // let mem_array: &mut [u8];
    // unsafe{
    //     mem_array = mem.data_unchecked_mut();
    // }
    // let wasm_in_mem_buffer_offset = begin_transfer_into_wasm().unwrap();
    // let point = Point{x:3,y:4};
    // let serialzied_size = bincode::serialized_size(&point).unwrap() as u32;
    // let serialized_array = bincode::serialize(&point).unwrap();
    // for i in 0..serialzied_size{
    //     mem_array[wasm_in_mem_buffer_offset as usize + i as usize ] = serialized_array[i as usize];
    // }
    
    // let end_transfer_into_wasm = instance
    // .get_export("end_transfer_into_wasm")
    // .and_then(|e| e.func())
    // .unwrap()
    // .get1::<i32, i32>()?;
    
    // end_transfer_into_wasm(serialzied_size as i32)?;
    binio.instance = Some(&instance);
    let do_compute = instance
        .get_export("do_compute")
        .and_then(|e| e.func())
        .unwrap()
        .get1::<i32, i32>()?;
    let buffer_size =  bincode::serialized_size(&point).unwrap() as i32;

    do_compute(buffer_size).unwrap();

    // let mem_array: &[u8];
    // unsafe{
    //     mem_array = mem.data_unchecked();
    // }
    // let point_from_wasm: Point = bincode::deserialize(&mem_array[r..]).unwrap();
    // println!("point from wasm is {:?}", point_from_wasm);
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

 fn wasm_binio_serilaize<T>(ptr: i32, len: i32, _point: &T) -> i32 where T: Serialize{
    // let mem = instance.get_export("memory").unwrap().memory().unwrap();
    // let mem_array: &mut [u8];
    // unsafe{
    //     mem_array = mem.data_unchecked_mut();
    // }
    // let serialized_array = bincode::serialize(point).unwrap();
    // for i in 0..len{
    //     mem_array[ptr as usize + i as usize ] = serialized_array[i as usize];
    // }
    println!("hi there, I am going to fill the buffer at {} length {}", ptr, len);
    5
 }

 fn wasm_binio_deserilaize(ptr: i32, len: i32) -> i32 {
    println!("hi there, I am going to get struct from the buffer at {} length {}", ptr, len);
    6
 }


 struct Wasm_binio_serilaize<'a>{
    pub instance: Option<&'a Instance>
 }
 impl <'a> Wasm_binio_serilaize<'a>{
     pub fn set_instance(&mut self, instance: &'a Instance){
         self.instance = Some(instance);
     }
 }
 impl <'a> wasmtime::Callable for Wasm_binio_serilaize<'a> {
    fn call(&self, params: &[Val], results: &mut [Val]) -> Result<(), wasmtime::Trap> {
        let mut value1 = params[0].unwrap_i32();
        let mut value2 = params[1].unwrap_i32();
        println!("inside binio_ser, v1 and v2 are {},{}", value1, value2);
        results[0] = (value1 + value2).into();

        Ok(())
    }
}