use wasmer_runtime::{
    error as wasm_error, func, imports, instantiate, Array, Ctx, WasmPtr, Func, Value,
    compile_with, Instance, Module, compile,
    cache::{
        WasmHash, FileSystemCache
    }
};
use wasmer_runtime_core::{
    backend::Compiler, 
    codegen::{MiddlewareChain, StreamingCompiler},
};
use wasmer_middleware_common::{
    metering,
    metering::{Metering}
};
use std::vec::Vec;

use wasmer_wasi::{
    is_wasi_module, generate_import_object, get_wasi_version, 
    generate_import_object_for_version,
    state::{self, WasiFile, WasiFsError},
    types
};

use std::path::{PathBuf};

mod logging;

use logging::{LoggingWrapper};
  
  
  
fn get_compiler() -> impl Compiler {
    use wasmer_singlepass_backend::ModuleCodeGenerator as SinglePassMCG;
    let c: StreamingCompiler<SinglePassMCG, _, _, _, _> = StreamingCompiler::new(move || {
      let mut chain = MiddlewareChain::new();
      chain.push(Metering::new(99999));
      chain
    });
  
    c
}
  
fn get_module(wasm: &Vec<u8>) -> Module {
    
    let compiler = get_compiler();
  
    let hasher = WasmHash::generate(&wasm);
    let hash = hasher.encode();

    
    let module = compile_with(&wasm, &compiler).expect("compile error");
    // let module = compile(&wasm).expect("compile error");
  
  
    let f = is_wasi_module(&module);
    println!("is_wasi_module : {}", f);
  
    module
}
  
  
  
pub fn get_instance(wasm: &Vec<u8>) -> Instance {
    let module = get_module(wasm);

    let wasi_version = get_wasi_version(&module, false).unwrap();
    println!("get_wasi_version : {:?}", wasi_version);

    let mut base_imports = generate_import_object_for_version(
        wasi_version, 
        vec!["jacky".as_bytes().to_vec(), b"hello world".to_vec()], 
        vec![b"HOME".to_vec()], 
        vec![
            PathBuf::from("./asset/")
        ], 
        vec![
            ("file".to_string(), PathBuf::from("./asset/"))
        ]
    );

    let custom_imports = imports! {
        "env" => {
            "it_works" => func!(it_works),
            "open_file" => func!(open_file),
        },
    };
    
    base_imports.extend(custom_imports);

    let mut instance = module.instantiate(&base_imports).expect("failed to instantiate wasm module");
    initialize(instance.context_mut());
    
    instance
}

fn main() {
    println!("start");
    let wasm_binary = std::fs::read("./demo/target/wasm32-wasi/release/demo.wasm").unwrap();
    let instance = get_instance(&wasm_binary);

    let entry_point : Func<(i32), i32> = instance.func("start").unwrap();
    let result = entry_point.call(100).expect("failed to execute plugin");

    let gas = metering::get_points_used(&instance);
    println!("wasm result: {} ||| gas: {}", result, gas);
}
  
fn it_works(_ctx: &mut Ctx) -> i32 {
    println!("Hello from outside WASI");
    5
}
fn open_file(_ctx: &mut Ctx) -> i32 {
    let state = unsafe { state::get_wasi_state(_ctx) };
    // println!("state : {:?}", state);

    // let args = String::from_utf8(state.args.as_slice()).unwrap();
    println!("state : {:?}", &"aa");

    1
}


fn initialize(_ctx: &mut Ctx) {
    let state = unsafe { state::get_wasi_state(_ctx) };

    // state.args("ffffff");

    // println!("{:?}, {:?}, {:?}", state.args, state.fs, state.envs);



    
    let wasi_file_inner = LoggingWrapper {
        wasm_module_name: " WASM ".to_string(),
    };
    // swap stdout with our new wasifile
    let _old_stdout = state
        .fs
        .swap_file(types::__WASI_STDOUT_FILENO, Box::new(wasi_file_inner))
        .unwrap();
}