use wasmer_runtime::{
    error as wasm_error, func, imports, instantiate, Array, Ctx, WasmPtr, Func, Value,
    compile_with, Instance, Module, 
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

  use wasmer_emscripten::{
    is_emscripten_module, EmscriptenGlobals, generate_emscripten_env, run_emscripten_instance
  };
  
  // use crate::crypto1::{a};
  
  
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
    
    let module = compile_with(&wasm, &compiler).unwrap();
  
    let f = is_emscripten_module(&module);
    println!("is_emscripten_module ======> {:?}", f);
  
    module
  }
  
  
  pub fn get_instance(wasm: &Vec<u8>) -> Instance {
    let module = get_module(wasm);
    
    let import_object = imports! {
      "env" => {
        
      },
    };
  
    
    let instance = module.instantiate(&import_object).unwrap();
    
    instance
  }

  fn run_em(module: Module) -> Result<(), String>{
    let mut emscripten_globals = EmscriptenGlobals::new(&module).expect("");
    println!("222222222");
    let import_object = generate_emscripten_env(&mut emscripten_globals);
    println!("111111111");
    let mut instance = module
        .instantiate(&import_object)
        .map_err(|e| format!("Can't instantiate emscripten module: {:?}", e))?;
    println!("{:#?}", instance.context());
    run_emscripten_instance(
        &module,
        &mut instance,
        &mut emscripten_globals,
        "./asset/",
        vec![],
        Some("a".to_string()),

        vec![],
    )
    .map_err(|e| format!("{:?}", e)).unwrap();
    
    let rs = instance.call("app.Shutdown", &[]).expect("bbbbbbb");
    println!("====== {:#?}", rs);


    Ok(())
  }
  
  fn main(){
    let wasm_binary = std::fs::read(String::from("./asset/bindings.wasm")).unwrap();
  
    let module = get_module(&wasm_binary);

    run_em(module);




    // let rs = metering_instance.call("add", &[Value::I64(x), Value::I64(y)])?;
  
    // let gas = metering::get_points_used(&metering_instance);
  
    // let n = rs.get(0).unwrap().to_u128() as i64;
    
  }
  
  fn test(){}