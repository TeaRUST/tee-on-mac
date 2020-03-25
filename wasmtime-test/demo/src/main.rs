use std::env;
use std::fs;

use serde::{Serialize, Deserialize};
const WASM_MEMORY_BUFFER_SIZE: usize = 2;
static mut WASM_MEMORY_BUFFER: [u8; WASM_MEMORY_BUFFER_SIZE] = [0; WASM_MEMORY_BUFFER_SIZE];
#[no_mangle]
pub fn store_value_in_wasm_memory_buffer_buffer_index_zero(value: u8) {
    unsafe{
        WASM_MEMORY_BUFFER[0] = value;
    }
}
#[no_mangle]
pub fn get_wasm_memory_buffer_pointer() -> *const u8 {
    let pointer: *const u8;
    unsafe{
        pointer = WASM_MEMORY_BUFFER.as_ptr();
    }
    pointer
}
#[no_mangle]
pub fn read_wasm_memory_buffer_and_return_index_one() -> u8 {
    let value: u8;
    unsafe {
      value = WASM_MEMORY_BUFFER[1];
    }
    value
}

fn main() {

    #[cfg(target_os = "wasi")]
    println!("aaaaaaaaaaaa");

    let args: Vec<String> = env::args().collect();
    println!("args : {:?}", args);

    let env_vars = env::vars()
        .map(|(arg, val)| format!("{}={}", arg, val))
        .collect::<Vec<String>>();
    println!("env => {:?}", env_vars);


    let read_dir = fs::read_dir("root").unwrap();
    let mut out = vec![];
    for entry in read_dir {
        out.push(format!("{:?}", entry.unwrap().path()));
    }
    let file_path = "root/zzz.txt";
    fs::write(file_path, b"sfdjlsfdls").unwrap();
    let c = fs::read_to_string(file_path).expect("no file");
    println!("content => {}", c);

    // for p in out {
    //     println!("{}", p);
    // }


    // println!("Hello, world11111!");
}
#[no_mangle]
fn orig(a:i32)->i32{
    //a + 1
    get_wasm_memory_buffer_pointer() as i32
}
#[no_mangle]
fn add(a:i32)->i32{
    //a + 1
    store_value_in_wasm_memory_buffer_buffer_index_zero(8);
    get_wasm_memory_buffer_pointer() as i32
}
#[no_mangle]
fn func_1() {}
