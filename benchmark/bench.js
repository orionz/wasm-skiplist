
var microtime = require("microtime");

const uuidv4 = require('uuid/v4');

const SIZE = 10000;

let skip_ref = require("./skip_list_ref");
let skip_async = require("../dist/index");

function pick(items) {
  return items[index(items)];
}

function index(items) {
  return Math.floor(Math.random()*(items.length));
}

function bench(key,f) {
  let start = microtime.nowDouble()
  let val = f()
  let end = microtime.nowDouble()
  console.log(key ,"::", end - start);
  return val
}

function indexOf(s) {
  for (let i = 0; i < SIZE; i++) {
    let key = pick(s.keys)
    s.list.indexOf(key)
  }
}

function keyOf(s) {
  for (let i = 0; i < SIZE; i++) {
    let i = index(s.keys);
    s.list.keyOf(i);
  }
}

function getValue(s) {
  for (let i = 0; i < SIZE; i++) {
    let key = pick(s.keys)
    s.list.getValue(key);
  }
}

function setValue(s) {
  for (let i = 0; i < SIZE; i++) {
    let key = pick(s.keys);
    let val = uuidv4();
    s.list = s.list.setValue(key,val);
  }
  return s
}

function removeIndex(s) {
  for (let i = 0; i < SIZE - 5; i++) {
    let i = index(s.keys);
    s.keys.splice(i,1)
    s.list = s.list.removeIndex(i);
  }
  return s
}

function fill(SkipList) {
  let s = new SkipList()
  let keys = []
  for (let i = 0; i < SIZE; i++) {
    let key = uuidv4();
    let val = uuidv4();
    let i = pick(keys) || null
    keys.push(key)
    s = s.insertAfter(i,key,val)
  }
  return { list: s, keys: keys }
}

skip_async.then((skip) => {

  console.log("------[ js - skiplist ] ------")
  let s1 = bench("fill-ref", () => fill(skip_ref.SkipList))
  bench("indexOf-ref", () => indexOf(s1))
  bench("keyOf-ref", () => keyOf(s1))
  bench("getValue-ref", () => getValue(s1))
  s1 = bench("setValue-ref", () => setValue(s1))
  s1 = bench("removeIndex-ref", () => removeIndex(s1))
  console.log("------[ js -> wasm - tree ] ------")
  let s2 = bench("fill-rust", () => fill(skip.SkipList))
  bench("indexOf-rust", () => indexOf(s2))
  bench("keyOf-rust", () => keyOf(s2))
  bench("getValue-rust", () => getValue(s2))
  s2 = bench("setValue-rust", () => setValue(s2))
  s2 = bench("removeIndex-rust", () => removeIndex(s2))
  console.log("------ [ wasm - tree ] ---------")
  skip.bench_tree(SIZE)
  //console.log("------ [ wasm - vec ] ---------")
  //skip.bench_vector(SIZE)
  console.log("------ [ binary - tree ] -------")
  require('child_process').exec("Cargo test -q -- --nocapture",(err,stdout,stderr) => {
    console.log(stdout)
  })
})

