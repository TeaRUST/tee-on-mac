use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, PartialEq, Debug)] 
pub struct Point {
    x: i32,
    y: i32,
    name: String
}

extern "C" {
    fn abc() -> i64;
}
// fn store_value_to_out_wasm_memory_buffer<T>(value: &T) 
//     -> u32 where T: Serialize{
    
//     let serialzied_size = bincode::serialized_size(value).unwrap() as u32;
//     let serialized_array = bincode::serialize(value).unwrap();
//     unsafe{
//         let serialized_array_ptr = serialized_array.as_ptr();
//         for i in 0..serialized_array.len(){
//             OUT_WASM_MEMORY_BUFFER[i] = *serialized_array_ptr.add(i);
//         }
//     }
//     serialzied_size
// }

//     fn get_out_wasm_memory_buffer_pointer() -> *const u8 {
//         let pointer: *const u8;
//         unsafe{
//             pointer = OUT_WASM_MEMORY_BUFFER.as_ptr();
//         }
//         pointer
//     }
 
// fn deserilize_and_print_point(ptr: i32, size: i32){
//     unsafe{
//         let point_from_runtime_host: Point = bincode::deserialize(&IN_WASM_MEMORY_BUFFER[ptr as usize ..]).unwrap();
//         println!("input data size is {}", size);
//         println!("point from runtime host is {:?}", point_from_runtime_host);
//     }
// }


// fn _start() {

//     #[cfg(target_os = "wasi")]
//     println!("aaaaaaaaaaaa");
     

//     let args: Vec<String> = env::args().collect();
//     println!("args : {:?}", args);

//     let env_vars = env::vars()
//         .map(|(arg, val)| format!("{}={}", arg, val))
//         .collect::<Vec<String>>();
//     println!("env => {:?}", env_vars);


//     let read_dir = fs::read_dir("root").unwrap();
//     let mut out = vec![];
//     for entry in read_dir {
//         out.push(format!("{:?}", entry.unwrap().path()));
//     }
//     let file_path = "root/zzz.txt";
//     fs::write(file_path, b"sfdjlsfdls").unwrap();
//     let c = fs::read_to_string(file_path).expect("no file");
//     println!("content => {}", c);

//     // for p in out {
//     //     println!("{}", p);
//     // }


//     // println!("Hello, world11111!");
// }

// // #[no_mangle]
// // fn begin_transfer_into_wasm() -> i32{
// //     get_in_wasm_memory_buffer_pointer() as i32
// // }

// // #[no_mangle]
// // fn end_transfer_into_wasm(size: i32)->i32{
// //     deserilize_and_print_point(0, size);
// //     0
// // }
use binio_wasm;
#[no_mangle]
fn prepare_buffer(buffer_size: i32)->i64 {
    binio_wasm::wasm_prepare_buffer(buffer_size)
}

#[no_mangle]
fn do_compute(ptr:i32, buffer_size: i32)->i64{
    let mut point_tuple : (Point, Point) = binio_wasm::wasm_deserialize(ptr, buffer_size);
    println!("point1 is {:?}", point_tuple.0);
    println!("point2 is {:?}", point_tuple.1);

    let price: i64;
    unsafe {
        price = abc();
        println!("price is {}", price);
    }


    1 as i64
}

