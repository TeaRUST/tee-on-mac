use wasmer_runtime::{
  error as wasm_error, func, imports, instantiate, Array, Ctx, WasmPtr, Func, Value,
  compile_with, Instance, Module, compile,
  cache::{
    WasmHash, FileSystemCache
  }
};
use wasmer_runtime_core::{
  backend::Compiler, import::ImportObject,
  codegen::{MiddlewareChain, StreamingCompiler},
};
use wasmer_middleware_common::metering::Metering;
use std::vec::Vec;

use wasmer_wasi::{generate_import_object_for_version, {is_wasi_module}};

// use crate::crypto1::{a};

use crate::crypto::{sha_256};
use crate::mem_cache::{get_module_cache};
use crate::wasm_imports::{print_str};


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

  {
    // try to find from cache
    let mm = get_module_cache().try_lock().unwrap();
      
    if(mm.contains_key(&hash)){
      println!("======================, {}", mm.contains_key(&hash));
      return mm.get(&hash).unwrap().to_owned();
    }

  }
  
  //let module = compile_with(&wasm, &compiler).unwrap();
	let module = compile(&wasm).expect("wasm compilation");
  // save module to cache
  {
    let mut mm = get_module_cache().try_lock().unwrap();
    println!("save hash : {}", hash);
    mm.insert(hash, module.clone());
  }
  

  save_module(hasher, module.clone());

  let f = is_wasi_module(&module);
  println!("is_wasi_module : {}", f);

  module
}

fn save_module(hash: WasmHash, module: Module) -> Result<(), wasm_error::CacheError>{
  use wasmer_runtime::cache::Cache;
  let mut fs_cache = unsafe {
    FileSystemCache::new(format!("./tmp/{}", hash.encode()))?
  };

  fs_cache.store(hash, module)?;

  Ok(())
}

pub fn get_instance(wasm: &Vec<u8>) -> Instance {
  let module = get_module(wasm);
  let custom_import_object = imports! {
    "env" => {
			"print_str" => func!(print_str),
			"it_works" => func!(it_works),
			
    },
  };
  let import_object = handle_wasi_import(&module, custom_import_object);

  
  let instance = module.instantiate(&import_object).unwrap();
  
  instance
}
fn it_works(_ctx: &mut Ctx) -> i32 {
	println!("Hello from outside WASI");
	5
}
fn handle_wasi_import(module: &Module, custom_import_object: ImportObject) -> ImportObject{
	if is_wasi_module(&module){
		// get the version of the WASI module in a non-strict way, meaning we're
    // allowed to have extra imports
    let wasi_version = wasmer_wasi::get_wasi_version(&module, false)
        .expect("WASI version detected from Wasm module");

    // WASI imports
    let mut base_imports =
        generate_import_object_for_version(wasi_version, vec![], vec![], vec![], vec![]);
    // The WASI imports object contains all required import functions for a WASI module to run.
    // Extend this imports with our custom imports containing "it_works" function so that our custom wasm code may run.
		base_imports.extend(custom_import_object);
		base_imports
	}
	else{
		custom_import_object
	}
}