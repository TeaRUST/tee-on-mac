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
  .bind("0.0.0.0:8000")?
  .run()
  .await
}

async fn upload_wasm(
    mut payload: Multipart
) -> Result<HttpResponse, Error> {
  

  while let Some(item) = payload.next().await {
    let mut field = item?;
    let content_type = field.content_disposition().unwrap();
    let filename = content_type.get_filename().unwrap();
    let path = String::from("./tmp/test.wasm");
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

  println!("upload success");

  let x = add("./tmp/test.wasm", 20, 40).unwrap();
  println!("result : {}", x);
  
  Ok(HttpResponse::Ok().body("success".to_string()))
}


// runtime

extern crate wasmer_runtime;
extern crate wasmer_middleware_common;
extern crate wasmer_runtime_core;
extern crate wasmer_singlepass_backend;

use wasmer_runtime::{
error as wasm_error, func, imports, instantiate, Array, Ctx, WasmPtr, Func, Value,
compile_with, Instance
};
use wasmer_runtime_core::{
backend::Compiler, 
codegen::{MiddlewareChain, StreamingCompiler},
};
use wasmer_middleware_common::metering::Metering;

// static WASM: &'static [u8] =
//   include_bytes!("../hello_world.wasm");
  // include_bytes!("../wasm-sample-app/target/wasm32-unknown-unknown/release/hello.wasm");

fn get_compiler(limit: u64) -> impl Compiler {
  use wasmer_singlepass_backend::ModuleCodeGenerator as SinglePassMCG;
  let c: StreamingCompiler<SinglePassMCG, _, _, _, _> = StreamingCompiler::new(move || {
    let mut chain = MiddlewareChain::new();
    chain.push(Metering::new(limit));
    chain
  });

  c
}

fn get_instance(wasm_path: &str) -> Instance {
  let metering_compiler = get_compiler(1000);
  let path = format!("{}", wasm_path.to_string());
    let wasm_binary: &'static [u8] = include_bytes!("../tmp/test.wasm");
    let metering_module = compile_with(&wasm_binary, &metering_compiler).unwrap();
    let metering_import_object = imports! {
      "env" => {
        "print_str" => func!(print_str),
      },
    };
  
    let metering_instance = metering_module.instantiate(&metering_import_object).unwrap();
  
    metering_instance
  
  
}


pub fn add(wasm_path: &str, x: i64, y: i64) -> wasm_error::Result<i64> {
  let metering_instance = get_instance(wasm_path);
  let rs = metering_instance.call("add", &[Value::I64(x), Value::I64(y)])?;

  let gas = wasmer_middleware_common::metering::get_points_used(&metering_instance);

  let n = rs.get(0).unwrap().to_u128() as i64;

  println!("wasm result: {} ||| gas: {}", n, gas);
  Ok(n)
}


// function list start

fn print_str(ctx: &mut Ctx, ptr: WasmPtr<u8, Array>, len: u32) {

let memory = ctx.memory(0);

// Use helper method on `WasmPtr` to read a utf8 string
let string = ptr.get_utf8_string(memory, len).unwrap();

// Print it!
println!("{}", string);
}

