extern "C" {
    fn it_works() -> i32;
}

#[no_mangle]
pub fn start(n: i32) -> i32 {
    println!("Hello from inside WASI");
    let result = unsafe { it_works() };
    result + n
}

pub fn main() {}