use actix_web::{
  error, middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer,
};
use bytes::{Bytes, BytesMut};
use futures::StreamExt;
use json::JsonValue;
use serde::{Deserialize, Serialize};

use actix_multipart::Multipart;
use actix_files;

use std::io::Write;
use std::rc::Rc;

use std::collections::HashMap;
use mut_static::MutStatic;

mod crypto;
mod mem_cache;
mod wasm_imports;

use crypto::{sha_256};
use mem_cache::{get_module_cache};

#[macro_use]
extern crate lazy_static;

lazy_static! {
  pub static ref Cache: MutStatic<HashMap<String, String>> = {
    MutStatic::from(HashMap::new())
    
  };

  pub static ref ModuleMap: MutStatic<HashMap<String, wasmer_runtime::Module>> = {
    MutStatic::from(HashMap::new())
  };
}


#[actix_rt::main]
pub async fn main() -> std::io::Result<()> {

  std::env::set_var("RUST_LOG", "actix_web=info");
  env_logger::init();

  HttpServer::new(|| {
    let app = App::new()
        .wrap(middleware::Logger::default())
        .data(web::JsonConfig::default().limit(4096))
        
        .service(
          web::resource("/upload_wasm").route(web::post().to(upload_wasm))
        )
        .service(
          actix_files::Files::new("/html/", "./html/").index_file("index.html"),
        );
    app.default_service(web::route().to(|| HttpResponse::Forbidden()))
  })
  .bind("0.0.0.0:8080")?
  .run()
  .await
}

async fn upload_wasm(
    mut payload: Multipart
) -> Result<HttpResponse, Error> {
  
  while let Some(item) = payload.next().await {
    println!("{:?}", item);
    let mut field = item?;
    
    let content_type = field.content_disposition().unwrap();
    let key = content_type.get_name().unwrap();
    println!("--- {:?}", key);

    if (key == "file") {
      let filename = content_type.get_filename().unwrap();
      let path = String::from(format!("./tmp/{}", filename));

      let mut cache = Cache.write().unwrap();
      cache.insert(key.to_owned(), format!("./tmp/{}", filename));

      // File::create is blocking operation, use threadpool
      let mut f = web::block(|| std::fs::File::create(path))
          .await
          .unwrap();
      // Field in turn is stream of *Bytes* object
      while let Some(chunk) = field.next().await {
          let data = chunk.unwrap();
          // filesystem operations are blocking, we have to use threadpool
          f = web::block(move || f.write_all(&data).map(|_| f)).await?;
      }

      
    }
    else if (key == "x" || key == "y") {

      while let Some(val) = field.next().await {
        let data = val.unwrap();
        let xx = std::str::from_utf8(&data).unwrap().to_owned();

        let mut cache = Cache.write().unwrap();
        cache.insert(key.to_owned(), xx);
      }

      
    }
    

  }

  println!("upload success");


  let cache = Cache.read().unwrap();
  println!("{:?}", cache.values());

  let x = add(
    cache.get("file").unwrap().to_string(),
    cache.get("x").unwrap().parse::<i64>().unwrap(),
    cache.get("y").unwrap().parse::<i64>().unwrap()
  ).unwrap();
  println!("result : {}", x);
  
  Ok(HttpResponse::Ok().body(x.to_string()))
}

async fn save_wasm_file() {

}


// runtime
mod runtime;
use runtime::{get_instance};



use wasmer_runtime::{
  error as wasm_error, func, imports, instantiate, Array, Ctx, WasmPtr, Func, Value,compile,
  compile_with, Instance,
};
use wasmer_wasi::{
  generate_import_object_for_version,
  state::{self, WasiFile, WasiFsError},
  types,
};
use wasmer_middleware_common::metering;
fn it_works(_ctx: &mut Ctx) -> i32 {
  println!("Hello from outside WASI");
  5
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

pub fn add(wasm_path: String, x: i64, y: i64) -> wasm_error::Result<i64> {
  let wasm_bytes = std::fs::read(wasm_path).unwrap();
  let module = compile(&wasm_bytes).expect("wasm compilation");

  // get the version of the WASI module in a non-strict way, meaning we're
  // allowed to have extra imports
  let wasi_version = wasmer_wasi::get_wasi_version(&module, false)
      .expect("WASI version detected from Wasm module");
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
  // set up logging by replacing stdout
  initialize(instance.context_mut());

  // get a reference to the function "plugin_entrypoint" which takes an i32 and returns an i32
  let entry_point = instance.func::<(i32), i32>("plugin_entrypoint").unwrap();
  // call the "entry_point" function in WebAssembly with the number "2" as the i32 argument
  let result = entry_point.call(2).expect("failed to execute plugin");
  println!("result: {}", result);



  // let metering_instance = get_instance(&wasm_binary);
  // let entry_point = metering_instance.func::<(i32), i32>("plugin_entrypoint").unwrap();
  // let rs = entry_point.call(2).expect("failed to execute plugin");
  // let gas = metering::get_points_used(&metering_instance);

  // //let n = rs.get(0).unwrap().to_u128() as i64;
  // println!(" gas: {}", gas);

  Ok(0)
}

fn main1(){
  println!("start");
  let wasm_path = String::from("./tmp/test.wasm");
  add(wasm_path, 1, 20);
  add(String::from("./tmp/hello_world.wasm"), 1, 20);

  let mm = get_module_cache().lock().unwrap();
  println!("{:?}", mm.keys());
}