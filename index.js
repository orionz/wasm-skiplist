
const sk1 = import("./skip_list");

global.now = () => (Date.now() / 1000)

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

module.exports = new Promise(function(resolve, reject) {
  sk1.then((sk) => {

    sk.SkipList.prototype.removeKey = function(k) {
      this._removeKey(k);
      let n = sk.SkipList.__construct(this.ptr)
      this.ptr = 0;
      return n;
    };

    sk.SkipList.prototype.setValue = function(k,v) {
      this._setValue(k,v);
      let n = sk.SkipList.__construct(this.ptr)
      this.ptr = 0;
      return n;
    };

    sk.SkipList.prototype.insertIndex = function(i,k,v) {
      this._insertIndex(i,k,v);
      let n = sk.SkipList.__construct(this.ptr)
      this.ptr = 0;
      return n;
    };

    sk.SkipList.prototype.removeIndex = function(i) {
      this._removeIndex(i);
      let n = sk.SkipList.__construct(this.ptr)
      this.ptr = 0;
      return n;
    };

    sk.SkipList.prototype.insertAfter = function(i,k,v) {
      this._insertAfter(i,k,v);
      let n = sk.SkipList.__construct(this.ptr)
      this.ptr = 0;
      return n;
    };

    sk.SkipList.prototype.iterator = iterator;
    sk.SkipList.prototype[Symbol.iterator] = function() { return this.iterator('values') }

    resolve(sk)
  }).catch((e) => reject(e))
})
