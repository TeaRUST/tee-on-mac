use std::env;
use std::fs;

use serde::{Serialize, Deserialize};
use bincode;

//We create two memory buffers. One for the input value from the runtime host
//Another is for the output value to the runtime host
const IN_WASM_MEMORY_BUFFER_SIZE: u32 = 1024;
static mut IN_WASM_MEMORY_BUFFER: [u8; IN_WASM_MEMORY_BUFFER_SIZE as usize] = [0; IN_WASM_MEMORY_BUFFER_SIZE as usize];
const OUT_WASM_MEMORY_BUFFER_SIZE: u32 = 1024;
static mut OUT_WASM_MEMORY_BUFFER: [u8; OUT_WASM_MEMORY_BUFFER_SIZE as usize] = [0; OUT_WASM_MEMORY_BUFFER_SIZE as usize];

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Point {
    x : u8,
    y : u8,
}
fn store_value_to_out_wasm_memory_buffer(value: u8) 
    -> u32 {
    let point = Point{x:1,y:2};
    let serialzied_size = bincode::serialized_size(&point).unwrap() as u32;
    let serialized_array = bincode::serialize(&point).unwrap();
    unsafe{
        let serialized_array_ptr = serialized_array.as_ptr();
        for i in 0..serialized_array.len(){
            OUT_WASM_MEMORY_BUFFER[i] = *serialized_array_ptr.add(i);
        }
    }
    serialzied_size
}
fn get_in_wasm_memory_buffer_pointer() -> *const u8 {
    let pointer: *const u8;
    unsafe{
        pointer = IN_WASM_MEMORY_BUFFER.as_ptr();
    }
    pointer
}

fn get_out_wasm_memory_buffer_pointer() -> *const u8 {
    let pointer: *const u8;
    unsafe{
        pointer = OUT_WASM_MEMORY_BUFFER.as_ptr();
    }
    pointer
}

fn deserilize_and_print_point(ptr: i32){
    unsafe{
        let point_from_runtime_host: Point = bincode::deserialize(&IN_WASM_MEMORY_BUFFER[0..]).unwrap();
        println!("point from runtime host is {:?}", point_from_runtime_host);

    }
    
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
fn begin_transfer_into_wasm() -> i32{
    get_in_wasm_memory_buffer_pointer() as i32
}

#[no_mangle]
fn end_transfer_into_wasm(size: i32)->i32{
    deserilize_and_print_point(size);
    0
}
#[no_mangle]
fn do_compute()->i32{
    store_value_to_out_wasm_memory_buffer(0) as i32

}

#[no_mangle]
fn transfer_out_from_wasm() -> i32{
    get_out_wasm_memory_buffer_pointer() as i32 
}
