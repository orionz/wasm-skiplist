
use ::{TreeMap,ListMap};

use std::boxed::Box;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = Math)]
    pub fn random() -> f64;
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
    pub fn throw(s: &str);
    pub fn now() -> f64;
}

#[wasm_bindgen]
pub struct SkipList {
    #[wasm_bindgen(readonly)]
    pub length: usize,
    list: Box<ListMap<String,JsValue>>,
}

#[wasm_bindgen]
impl SkipList {

  #[wasm_bindgen(constructor)]
  pub fn new() -> SkipList {
    SkipList { length: 0, list: Box::new( TreeMap::new() ) }
//            SkipList { length: 0, list: Box::new(IndexedVector::new()) }
  }

  #[wasm_bindgen(js_name = indexOf)]
  pub fn index_of(&self, key: String) -> isize {
    match self.list.index_of(&key) {
      Some(i) => i as isize,
      None => -1 as isize
    }
  }

  #[wasm_bindgen(js_name = _insertAfter)]
  pub fn insert_after(&mut self, after: Option<String>, key: String, val: JsValue) {
    match after {
      Some(s) => self.list.insert_after(&s,key,val),
      None => self.list.insert(0,key,val)
    }
    self.length += 1;
  }

  #[wasm_bindgen(js_name = _removeKey)]
  pub fn remove_key(&mut self, key: String) {
    if let Some(i) = self.list.index_of(&key) {
      self.list.remove(i);
      self.length -= 1;
    };
  }

  #[wasm_bindgen(js_name = keyOf)]
  pub fn key_of(&self, index: isize) -> JsValue {
    self.list.get_key(self.clean_index(index)).map(|k| JsValue::from_str(k)).unwrap_or(JsValue::null())
  }

  #[wasm_bindgen(js_name = valueOf)]
  pub fn value_of(&self, index: isize) -> JsValue {
    self.list.get_value(self.clean_index(index)).cloned().unwrap_or(JsValue::null())
  }

  fn clean_index(&self, index: isize) -> usize {
    if index == -1 {
      if self.length == 0 {
        0 as usize
      } else {
        self.length - 1
      }
    } else {
      index as usize
    }
  }

  #[wasm_bindgen(js_name = getValue)]
  pub fn get_value(&self, key: String) -> JsValue {
    self.list.get(&key).cloned().unwrap_or(JsValue::undefined())
  }

  #[wasm_bindgen(js_name = _setValue)]
  pub fn set_value(&mut self, key: String, value: JsValue) {
    if ! self.list.set(&key,value) {
      throw("referenced key does not exist");
    };
  }

  #[wasm_bindgen(js_name = _insertIndex)]
  pub fn insert_index(&mut self, index: usize, key: String, value: JsValue) {
    self.list.insert(index,key,value);
    self.length += 1;
  }

  #[wasm_bindgen(js_name = _removeIndex)]
  pub fn remove_index(&mut self, index: usize) {
    if index >= self.length {
      throw("key cannot be removed");
    }
    self.list.remove(index);
    self.length -= 1;
  }
}

#[wasm_bindgen]
pub fn bench_tree(size: usize) {
  ::bench_tree_impl(size);
}

#[wasm_bindgen]
pub fn bench_vector(size: usize) {
  ::bench_vec_impl(size);
}
