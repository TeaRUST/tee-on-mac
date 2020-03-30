#![feature(shrink_to)]

use bincode;
use serde::{Serialize, Deserialize};

pub fn wasm_prepare_buffer(size: i32) -> i64 {
    let buffer : Vec<u8> = Vec::with_capacity(size as usize);
    let ptr = buffer.as_ptr() as i32;
    join_i32_to_i64(ptr, size )
}
pub fn wasm_deserialize<'a, T>(offset:i32, size:i32)->T where T: Deserialize<'a> {
    let slice = unsafe { std::slice::from_raw_parts(offset as *const u8, size as usize) };
    bincode::deserialize(slice).unwrap()
}
pub fn join_i32_to_i64( a:i32, b:i32)->i64 {
    //((a as i64) << 32) | (b as i64)
    (((a as u64) << 32) | (b as u64) & 0x00000000ffffffff) as i64
}

pub fn split_i64_to_i32( r: i64)->(i32,i32){
    ( (((r as u64) & 0xffffffff00000000) >> 32) as i32 , ((r as u64) & 0x00000000ffffffff) as i32)
}