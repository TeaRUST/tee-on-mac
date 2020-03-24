use std::env;
use std::fs;

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
    fs::write(file_path, b"sfdjlsfdls");
    let c = fs::read_to_string(file_path).expect("no file");
    println!("content => {}", c);

    // for p in out {
    //     println!("{}", p);
    // }


    // println!("Hello, world11111!");
}

#[no_mangle]
fn func_1() {}