
use wasmtime::Instance;
use serde::{Serialize, Deserialize};
pub fn reserve_wasm_memory_buffer<T> (obj: &T, instance: &Instance ) -> (i32, i32) where T: Serialize {
    let buffer_size = bincode::serialized_size(obj).unwrap() as i32; 
    let prepare_buffer_func = instance
        .get_export("prepare_buffer")
        .and_then(|e| e.func())
        .unwrap()
        .get1::<i32, i64>().unwrap();
    
    let result = prepare_buffer_func(buffer_size).unwrap();
    split_i64_to_i32(result)
}

pub fn fill_buffer<T> (obj: &T, instance: &Instance, ptr:i32, len:i32) -> Result<(), &'static str> where T: Serialize {
    let mem = instance.get_export("memory").unwrap().memory().unwrap();
    let mem_array: &mut [u8];
    let serialized_array = bincode::serialize(obj).unwrap();
    // if serialized_array.len() != len as usize {
    //     return Err("memory allocated in different size");
    // }
    unsafe{
        mem_array = mem.data_unchecked_mut();
        for i in 0..len {
            mem_array[ptr as usize + i as usize] = serialized_array[i as usize];
        }
    }
    Ok(())
}

pub fn join_i32_to_i64( a:i32, b:i32)->i64 {
    //((a as i64) << 32) | (b as i64)
    (((a as u64) << 32) | (b as u64) & 0x00000000ffffffff) as i64
}

pub fn split_i64_to_i32( r: i64)->(i32,i32){
    ( (((r as u64) & 0xffffffff00000000) >> 32) as i32 , ((r as u64) & 0x00000000ffffffff) as i32)
}
#[cfg(test)]
mod test{
    
    use super::*;
    #[test]
    
    fn test_i64_i32_convertor(){
        let a = (i32::MAX, i32::MAX);
        assert_eq!(a, split_i64_to_i32(join_i32_to_i64(a.0, a.1)));

        let a = (i32::MIN, i32::MAX);
        assert_eq!(a, split_i64_to_i32(join_i32_to_i64(a.0, a.1)));

        let a = (i32::MAX, i32::MIN);
        assert_eq!(a, split_i64_to_i32(join_i32_to_i64(a.0, a.1)));
        let a = (i32::MIN, i32::MIN);
        assert_eq!(a, split_i64_to_i32(join_i32_to_i64(a.0, a.1)));
        let a = (23, -32);
        assert_eq!(a, split_i64_to_i32(join_i32_to_i64(a.0, a.1)));
    }
    
}

pub mod test_struct {
    use serde::{Serialize, Deserialize};
    #[derive(Serialize, Deserialize, PartialEq, Debug)] 
    pub struct Point {
        x: i32,
        y: i32,
    }
}
