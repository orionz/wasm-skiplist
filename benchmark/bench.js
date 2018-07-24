
var microtime = require("microtime");

const uuidv4 = require('uuid/v4');

let skip_ref = require("./skip_list_ref");
let skip_async = require("../dist/index");

function pick(items) {
  return items[Math.floor(Math.random()*(items.length + 1))];
}

function bench(key,f) {
  let start = microtime.nowDouble()
  let val = f()
  let end = microtime.nowDouble()
  console.log(key ,"::", end - start);
  return val
}

function indexOf(SkipList) { }
function keyOf(SkipList) { }
function getValue(SkipList) { }
function setValue(SkipList) { }
function insertIndex(SkipList) { }
function removeIndex(SkipList) { }
function inter(SkipList) { }

function fill(SkipList) {
  let s = new SkipList()
  let keys = []
  for (let i = 0; i < 1000; i++) {
    let key = uuidv4();
    let val = uuidv4();
    let i = pick(keys) || null
    keys.push(key)
    s = s.insertAfter(i,key,val)
  }
  s.keys = keys;
  return s
}

skip_async.then((skip) => {
//  console.log("Ref", skip_ref)
//  console.log("Rust", skip)

  let s1 = bench("fill-ref", () => fill(skip_ref.SkipList))
  let s2 = bench("fill-rust", () => fill(skip.SkipList))
})

