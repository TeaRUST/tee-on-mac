use serde::{Deserialize, Serialize};
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
use wasmer_wasi::{
  generate_import_object_for_version,
  state::{self, WasiFile, WasiFsError},
  types,
};
use std::vec::Vec;

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
  
  let module = compile_with(&wasm, &compiler).unwrap();

  // save module to cache
  {
    let mut mm = get_module_cache().try_lock().unwrap();
    println!("save hash : {}", hash);
    mm.insert(hash, module.clone());
  }
  

  save_module(hasher, module.clone());


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

 // get the version of the WASI module in a non-strict way, meaning we're
  // allowed to have extra imports
  let wasi_version = wasmer_wasi::get_wasi_version(&module, false)
      .expect("WASI version detected from Wasm module");

  // WASI imports
  let mut base_imports =
      generate_import_object_for_version(wasi_version, vec![], vec![], vec![], vec![]);
  // env is the default namespace for extern functions
  let custom_imports = imports! {
      "env" => {
        "it_works" => func!(it_works),
      },
  };
  // The WASI imports object contains all required import functions for a WASI module to run.
  // Extend this imports with our custom imports containing "it_works" function so that our custom wasm code may run.
  base_imports.extend(custom_imports);
  let mut instance = module
      .instantiate(&base_imports)
      .expect("failed to instantiate wasm module");
  //let instance = module.instantiate(&import_object).unwrap();
  initialize(instance.context_mut());
  
  instance
}
/// Called by the program when it wants to set itself up
fn initialize(ctx: &mut Ctx) {
  let state = unsafe { state::get_wasi_state(ctx) };
  let wasi_file_inner = LoggingWrapper {
      wasm_module_name: "example module name".to_string(),
  };
  // swap stdout with our new wasifile
  let _old_stdout = state
      .fs
      .swap_file(types::__WASI_STDOUT_FILENO, Box::new(wasi_file_inner))
      .unwrap();
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LoggingWrapper {
    pub wasm_module_name: String,
}

// std io trait boiler plate so we can implement WasiFile
// LoggingWrapper is a write-only type so we just want to immediately
// fail when reading or Seeking
impl std::io::Read for LoggingWrapper {
    fn read(&mut self, _buf: &mut [u8]) -> std::io::Result<usize> {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "can not read from logging wrapper",
        ))
    }
    fn read_to_end(&mut self, _buf: &mut Vec<u8>) -> std::io::Result<usize> {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "can not read from logging wrapper",
        ))
    }
    fn read_to_string(&mut self, _buf: &mut String) -> std::io::Result<usize> {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "can not read from logging wrapper",
        ))
    }
    fn read_exact(&mut self, _buf: &mut [u8]) -> std::io::Result<()> {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "can not read from logging wrapper",
        ))
    }
}
impl std::io::Seek for LoggingWrapper {
    fn seek(&mut self, _pos: std::io::SeekFrom) -> std::io::Result<u64> {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "can not seek logging wrapper",
        ))
    }
}
impl std::io::Write for LoggingWrapper {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let stdout = std::io::stdout();
        let mut out = stdout.lock();
        out.write(b"[")?;
        out.write(self.wasm_module_name.as_bytes())?;
        out.write(b"]: ")?;
        out.write(buf)
    }
    fn flush(&mut self) -> std::io::Result<()> {
        std::io::stdout().flush()
    }
    fn write_all(&mut self, buf: &[u8]) -> std::io::Result<()> {
        let stdout = std::io::stdout();
        let mut out = stdout.lock();
        out.write(b"[")?;
        out.write(self.wasm_module_name.as_bytes())?;
        out.write(b"]: ")?;
        out.write_all(buf)
    }
    fn write_fmt(&mut self, fmt: std::fmt::Arguments) -> std::io::Result<()> {
        let stdout = std::io::stdout();
        let mut out = stdout.lock();
        out.write(b"[")?;
        out.write(self.wasm_module_name.as_bytes())?;
        out.write(b"]: ")?;
        out.write_fmt(fmt)
    }
}

// the WasiFile methods aren't relevant for a write-only Stdout-like implementation
// we must use typetag and serde so that our trait objects can be safely Serialized and Deserialized
#[typetag::serde]
impl WasiFile for LoggingWrapper {
    fn last_accessed(&self) -> u64 {
        0
    }
    fn last_modified(&self) -> u64 {
        0
    }
    fn created_time(&self) -> u64 {
        0
    }
    fn size(&self) -> u64 {
        0
    }
    fn set_len(&mut self, _len: u64) -> Result<(), WasiFsError> {
        Ok(())
    }
    fn unlink(&mut self) -> Result<(), WasiFsError> {
        Ok(())
    }
    fn bytes_available(&self) -> Result<usize, WasiFsError> {
        // return an arbitrary amount
        Ok(1024)
    }
}
fn it_works(_ctx: &mut Ctx) -> i32 {
    println!("Hello from outside WASI");
    5
}