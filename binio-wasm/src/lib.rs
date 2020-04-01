
use bincode;
use serde::{Serialize, Deserialize};
static mut BUFFERS : Vec<Vec<u8>> = Vec::new();
pub fn wasm_prepare_buffer(size: i32) -> i64 {
    let buffer : Vec<u8> = Vec::with_capacity(size as usize);
    let ptr = buffer.as_ptr() as i32;
    unsafe{BUFFERS.push(buffer)};
    join_i32_to_i64(ptr, size )
}
pub fn wasm_deserialize<'a, T>(offset:i32, size:i32)->T where T: Deserialize<'a> {
    let slice = unsafe { std::slice::from_raw_parts(offset as *const _, size as usize) };
    let buffer_would_be_dropped = unsafe{BUFFERS.pop()};
    bincode::deserialize(slice).unwrap()
}

pub fn wasm_serialize<'a, T>(value: &T)->i64 where T: Serialize {
    let buffer_size = bincode::serialized_size(value).unwrap() as i32; 
    let (result_ptr, result_len) = split_i64_to_i32(wasm_prepare_buffer(buffer_size));
    println!("in wasm_serialize, result ptr and len {},{}", result_ptr, result_len);
    let serialized_array = bincode::serialize(value).unwrap();
    println!("serialzied_array: {:?}", &serialized_array);
    let mut slice = unsafe { std::slice::from_raw_parts_mut(result_ptr as *mut _, result_len as usize)};
    for i in 0..result_len {
        slice[i as usize] = serialized_array[i as usize];
    }
    println!("slice: {:?}", &slice);
    join_i32_to_i64(result_ptr, result_len)
}
pub fn join_i32_to_i64( a:i32, b:i32)->i64 {
    //((a as i64) << 32) | (b as i64)
    (((a as u64) << 32) | (b as u64) & 0x00000000ffffffff) as i64
}

pub fn split_i64_to_i32( r: i64)->(i32,i32){
    ( (((r as u64) & 0xffffffff00000000) >> 32) as i32 , ((r as u64) & 0x00000000ffffffff) as i32)
}