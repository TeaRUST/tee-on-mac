use wasmer_runtime::{
  error as wasm_error, func, imports, instantiate, Array, Ctx, WasmPtr, Func, Value,
  compile_with, Instance, Module
};
use wasmer_runtime_core::{
  backend::Compiler, 
  codegen::{MiddlewareChain, StreamingCompiler},
};
use wasmer_middleware_common::metering::Metering;
use std::vec::Vec;

// use crate::crypto1::{a};

use crate::crypto::{sha_256};
use crate::mem_cache::{set_module_map};
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

  let hash = sha_256(wasm);
  
  // try to find from cache
  // let mm = get_obj();
  // if(mm.contains_key(&hash)){
  //   println!("{}", "hash");
  //   return mm.get(&hash).unwrap().to_owned();
  // }
  
  let module = compile_with(&wasm, &compiler).unwrap();

  
  // save module to cache
  // let mut mm = ModuleMap.write().unwrap();
  set_module_map(hash, module.clone());
  // mm.insert(hash, module.clone());
  module
}

pub fn get_instance(wasm: &Vec<u8>) -> Instance {
  let module = get_module(wasm);
  
  let import_object = imports! {
    "env" => {
      "print_str" => func!(print_str),
    },
  };

  
  let instance = module.instantiate(&import_object).unwrap();
  
  instance
}
