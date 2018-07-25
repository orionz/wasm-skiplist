
#![feature(use_extern_macros, wasm_custom_section, wasm_import_module)]
#![feature(linked_list_extras)] 

extern crate uuid;
extern crate time;
extern crate wasm_bindgen;


//use std::vec::Vec;
//use std::collections::LinkedList;
//use std::collections::HashSet;
//use wbg_rand::{Rng, wasm_rng};
use wasm_bindgen::prelude::*;
use uuid::Uuid;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = Math)]
    fn random() -> f64;
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    fn throw(s: &str);
    fn now() -> f64;
}

#[wasm_bindgen]
//#[derive(Clone)]
pub struct SkipList {
    #[wasm_bindgen(readonly)]
    pub length: usize,
    list: IndexedList<String,String>,
}

/*
struct BranchNode<K,V> {
  leftKeys: HashSet<K>,
  left: Box<TreeNode<K,V>>,
  rightKeys: HashSet<K>,
  right: Box<TreeNode<K,V>>,
}

struct LeafNode<K,V> {
  keys: Vec<K>,
  vals: Vec<V>,
}

enum TreeNode<K,V> {
  Branch(BranchNode<K,V>),
  Leaf(LeafNode<K,V>),
}
*/

//#[derive(Clone)]
struct IndexedList<K,V> {
  v: Vec<(K,V)>
}

impl<K: Clone + PartialEq ,V: Clone> IndexedList<K,V> {
  pub fn new() -> IndexedList<K,V> {
    IndexedList {
      v: vec![]
    }
  }

  pub fn index_of(&self, key: &K) -> Option<usize> {
    self.v.iter().position(|&(ref k,ref _v)| k == key )
  }

  pub fn insert(&mut self, index: usize, key: K, val: V) {
    self.v.insert(index, (key,val))
  }

  pub fn insert_after(&mut self, node: &K, key: K, val: V) {
    match self.index_of(&node) {
      Some(i) => self.insert(i+1, key, val),
      None => self.insert(0, key, val)
    }
  }

  pub fn remove(&mut self, index: usize) -> (K,V) {
    self.v.remove(index)
  }

  pub fn get_entry(&self, index: usize) -> Option<&(K,V)> {
    let mut iter = self.v.iter();
    iter.nth(index)
  }

  pub fn set(&mut self, key: K, val: V) -> Option<()> {
    match self.v.iter_mut().find(|&(ref k,ref _v)| *k == key) {
      Some(o) => { *o = (key,val); Some(()) },
      None => None
    }
  }

  pub fn get_value(&self, key: K) -> Option<V> {
    match self.index_of(&key) {
      Some(index) =>
        match self.get_entry(index) {
          Some((_k,v)) => Some(v.clone()),
          None => None
        },
      None => None
    }
  }
}

#[wasm_bindgen]
impl SkipList {

  #[wasm_bindgen(constructor)]
  pub fn new() -> SkipList {
    SkipList { length: 0, list: IndexedList::new() }
  }

  #[wasm_bindgen(js_name = indexOf)]
  pub fn index_of(&self, key: String) -> isize {
    match self.list.index_of(&key) {
      Some(i) => i as isize,
      None => -1 as isize
    }
  }

  #[wasm_bindgen(js_name = _insertAfter)]
  pub fn insert_after(&mut self, after: Option<String>, key: String, val: String) {
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

  // FIXME - these are supposed to return null instead of undefined
  #[wasm_bindgen(js_name = _keyOf)]
  pub fn key_of(&self, index: isize) -> Option<String> {
    self.list.get_entry(self.clean_index(index)).and_then(|(k,_v)| Some(k.clone()))
  }

  #[wasm_bindgen(js_name = _valueOf)]
  pub fn value_of(&self, index: isize) -> Option<String> {
    self.list.get_entry(self.clean_index(index)).and_then(|(_k,v)| Some(v.clone()))
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
  pub fn get_value(&self, key: String) -> Option<String> {
    self.list.get_value(key).clone()
  }

  #[wasm_bindgen(js_name = _setValue)]
  pub fn set_value(&mut self, key: String, value: String) {
    self.list.set(key,value).or_else(|| {
      throw("referenced key does not exist");
      None
    });
  }

  #[wasm_bindgen(js_name = _insertIndex)]
  pub fn insert_index(&mut self, index: usize, key: String, value: String) {
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

//fn pick(v: Vec<Uuid>) -> Option<&Uuid> { }

fn measure<F: FnMut()>(label: &str, mut f: F) {
  let start = now();
  f();
  let end = now();
  log(&format!("{} :: {}", label, end - start));
}

fn uuid() -> Uuid {
  let mut bytes = [0; 16];
  for i in 0..16 {
    bytes[i] = random() as u8;
  }
//  rng.fill_bytes(&mut bytes);
  Uuid::from_random_bytes(bytes)
}

fn choose<'a, T>(values: &'a [T]) -> Option<&'a T> {
  if values.len() == 0 {
    None
  } else {
    let i = (random() * (values.len() as f64)).floor() as usize;
    Some(&values[i])
  }
}

fn fill(size: usize, s: &mut IndexedList<Uuid,Uuid>, keys: &mut Vec<Uuid>) {
  for _ in 0..size {
    let key = uuid();
    let val = uuid();
    match choose(keys) {
      Some(index) => s.insert_after(index,key,val),
      None => s.insert(0,key,val)
    }
    keys.push(key.clone());
  }
}

#[wasm_bindgen]
pub fn bench(size: usize) {
  let mut keys = vec![];
  let mut s = IndexedList::new();
  measure("fill-native",|| fill(size, &mut s,&mut keys));
}

#[cfg(test)]
mod tests {
    #[test]
    fn bench_test() {
      ::bench();
    }
}
