# Compile rs to wasm
Go to each folder and using rustc. For example
```
cd hello_world
rustc +nightly --target wasm32-wasi main.rs
```

The `pub fn main(){}` is useless but necessary because we did not compile dynlib. so Main is required by rustc. However, main() is never called.