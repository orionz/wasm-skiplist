
use std::fmt::Debug;
use std::cmp::Eq;
use std::hash::Hash;
use std::vec::Vec;

use ::{ListMap,log};

pub struct IndexedVector<K,V> {
  v: Vec<(K,V)>,
}

impl<K,V> IndexedVector<K,V> {
  pub fn new() -> IndexedVector<K,V> {
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

  fn len(&self) -> usize {
    self.v.len()
  }
}

