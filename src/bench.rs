
use std::boxed::Box;
use std::vec::Vec;
use uuid::Uuid;

use ::{IndexedVector,TreeMap,ListMap,log,random,now};

fn measure<F: FnMut()>(label: &str, mut f: F) {
  let start = now();
  f();
  let end = now();
  log(&format!("{} :: {}", label, end - start));
}

fn uuid() -> Uuid {
  let mut bytes : [u8; 16]= [0; 16];
  for i in 0..16 {
    bytes[i] = (random() * 256.0) as u8;
  }
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


pub fn bench_vec_impl(size: usize) {
  let mut keys1 = vec![];
  let mut s1 : Box<ListMap<Uuid,Uuid>> = Box::new(IndexedVector::new());

  measure("fill-native-vec",|| fill(size, &mut s1,&mut keys1));
  measure("indexof-native-vec",|| indexof(size, &mut s1,&mut keys1));
  measure("keyOf-native-vec",|| keyof(size, &mut s1,&mut keys1));
  measure("getValue-native-vec",|| getvalue(size, &mut s1,&mut keys1));
  measure("setValue-native-vec",|| setvalue(size, &mut s1,&mut keys1));
  measure("remove-native-vec",|| remove(size, &mut s1,&mut keys1));
}

pub fn bench_tree_impl(size: usize) {
    let mut keys2 = vec![];
    let mut s2 : Box<ListMap<Uuid,Uuid>> = Box::new(TreeMap::new());
    measure("fill-native-tree",|| fill(size, &mut s2,&mut keys2));
    measure("indexof-native-tree",|| indexof(size, &mut s2,&mut keys2));
    measure("keyOf-native-tree",|| keyof(size, &mut s2,&mut keys2));
    measure("getValue-native-tree",|| getvalue(size, &mut s2,&mut keys2));
    measure("setValue-native-tree",|| setvalue(size, &mut s2,&mut keys2));
    measure("remove-native-tree",|| remove(size, &mut s2,&mut keys2));
}

