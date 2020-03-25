use std::env;
use std::fs;

use serde::{Serialize, Deserialize};
use bincode;
const WASM_MEMORY_BUFFER_SIZE2: u32 = 1024;
static mut WASM_MEMORY_BUFFER2: [u8; WASM_MEMORY_BUFFER_SIZE2 as usize] = [0; WASM_MEMORY_BUFFER_SIZE2 as usize];
const WASM_MEMORY_BUFFER_SIZE: u32 = 1024;
static mut WASM_MEMORY_BUFFER: [u8; WASM_MEMORY_BUFFER_SIZE as usize] = [0; WASM_MEMORY_BUFFER_SIZE as usize];
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Point {
    x : u8,
    y : u8,
}
fn store_value_in_wasm_memory_buffer_buffer_index_zero(value: u8) {
    let serialized_array = bincode::serialize(&Point{x:1,y:2}).unwrap();
    unsafe{
        let serialized_array_ptr = serialized_array.as_ptr();
        for i in 0..serialized_array.len(){
            WASM_MEMORY_BUFFER[i] = *serialized_array_ptr.add(i);
        }
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
pub fn get_wasm_memory_buffer_size() -> u32 {
    WASM_MEMORY_BUFFER_SIZE 
}

fn read_wasm_memory_buffer_and_return_index_one() -> u8 {
    let value: u8;
    unsafe {
      value = WASM_MEMORY_BUFFER[1];
    }
    value
}
 
fn deserilize_and_print_point(ptr: i32){

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
    store_value_in_wasm_memory_buffer_buffer_index_zero(0);
    get_wasm_memory_buffer_pointer() as i32
}
#[no_mangle]
fn func_1() {}
