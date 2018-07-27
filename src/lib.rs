
#![feature(use_extern_macros, wasm_custom_section, wasm_import_module)]
#![feature(linked_list_extras)] 

//#![allow(dead_code)]

#[macro_use]
extern crate cfg_if;

extern crate uuid;
extern crate time;
extern crate rand;

use rand::Rng;
use std::fmt::Debug;
use std::cmp::Eq;
use std::hash::Hash;
use std::boxed::Box;
use std::vec::Vec;
use std::collections::HashSet;
use std::iter::FromIterator;
use uuid::Uuid;
//use std::iter::FromIterator;
//use std::mem;
//use std::ops::Deref;

#[derive(Clone)]
enum Node<K,V> {
  Leaf(Leaf<K,V>),
  Branch(Branch<K,V>)
}

#[derive(Clone)]
struct TreeMap<K,V> {
  node: Node<K,V>
}

#[derive(Clone)]
struct Branch<K,V> {
  left_keys: HashSet<K>,
  left: Box<Node<K,V>>,
  right_keys: HashSet<K>,
  right: Box<Node<K,V>>,
}

impl<K: Debug + Eq + PartialEq + Clone + Hash, V: Clone> TreeMap<K,V> {
  pub fn new() -> TreeMap<K,V> {
    TreeMap { node: Node::Leaf(Leaf::new(vec![], vec![])) }
  }
}


impl<K: Debug + Eq + PartialEq + Clone + Hash, V: Clone> ListMap<K,V> for TreeMap<K,V> {
  fn index_of(&self, key: &K) -> Option<usize> {
    self.node.index_of(key)
  }

  fn insert(&mut self, index: usize, key: K, val: V) {
    if self.node.full() {
      if let Node::Leaf(mut l) = self.node.clone() {
        let right = l.pop();
        let branch = Node::Branch(Branch::new(l, right));
        self.node = branch;
      }
    };
    self.node.insert(index,key,val)
  }

  fn remove(&mut self, index: usize) -> Option<K> {
    self.node.remove(index)
  }

  fn get_key(&self, index: usize) -> Option<&K> {
    self.node.get_key(index)
  }

  fn get_value(&self, index: usize) -> Option<&V> {
    self.node.get_value(index)
  }

  fn set(&mut self, key: &K, val: V) -> bool {
    self.node.set(key,val)
  }

  fn debug(&self) {
    log("ROOT");
    self.node.debug();
  }
}

impl<K: Debug + Eq + PartialEq + Clone + Hash,V: Clone> Node<K,V> {
  fn full(&self) -> bool {
    match self {
      Node::Leaf(l) => l.full(),
      _ => false
    }
  }
}

impl<K: Debug + Eq + PartialEq + Clone + Hash, V: Clone> ListMap<K,V> for Node<K,V> {

  fn debug(&self) {
    match self {
      Node::Leaf(l) => l.debug(), 
      Node::Branch(b) => b.debug()
    }
  }

  fn index_of(&self, key: &K) -> Option<usize> {
    match self {
      Node::Leaf(l) => l.index_of(key),
      Node::Branch(b) => b.index_of(key),
    }
  }

  fn insert(&mut self, index: usize, key: K, val: V) {
    match self {
      Node::Leaf(l) => l.insert(index,key,val),
      Node::Branch(b) => b.insert(index,key,val),
    }
  }

  fn remove(&mut self, index: usize) -> Option<K> {
    match self {
      Node::Leaf(l) => l.remove(index),
      Node::Branch(b) => b.remove(index),
    }
  }

  fn get_key(&self, index: usize) -> Option<&K> {
    match self {
      Node::Leaf(l) => l.get_key(index),
      Node::Branch(b) => b.get_key(index),
    }
  }

  fn get_value(&self, index: usize) -> Option<&V> {
    match self {
      Node::Leaf(l) => l.get_value(index),
      Node::Branch(b) => b.get_value(index),
    }
  }

  fn set(&mut self, key: &K, val: V) -> bool {
    match self {
      Node::Leaf(l) => l.set(key,val),
      Node::Branch(b) => b.set(key,val),
    }
  }
}

impl<K: Debug + Eq + PartialEq + Clone + Hash,V: Clone> Branch<K,V> {
  pub fn new(left: Leaf<K,V>, right: Leaf<K,V>) -> Branch<K,V> {
//    left.keys.iter().for_each(|k| log(&format!("LEFT KEY {:?}", k)));
//    log(&format!("INPUT {} {}", left.keys.len(), right.keys.len()));
    let x = Branch {
      left_keys: HashSet::from_iter(left.keys.iter().cloned()),
      right_keys: HashSet::from_iter(right.keys.iter().cloned()),
      left: Box::new(Node::Leaf(left)),
      right: Box::new(Node::Leaf(right))
    };
//    log(&format!("OUTPUT ++ {:?}", x.left_keys));
//    log(&format!("OUTPUT {} {}", x.left_keys.len(), x.right_keys.len()));
    x
  }
}

static mut NODE_SIZE : usize = 90;

#[derive(Clone)]
struct Leaf<K,V> {
  keys: Vec<K>,
  vals: Vec<V>,
}

impl<K: Debug + Eq + PartialEq + Clone + Hash,V: Clone> Leaf<K,V> {

  pub fn new(left: Vec<K>, right: Vec<V>) -> Leaf<K,V> {
    Leaf { keys: left, vals: right }
  }

  fn full(&self) -> bool {
    unsafe {
    self.keys.len() >= NODE_SIZE
    }
  }

  fn pop(&mut self) -> Leaf<K,V> {
    let pivot = self.keys.len() / 2;
    let pop_keys = self.keys.split_off(pivot);
    let pop_vals = self.vals.split_off(pivot);
    Leaf::new(pop_keys,pop_vals)
  }
}

trait ListMap<K: Debug + Eq + Hash + PartialEq + Clone,V: Clone> {
  fn index_of(&self, key: &K) -> Option<usize>;
  fn insert(&mut self, index: usize, key: K, val: V);
  fn remove(&mut self, index: usize) -> Option<K>;
  fn get_key(&self, index: usize) -> Option<&K>;
  fn get_value(&self, index: usize) -> Option<&V>;
  fn set(&mut self, key: &K, val: V) -> bool;

  fn debug(&self);

  fn insert_after(&mut self, node: &K, key: K, val: V) {
    match self.index_of(&node) {
      Some(i) => self.insert(i+1, key, val),
      None => self.insert(0, key, val)
    }
  }

  fn get(&self, key: &K) -> Option<&V> {
    match self.index_of(key) {
      Some(index) => self.get_value(index),
      None => None
    }
  }
}

impl<K: Debug + Clone + Eq + Hash + PartialEq,V: Clone> ListMap<K,V> for Branch<K,V> {

  fn debug(&self) {
    log(&format!("BRANCH l:{} r:{}", self.left_keys.len(), self.right_keys.len()));
    self.left.debug();
    self.right.debug();
  }

  fn index_of(&self, key: &K) -> Option<usize> {
    if self.left_keys.contains(key) {
      self.left.index_of(key)
    } else if self.right_keys.contains(key) {
      self.right.index_of(key).map(|n| n + self.left_keys.len())
    } else {
      None
    }
  }

  fn insert(&mut self, index: usize, key: K, val: V) {
    if index < self.left_keys.len() {
      if self.left.full() {
        if let Node::Leaf(mut l) = *self.left.clone() {
          let right = l.pop();
          let branch = Box::new(Node::Branch(Branch::new(l, right)));
          self.left = branch;
        }
      };
      self.left_keys.insert(key.clone());
      self.left.insert(index, key, val);
    } else {
      if self.right.full() {
        if let Node::Leaf(mut l) = *self.right.clone() {
          let r = l.pop();
          let branch = Box::new(Node::Branch(Branch::new(l, r)));
          self.right = branch;
        }
      };
      self.right_keys.insert(key.clone());
      self.right.insert(index - self.left_keys.len(), key, val);
    }
  }

  fn remove(&mut self, index: usize) -> Option<K> {
    if index < self.left_keys.len() {
      self.left.remove(index).and_then(|key| {
        self.left_keys.remove(&key);
        Some(key)
      })
    } else {
      self.right.remove(index - self.left_keys.len()).and_then(|key| {
        self.right_keys.remove(&key);
        Some(key)
      })
    }
  }

  fn get_key(&self, index: usize) -> Option<&K> {
    if index < self.left_keys.len() {
      self.left.get_key(index)
    } else {
      self.right.get_key(index - self.left_keys.len())
    }
  }

  fn get_value(&self, index: usize) -> Option<&V> {
    if index < self.left_keys.len() {
      self.left.get_value(index)
    } else {
      self.right.get_value(index - self.left_keys.len())
    }
  }

  fn set(&mut self, key: &K, val: V) -> bool {
    if self.left_keys.contains(key) {
      self.left.set(key,val)
    } else if self.right_keys.contains(key) {
      self.right.set(key,val)
    } else {
      false
    }
  }
}
    

impl<K: Debug + Clone + Eq + Hash + PartialEq,V: Clone> ListMap<K,V> for Leaf<K,V> {
  fn debug(&self) {
    log(&format!("LEAF {}", self.keys.len()));
  }

  fn index_of(&self, key: &K) -> Option<usize> {
    self.keys.iter().position(|ref k| k == &key )
  }

  fn insert(&mut self, index: usize, key: K, val: V) {
//    log(&format!("LEAF INSERT {:?}",index));
    self.keys.insert(index, key);
    self.vals.insert(index, val);
  }

  fn remove(&mut self, index: usize) -> Option<K> {
    let k = self.keys.get(index).map(|kk| kk.clone());
    if k.is_some() {
      self.keys.remove(index); 
      self.vals.remove(index);
    };
    k
  }

  fn get_key(&self, index: usize) -> Option<&K> {
    self.keys.get(index)
  }

  fn get_value(&self, index: usize) -> Option<&V> {
    self.vals.get(index)
  }

  fn set(&mut self, key: &K, val: V) -> bool {
    self.index_of(key).map(|i| self.vals[i] = val ).is_some()
  }
}

//#[derive(Clone)]
struct IndexedVector<K,V> {
  v: Vec<(K,V)>,
}

impl<K,V> IndexedVector<K,V> {
  fn new() -> IndexedVector<K,V> {
    IndexedVector {
      v: vec![]
    }
  }
}

impl<K: Debug + Eq + Hash + PartialEq + Clone,V: Clone> ListMap<K,V> for IndexedVector<K,V> {

  fn debug(&self) {
    log(&format!("LISTMAP {}", self.v.len()));
  }

  fn index_of(&self, key: &K) -> Option<usize> {
    self.v.iter().position(|&(ref k,ref _v)| k == key )
  }

  fn insert(&mut self, index: usize, key: K, val: V) {
    self.v.insert(index, (key,val))
  }

  fn remove(&mut self, index: usize) -> Option<K> {
    let k = self.get_key(index).map(|ref k| (*k).clone());
    if k.is_some() {
      self.v.remove(index); 
    };
    k
  }

  fn get_key(&self, index: usize) -> Option<&K> {
    match self.v.get(index) {
      Some(&(ref k,ref _v)) => Some(k),
      None => None
    }
  }
  fn get_value(&self, index: usize) -> Option<&V> {
    match self.v.get(index) {
      Some(&(ref _k,ref v)) => Some(v),
      None => None
    }
  }

  fn set(&mut self, key: &K, val: V) -> bool {
    match self.v.iter_mut().find(|&(ref k,ref _v)| k == key) {
      Some(o) => { o.1 = val; true },
      None => false
    }
  }
}


cfg_if! {
    if #[cfg(feature = "wasm")] {
        #[macro_use]
        extern crate wasm_bindgen;
        use wasm_bindgen::prelude::*;

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
            list: Box<ListMap<String,String>>,
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

          #[wasm_bindgen(js_name = _keyOf)]
          pub fn key_of(&self, index: isize) -> Option<String> {
            self.list.get_key(self.clean_index(index)).map(|k| k.clone())
          }

          #[wasm_bindgen(js_name = _valueOf)]
          pub fn value_of(&self, index: isize) -> Option<String> {
            self.list.get_value(self.clean_index(index)).and_then(|v| Some(v.clone()))
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
            self.list.get(&key).map(|k| k.clone())
          }

          #[wasm_bindgen(js_name = _setValue)]
          pub fn set_value(&mut self, key: String, value: String) {
            if ! self.list.set(&key,value) {
              throw("referenced key does not exist");
            };
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

        #[wasm_bindgen]
        pub fn bench(size: usize) {
          bench_local(size);
        }

    } else {
      use std::time::{SystemTime};
      fn now() -> f64 {
          match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
              Ok(n) => (n.as_secs() as f64 + (n.subsec_millis() as f64 / 1000.0)),
              Err(_) => 0.0
          }
      }

      fn log(s: &str) {
        println!("{}",s);
      }

      fn random() -> f64 {
        rand::random()
      }
    }
}

fn measure<F: FnMut()>(label: &str, mut f: F) {
  let start = now();
  f();
  let end = now();
  log(&format!("{} :: {}", label, end - start));
}

fn uuid() -> Uuid {
  let mut bytes : [u8; 16]= [0; 16];
/*
  unsafe {
    let a = mem::transmute::<f64, [u8; 8]>(random());
    let b = mem::transmute::<f64, [u8; 8]>(random());
    for i in 0..8 {
      bytes[i] = a[i];
      bytes[i+8] = b[i];
    }
  }
*/
  for i in 0..16 {
    //let r = random();
    //log(&format!("{:?} {:?}", r, r as u8));
    bytes[i] = (random() * 256.0) as u8;
  }
  //log(&format!("{:?}", bytes));
  Uuid::from_random_bytes(bytes)
}

fn choose<'a, T>(values: &'a [T]) -> Option<&'a T> {
  if values.len() == 0 {
//    log(&format!("NONE"));
    None
  } else {
    let i = (random() * (values.len() as f64)).floor() as usize;
//    log(&format!("CHOOSE {:?}",i));
    Some(&values[i])
  }
}

fn fill(size: usize, s: &mut Box<ListMap<Uuid,Uuid>>, keys: &mut Vec<Uuid>) {
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

fn indexof(size: usize, s: &mut Box<ListMap<Uuid,Uuid>>, keys: &mut Vec<Uuid>) {
  for _ in 0..size {
    match choose(keys) {
      Some(index) => s.index_of(&index),
      None => None
    };
  };
}

fn keyof(size: usize, s: &mut Box<ListMap<Uuid,Uuid>>, keys: &mut Vec<Uuid>) {
  for _ in 0..size {
    let index = (random() * keys.len() as f64).floor() as usize;
    s.get_key(index);
  };
}

fn getvalue(size: usize, s: &mut Box<ListMap<Uuid,Uuid>>, keys: &mut Vec<Uuid>) {
  for _ in 0..size {
    let index = (random() * keys.len() as f64).floor() as usize;
    s.get_value(index);
  };
}

fn setvalue(size: usize, s: &mut Box<ListMap<Uuid,Uuid>>, keys: &mut Vec<Uuid>) {
  for _ in 0..size {
    let val = uuid();
    match choose(keys) {
      Some(key) => s.set(&key,val),
      None => false
    };
  };
}

fn remove(size: usize, s: &mut Box<ListMap<Uuid,Uuid>>, keys: &mut Vec<Uuid>) {
  for _ in 0..(size - 10) {
    let index = (random() * keys.len() as f64).floor() as usize;
    s.remove(index);
    keys.remove(index);
  };
}


pub fn bench_local(size: usize) {
/*
  let mut keys1 = vec![];
  let mut s1 : Box<ListMap<Uuid,Uuid>> = Box::new(IndexedVector::new());

  measure("fill-native-vec",|| fill(size, &mut s1,&mut keys1));
  measure("indexof-native-vec",|| indexof(size, &mut s1,&mut keys1));
  measure("keyOf-native-vec",|| keyof(size, &mut s1,&mut keys1));
  measure("getValue-native-vec",|| getvalue(size, &mut s1,&mut keys1));
  measure("setValue-native-vec",|| setvalue(size, &mut s1,&mut keys1));
  measure("remove-native-vec",|| remove(size, &mut s1,&mut keys1));
 // s1.debug();

  log("------");
*/

//  for i in vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100 ] {
//    unsafe { NODE_SIZE = i; }
//    log(&format!("i={:?}",i));
    let mut keys2 = vec![];
    let mut s2 : Box<ListMap<Uuid,Uuid>> = Box::new(TreeMap::new());
    measure("fill-native-tree",|| fill(size, &mut s2,&mut keys2));
    measure("indexof-native-tree",|| indexof(size, &mut s2,&mut keys2));
    measure("keyOf-native-tree",|| keyof(size, &mut s2,&mut keys2));
    measure("getValue-native-tree",|| getvalue(size, &mut s2,&mut keys2));
    measure("setValue-native-tree",|| setvalue(size, &mut s2,&mut keys2));
    measure("remove-native-tree",|| remove(size, &mut s2,&mut keys2));
//    s2.debug();
//  }
}

#[cfg(test)]
mod tests {
    #[test]
    fn bench_test() {
      ::bench_local(10000);
    }
}
