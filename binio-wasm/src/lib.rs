#![feature(shrink_to)]

use bincode;
use serde::{Serialize, Deserialize};
use std::ptr;

static mut buffer: Vec<u8> = Vec::new();//: Vec<u8> = Vec::with_capacity(i32::MAX as usize);

pub fn wasm_prepare_buffer(size: i32) -> i64 {
    unsafe {
        buffer.shrink_to(size as usize);
    
        let len = buffer.capacity() as i32;
        let ptr = buffer.as_ptr() as i32;
        join_i32_to_i64(ptr, len )
    }
}
pub fn wasm_deserialize<'a, T>(offset:i32, len:i32)->T where T: Deserialize<'a> {
    unsafe{
        
        //let buff_pointer = (offset as *const u8);
        //buffer = Vec::from_raw_parts(offset as *mut u8, len as usize, len as usize);
        let result: T = bincode::deserialize(& buffer).unwrap();
        result.into()
    }
    
}
pub fn join_i32_to_i64( a:i32, b:i32)->i64 {
    //((a as i64) << 32) | (b as i64)
    (((a as u64) << 32) | (b as u64) & 0x00000000ffffffff) as i64
}

pub fn split_i64_to_i32( r: i64)->(i32,i32){
    ( (((r as u64) & 0xffffffff00000000) >> 32) as i32 , ((r as u64) & 0x00000000ffffffff) as i32)
}