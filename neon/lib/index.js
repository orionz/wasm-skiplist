var addon = require('../native');

//let x = new addon.SkipList();

//console.log(addon.hello());
//console.log("x.length",x.length);
//console.log("x.get(length)",x.get("length"));

class SkipList extends addon.SkipList {
  get length() {
      return this.get('length')
  }

  insertAfter(after, key, val) {
    let changed = new SkipList();
    this._insertAfter(changed, after, key, val);
    return changed
  }

  setValue(key, val) {
    let changed = new SkipList();
    this._setValue(changed, key, val);
    return changed
  }

  insertIndex(index, key, val) {
    let changed = new SkipList();
    this._insertIndex(changed, index, key, val);
    return changed
  }

  removeIndex(index) {
    let changed = new SkipList();
    this._removeIndex(changed, index);
    return changed
  }

  removeKey(key) {
    let changed = new SkipList();
    this._removeKey(changed, key);
    return changed
  }
}


//let x = new SkipList();
//console.log("X", x.length)

let iterator = function(mode) {
  let index = 0;
  let key = () => this.keyOf(index)
  let value = () => this.valueOf(index)
  return {
    next () {
      let k = key()
      if (!k) return { value: undefined, done: true };
      let rval = undefined
      switch (mode) {
        case 'keys':    rval = {value: k,            done: false}; break
        case 'values':  rval = {value: value(),      done: false}; break
        case 'entries': rval = {value: [k, value()], done: false}; break
      }
      index++;
      return rval
    },
    [Symbol.iterator]: () => this.iterator(mode),
  }
}

SkipList.prototype.iterator = iterator;
SkipList.prototype[Symbol.iterator] = function() { return this.iterator('values') }



module.exports = { SkipList : SkipList }
