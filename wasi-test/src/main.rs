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
use wasmer_middleware_common::metering::Metering;
use std::vec::Vec;

use wasmer_wasi::{
    is_wasi_module, generate_import_object, get_wasi_version, 
    generate_import_object_for_version
};

mod logging;

use logging::{LoggingWrapper};
  
  
  
fn get_compiler() -> impl Compiler {
    use wasmer_singlepass_backend::ModuleCodeGenerator as SinglePassMCG;
    let c: StreamingCompiler<SinglePassMCG, _, _, _, _> = StreamingCompiler::new(move || {
      let mut chain = MiddlewareChain::new();
      chain.push(Metering::new(1000));
      chain
    });
  
    c
}
  
fn get_module(wasm: &Vec<u8>) -> Module {
    
    let compiler = get_compiler();
  
    let hasher = WasmHash::generate(&wasm);
    let hash = hasher.encode();

    
    // let module = compile_with(&wasm, &compiler).unwrap();
    let module = compile(&wasm).expect("compile error");
  
  
    let f = is_wasi_module(&module);
    println!("is_wasi_module : {}", f);
  
    module
}
  
  
  
pub fn get_instance(wasm: &Vec<u8>) -> Instance {
    let module = get_module(wasm);

    let wasi_version = get_wasi_version(&module, false).unwrap();
    println!("get_wasi_version : {:?}", wasi_version);

    let mut base_imports = generate_import_object_for_version(wasi_version, vec![], vec![], vec![], vec![]);

    let custom_imports = imports! {
        "env" => {
            "it_works" => func!(it_works),
        },
    };
    
    base_imports.extend(custom_imports);

    let mut instance = module.instantiate(&base_imports).expect("failed to instantiate wasm module");
    initialize(instance.context_mut());
    
    instance
}

fn main() {
    println!("start");
    let wasm_binary = std::fs::read("./demo/target/wasm32-wasi/debug/demo.wasm").unwrap();
    let instance = get_instance(&wasm_binary);

    let entry_point = instance.func::<(i32), i32>("plugin_entrypoint").unwrap();
    let result = entry_point.call(2).expect("failed to execute plugin");
    println!("result: {}", result);
}
  
fn it_works(_ctx: &mut Ctx) -> i32 {
    println!("Hello from outside WASI");
    5
}

fn initialize(ctx: &mut Ctx) {
    let state = unsafe { state::get_wasi_state(ctx) };
    let wasi_file_inner = LoggingWrapper {
        wasm_module_name: " WASM ".to_string(),
    };
    // swap stdout with our new wasifile
    let _old_stdout = state
        .fs
        .swap_file(types::__WASI_STDOUT_FILENO, Box::new(wasi_file_inner))
        .unwrap();
}