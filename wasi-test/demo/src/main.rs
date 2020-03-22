use std::env;

extern "C" {
    fn it_works() -> i32;
    fn open_file() -> i32;
}

#[no_mangle]
pub fn start(n: i32) -> i32 {
    let args: Vec<String> = env::args().collect();
    println!("args : {:?}", env::args());
    // let _envs: Vec<String> = env::vars().collect();
    // println!("envs : {:?}", env::vars());

    unsafe { open_file() };

    println!("Hello from inside WASI , {}", n.to_string());
    let result = unsafe { it_works() };
    result
}

pub fn main() {}