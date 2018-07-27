
use std::fmt::Debug;
use std::cmp::Eq;
use std::hash::Hash;
use std::boxed::Box;
use std::vec::Vec;
use std::collections::HashSet;
use std::iter::FromIterator;

use ::{ListMap,log};

const NODE_SIZE : usize = 90;

#[derive(Clone)]
enum Node<K,V> {
  Leaf(Leaf<K,V>),
  Branch(Branch<K,V>)
}

#[derive(Clone)]
pub struct TreeMap<K,V> {
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
    Branch {
      left_keys: HashSet::from_iter(left.keys.iter().cloned()),
      right_keys: HashSet::from_iter(right.keys.iter().cloned()),
      left: Box::new(Node::Leaf(left)),
      right: Box::new(Node::Leaf(right))
    }
  }
}

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
    self.keys.len() >= NODE_SIZE
  }

  fn pop(&mut self) -> Leaf<K,V> {
    let pivot = self.keys.len() / 2;
    let pop_keys = self.keys.split_off(pivot);
    let pop_vals = self.vals.split_off(pivot);
    Leaf::new(pop_keys,pop_vals)
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
