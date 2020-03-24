use std::env;
use std::path::PathBuf;
use std::fs;

#[link(wasm_import_module = "env1")]
extern "C" {
    fn it_works() -> i32;
    fn open_file() -> i32;
}

#[no_mangle]
pub fn start(n: i32) -> i32 {
    let args: Vec<String> = env::args().collect();
    println!("args : {:?}", args);


    let env_vars = std::env::vars()
        .map(|(arg, val)| format!("{}={}", arg, val))
        .collect::<Vec<String>>();
    println!("env => {:?}", env_vars);
    let _envs = env::vars();
    for (key, value) in _envs {
        println!("env => {:?} = {:?}", key, value);
    }

    #[cfg(target_os = "wasi")]
    println!("aaaaaaaaaaaa");


    let mut f = fs::OpenOptions::new()
            .create_new(true)
            .read(true)
            .write(true)
            .open("/xxx")
            .unwrap();
    // f.write_all(b"sfdjslfjsldfjsk").unwrap();
    println!("11111111111");


    // let read_dir = std::fs::read_dir("/").unwrap();
    // let mut out = vec![];
    // for entry in read_dir {
    //     out.push(format!("{:?}", entry.unwrap().path()));
    // }
    // out.sort();

    // for p in out {
    //     println!("{}", p);
    // }

    // let mut base_url = PathBuf::from(".");
    // base_url.push("x.txt");

    // let x = std::fs::File::open(&base_url).unwrap();
    // println!("args : {:?}", x);

    unsafe { open_file() };

    println!("Hello from inside WASI , {}", n.to_string());
    let result = unsafe { it_works() };
    result
}

pub fn main() {}