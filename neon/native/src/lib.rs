#[macro_use]
extern crate neon;
extern crate skip_list;

use skip_list::{TreeMap,ListMap};

use neon::prelude::*;

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("hello node"))
}

pub struct Skip {
  data: TreeMap<String,String>
}

declare_types! {
  pub class MyClass for Skip {
    init(mut _cx) {
      Ok(Skip {
        data: TreeMap::new()
      })
    }

/*
    fn index_of(&self, key: &K) -> Option<usize>;
    fn insert(&mut self, index: usize, key: K, val: V);
    fn remove(&mut self, index: usize) -> Option<K>;
    fn get_key(&self, index: usize) -> Option<&K>;
    fn get_value(&self, index: usize) -> Option<&V>;
    fn set(&mut self, key: &K, val: V) -> bool;
    fn insert_after(&mut self, node: &K, key: K, val: V)
    fn get(&self, key: &K) -> Option<&V>
*/

    // string -> int
    method indexOf(mut cx) {
      let key: Handle<JsString> = cx.argument::<JsString>(0)?;
      let i = {
        let this = cx.this();
        let guard = cx.lock();
        let skip = this.borrow(&guard);
        skip.data.index_of(&key.value()).map(|n| n as f64).unwrap_or(-1.0)
      };
      Ok(cx.number(i).upcast())
    }

    method _insertAfter(mut cx) {
      let mut copy = cx.argument::<MyClass>(0)?;
      let after = cx.argument::<JsValue>(1)?;
      let key: Handle<JsString> = cx.argument::<JsString>(2)?;
      let val: Handle<JsString> = cx.argument::<JsString>(3)?;

/*
      let args: Vec<Handle<JsValue>> = vec![];
      let n = MyClass::new(&mut cx, args);

      // vs
  
      let args: Vec<Handle<JsValue>> = vec![];
      let mut copy : Handle<MyClass> = MyClass::constructor(&mut cx)?.construct(&mut cx, args)?;
*/

      {
        let this = cx.this();
        let guard1 = cx.lock();
        let guard2 = cx.lock();
        let oldskip = this.borrow(&guard1);
        let mut newskip = copy.borrow_mut(&guard2);
        let data = oldskip.data.clone();

        newskip.data = data;

        if let Ok(s) = after.downcast::<JsString>() {
          newskip.data.insert_after(&s.value(),key.value(),val.value());
        } else {
          newskip.data.insert(0,key.value(),val.value());
        }
      };

      Ok(copy.upcast())
    }


    // string -> ()
    method _removeKey(mut cx) {
      let mut copy = cx.argument::<MyClass>(0)?;
      let key: Handle<JsString> = cx.argument::<JsString>(1)?;

      {
        let this = cx.this();

        let guard1 = cx.lock();
        let guard2 = cx.lock();

        let oldskip = this.borrow(&guard1);
        let mut newskip = copy.borrow_mut(&guard2);

        let i = oldskip.data.index_of(&key.value()).unwrap(); // throw if no key
        newskip.data = oldskip.data.clone();
        newskip.data.remove(i);
      }

      Ok(copy.upcast())
    }

    // int -> string
    method keyOf(mut cx) {
      let index_handle: Handle<JsNumber> = cx.argument::<JsNumber>(0)?;
      let result = {
        let index = index_handle.value() as isize;
        let this = cx.this();
        let guard = cx.lock();
        let skip = this.borrow(&guard);
        if index == -1 {
          skip.data.get_key(skip.data.len() - 1).cloned()
        } else {
          skip.data.get_key(index as usize).cloned()
        }
      };
      if let Some(string) = result {
        Ok(cx.string(string).upcast())
      } else {
        Ok(cx.null().upcast())
      }
    }

    // int -> jsval
    method valueOf(mut cx) {
      let index_handle: Handle<JsNumber> = cx.argument::<JsNumber>(0)?;
      let result = {
        let index = index_handle.value() as isize;
        let this = cx.this();
        let guard = cx.lock();
        let skip = this.borrow(&guard);
        if index == -1 {
          skip.data.get_value(skip.data.len() - 1).cloned()
        } else {
          skip.data.get_value(index as usize).cloned()
        }
      };
      if let Some(string) = result {
        Ok(cx.string(string).upcast())
      } else {
        Ok(cx.null().upcast())
      }
    }

    // string -> value
    method getValue(mut cx) {
      let key: Handle<JsString> = cx.argument::<JsString>(0)?;
      let result = {
        let this = cx.this();
        let guard = cx.lock();
        let skip = this.borrow(&guard);
        skip.data.get(&key.value()).cloned()
      };
      if let Some(string) = result {
        Ok(cx.string(string).upcast())
      } else {
        Ok(cx.undefined().upcast())
      }
    }

    // string, value -> ()
    method _setValue(mut cx) {
      let mut copy: Handle<MyClass> = cx.argument::<MyClass>(0)?;
      let key: Handle<JsString> = cx.argument::<JsString>(1)?;
      let val: Handle<JsString> = cx.argument::<JsString>(2)?;

      let data = {
        let this = cx.this();
        let guard = cx.lock();
        let skip = this.borrow(&guard);
        skip.data.clone()
      };

      let ok = {
        let guard = cx.lock();
        let mut skip = copy.borrow_mut(&guard);
        skip.data = data;
        skip.data.set(&key.value(),val.value())
      };

      if ok {
        Ok(copy.upcast())
      } else { 
        cx.throw_error("referenced key does not exist")
      }
    }

    // int, string, value -> ()
    method _insertIndex(mut cx) {
      let mut copy: Handle<MyClass> = cx.argument::<MyClass>(0)?;
      let index  = cx.argument::<JsNumber>(1)?.value() as usize;
      let key: Handle<JsString> = cx.argument::<JsString>(2)?;
      let val: Handle<JsString> = cx.argument::<JsString>(3)?;

      let data = {
        let this = cx.this();
        let guard = cx.lock();
        let skip = this.borrow(&guard);
        skip.data.clone()
      };

//      let mut this = cx.this().clone();

      {
        let guard = cx.lock();
        let mut skip = copy.borrow_mut(&guard);
        skip.data = data;
        skip.data.insert(index,key.value(),val.value());
      }

      Ok(copy.upcast())
    }

    // int -> ()
    method _removeIndex(mut cx) {
      let mut copy: Handle<MyClass> = cx.argument::<MyClass>(0)?;
      let index: Handle<JsNumber> = cx.argument::<JsNumber>(1)?;

      let data = {
        let this = cx.this();
        let guard = cx.lock();
        let skip = this.borrow(&guard);
        skip.data.clone()
      };

//      let mut this = cx.this().clone();

      let ok = {
        let guard = cx.lock();
        let mut skip = copy.borrow_mut(&guard);
        skip.data = data;
        skip.data.remove(index.value() as usize)
      };

      if ok.is_some() {
        Ok(copy.upcast())
      } else { 
        cx.throw_error("key cannot be removed")
      }
    }

    method get(mut cx) {
      let attr: String = cx.argument::<JsString>(0)?.value();

      let this = cx.this();

      match &attr[..] {
        "length" => {
          let length = {
            let guard = cx.lock();
            let user = this.borrow(&guard);
            user.data.len()
          };
          Ok(cx.number(length as f64).upcast())
        },
        _ => cx.throw_type_error("property does not exist")
      }
    }
  }
}

register_module!(mut cx, {
    cx.export_function("hello", hello)?;
    cx.export_class::<MyClass>("SkipList")?;
    Ok(())
});

