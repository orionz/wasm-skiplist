
use std::fmt::Debug;
use std::cmp::Eq;
use std::hash::Hash;
use std::vec::Vec;

use ::{ListMap};

pub struct SkipList<K,V> {
  _k: Vec<K>,
  _v: Vec<V>
}

impl<K: Debug + Eq + PartialEq + Clone + Hash, V: Clone> SkipList<K,V> {
  pub fn new() -> SkipList<K,V> {
    SkipList { _k: vec![], _v: vec![] }
  }
}

impl<K: Debug + Eq + PartialEq + Clone + Hash, V: Clone> ListMap<K,V> for SkipList<K,V> {
  fn index_of(&self, _key: &K) -> Option<usize> {
    None
  }

  fn insert(&mut self, _index: usize, _key: K, _val: V) {
  }

  fn remove(&mut self, _index: usize) -> Option<K> {
    None
  }

  fn get_key(&self, _index: usize) -> Option<&K> {
    None
  }

  fn get_value(&self, _index: usize) -> Option<&V> {
    None
  }

  fn set(&mut self, _key: &K, _val: V) -> bool {
    false
  }

  fn debug(&self) {
  }
}

