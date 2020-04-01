
use wasmtime::Instance;
use serde::{Serialize, Deserialize};

fn reserve_wasm_memory_buffer<T> (obj: &T, instance: &Instance ) -> (i32, i32) where T: Serialize {
    let buffer_size = bincode::serialized_size(obj).unwrap() as i32; 
    let prepare_buffer_func = instance
        .get_export("prepare_buffer")
        .and_then(|e| e.func())
        .unwrap()
        .get1::<i32, i64>().unwrap();
    
    let result = prepare_buffer_func(buffer_size).unwrap();
    split_i64_to_i32(result)
}

fn fill_buffer<T> (obj: &T, instance: &Instance, ptr:i32, len:i32) -> Result<(), &'static str> where T: Serialize {
    let mem = instance.get_export("memory").unwrap().memory().unwrap();
    let mem_array: &mut [u8];
    let serialized_array = bincode::serialize(obj).unwrap();
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

pub fn call_stub <'a, T, R> (instance: &'a Instance, arg: &T, func_name: &str) -> R 
    where T: Serialize, R: Deserialize<'a> {
    
    let (arg_buffer_ptr, arg_buffer_len) = reserve_wasm_memory_buffer(arg, instance);
    fill_buffer(&arg, instance, arg_buffer_ptr, arg_buffer_len).unwrap();
    let do_compute = instance
        .get_export(func_name)
        .and_then(|e| e.func())
        .unwrap()
        .get2::<i32, i32, i64>().unwrap();

    let result_in_i64 = do_compute(arg_buffer_ptr, arg_buffer_len).expect("do_compute error"); //TODO, handle error
    let (result_buffer_ptr, result_buffer_len) = split_i64_to_i32(result_in_i64);
    let mem = instance.get_export("memory").unwrap().memory().expect("memory fail");
    let mem_array_ref = unsafe {mem.data_unchecked()};

    let start_ptr = mem_array_ref.as_ptr().wrapping_offset(result_buffer_ptr as isize);
    let a = unsafe{std::slice::from_raw_parts(start_ptr, result_buffer_len as usize)};
    bincode::deserialize(a).expect("deseralize error")
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
