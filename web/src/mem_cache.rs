use std::collections::HashMap;
use mut_static::MutStatic;


use lazy_static::lazy_static;
use std::cell::{RefCell, RefMut};
use std::rc::Rc;

lazy_static! {
  pub static ref Cache: MutStatic<HashMap<String, String>> = {
    MutStatic::from(HashMap::new())
    
  };

  static ref ModuleMap: MutStatic<HashMap<String, wasmer_runtime::Module>> = {
    MutStatic::from(HashMap::new())
  };
}


pub fn set_module_map(key: String, module: wasmer_runtime::Module){
  let mut mm = ModuleMap.write().unwrap();
  mm.insert(key, module);
}

// TODO
// pub fn get_obj() -> HashMap<String, wasmer_runtime::Module> {
  
// }

pub fn display_module_map(){
  let mm = ModuleMap.read().unwrap();
  println!("{:#?}", mm.keys());
}