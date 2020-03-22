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
  .bind("0.0.0.0:8000")?
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
  error as wasm_error, func, imports, instantiate, Array, Ctx, WasmPtr, Func, Value,
  compile_with, Instance,
};

use wasmer_middleware_common::metering;


pub fn add(wasm_path: String, x: i64, y: i64) -> wasm_error::Result<i64> {
  let wasm_binary = std::fs::read(wasm_path).unwrap();
  let metering_instance = get_instance(&wasm_binary);
  //let rs = metering_instance.call("add", &[Value::I64(x), Value::I64(y)])?;
  let rs = metering_instance.call("main", &[])?;
  let gas = metering::get_points_used(&metering_instance);

  //let n = rs.get(0).unwrap().to_u128() as i64;
  println!(" gas: {}", gas);

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