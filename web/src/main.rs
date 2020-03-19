extern crate request;



fn main() {
    // old_main();
  request::start().unwrap();

}

// extern crate wasmer_runtime;
// extern crate wasmer_middleware_common;
// extern crate wasmer_runtime_core;
// extern crate wasmer_singlepass_backend;

// use wasmer_runtime::{
//   error as wasm_error, func, imports, instantiate, Array, Ctx, WasmPtr, Func, Value,
//   compile_with, Instance
// };
// use wasmer_runtime_core::{
//   backend::Compiler, 
//   codegen::{MiddlewareChain, StreamingCompiler},
// };
// use wasmer_middleware_common::metering::Metering;

// static WASM: &'static [u8] =
//     include_bytes!("../hello_world.wasm");
//     // include_bytes!("../wasm-sample-app/target/wasm32-unknown-unknown/release/hello.wasm");

// fn get_compiler(limit: u64) -> impl Compiler {
//   use wasmer_singlepass_backend::ModuleCodeGenerator as SinglePassMCG;
//   let c: StreamingCompiler<SinglePassMCG, _, _, _, _> = StreamingCompiler::new(move || {
//     let mut chain = MiddlewareChain::new();
//     chain.push(Metering::new(limit));
//     chain
//   });

//   c
// }

// fn get_instance() -> Instance {
//   let metering_compiler = get_compiler(1000);
//   let wasm_binary = WASM;
//   let metering_module = compile_with(&wasm_binary, &metering_compiler).unwrap();
//   let metering_import_object = imports! {
//     "env" => {
//       "print_str" => func!(print_str),
//     },
//   };

//   let metering_instance = metering_module.instantiate(&metering_import_object).unwrap();

//   metering_instance
// }

// pub fn old_main() -> wasm_error::Result<()> {
//   let metering_instance = get_instance();

//   let mut rs = metering_instance.call("execute", &[])?;

//   let x = wasmer_middleware_common::metering::get_points_used(&metering_instance);
  
//   println!("gas: {:}", x);
//   Ok(())

// }

// pub fn add(x: i64, y: i64) -> wasm_error::Result<i64> {
//   let metering_instance = get_instance();
//   let rs = metering_instance.call("add", &[Value::I64(x), Value::I64(y)])?;

//   let gas = wasmer_middleware_common::metering::get_points_used(&metering_instance);
  
//   let n = rs.get(0).unwrap().to_u128() as i64;

//   println!("wasm result: {} ||| gas: {}", n, gas);
//   Ok(n)
// }


// // function list start

// fn print_str(ctx: &mut Ctx, ptr: WasmPtr<u8, Array>, len: u32) {
  
//   let memory = ctx.memory(0);

//   // Use helper method on `WasmPtr` to read a utf8 string
//   let string = ptr.get_utf8_string(memory, len).unwrap();

//   // Print it!
//   println!("{}", string);
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use actix_web::dev::Service;
//     use actix_web::{http, test, web, App};

//     #[actix_rt::test]
//     async fn test_index() -> Result<(), Error> {
//         let mut app = test::init_service(
//             App::new().service(web::resource("/").route(web::post().to(index))),
//         )
//         .await;

//         let req = test::TestRequest::post()
//             .uri("/")
//             .set_json(&MyObj {
//                 name: "my-name".to_owned(),
//                 number: 43,
//             })
//             .to_request();
//         let resp = app.call(req).await.unwrap();

//         assert_eq!(resp.status(), http::StatusCode::OK);

//         let response_body = match resp.response().body().as_ref() {
//             Some(actix_web::body::Body::Bytes(bytes)) => bytes,
//             _ => panic!("Response error"),
//         };

//         assert_eq!(response_body, r##"{"name":"my-name","number":43}"##);

//         Ok(())
//     }
// }
