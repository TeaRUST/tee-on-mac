use std::collections::HashMap;
use mut_static::MutStatic;


use lazy_static::lazy_static;
use std::cell::{RefCell, RefMut};
use std::sync::{Mutex, MutexGuard};

use once_cell::sync::OnceCell;

type WasmHashMap = HashMap<String, wasmer_runtime::Module>;

pub fn get_module_cache() -> &'static Mutex<HashMap<String, wasmer_runtime::Module>> {
  pub static instance: OnceCell<Mutex<HashMap<String, wasmer_runtime::Module>>> = OnceCell::new();
  instance.get_or_init(|| {
    let mut m = HashMap::new();
    Mutex::new(m)
  })
}




#[cfg(test)]
mod tests {
  use super::*;

  fn test_data_sync() -> &'static Mutex<HashMap<i32, String>> {
    static INSTANCE: OnceCell<Mutex<HashMap<i32, String>>> = OnceCell::new();
    INSTANCE.get_or_init(|| {
      let mut m = HashMap::new();
      m.insert(1, "a".to_string());
      Mutex::new(m)
    })
  }
  fn test_data() -> &'static HashMap<i32, String> {
    static INSTANCE: OnceCell<HashMap<i32, String>> = OnceCell::new();
    INSTANCE.get_or_init(|| {
      let mut m = HashMap::new();
      m.insert(1, "a".to_string());
      m
    })
  }

  #[test]
  fn t1(){
    let mut hm = test_data_sync().lock().unwrap();
    hm.insert(22, "b".to_string());
    assert_eq!(hm.get(&22), Some(&"b".to_string()));
  }

  #[test]
  fn t2(){
    let mut hm = test_data().to_owned();
    hm.insert(22, "b".to_string());
    println!("{:?}", hm);
    assert_eq!(hm.get(&22), Some(&"b".to_string()));
  }
}
