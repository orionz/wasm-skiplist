
#![feature(use_extern_macros, wasm_custom_section, wasm_import_module)]
#![feature(linked_list_extras)] 

#[macro_use]
extern crate cfg_if;

cfg_if! {
    if #[cfg(feature = "wasm")] {
      #[macro_use]
      extern crate wasm_bindgen;
    }
}

extern crate uuid;
extern crate time;
extern crate rand;

use std::fmt::Debug;
use std::cmp::Eq;
use std::hash::Hash;

pub mod tree;
use tree::{TreeMap};

pub mod vec;
use vec::{IndexedVector};

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

cfg_if! {
    if #[cfg(feature = "wasm")] {
      pub mod wasm;
      use wasm::{log,now,random};
    } else {
      pub mod native;
      use native::{log,now,random};
    }
}

pub mod bench;
use bench::{bench_tree_impl};
cfg_if! {
  if #[cfg(feature = "wasm")] {
    use bench::{bench_vec_impl};
  }
}

pub mod skip;

#[cfg(test)]
mod tests {
    #[test]
    fn bench_test() {
      ::bench_tree_impl(10000);
    }
}
