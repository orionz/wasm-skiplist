
use std::fmt::Debug;
use std::cmp::Eq;
use std::hash::Hash;

pub trait ListMap<K: Debug + Eq + Hash + PartialEq + Clone,V: Clone> {
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

  fn len(&self) -> usize;
}

