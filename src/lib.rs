
#![feature(use_extern_macros, wasm_custom_section, wasm_import_module)]
#![feature(linked_list_extras)] 

#[macro_use]
extern crate cfg_if;

cfg_if! {
    if #[cfg(feature = "wasm")] {
      #[macro_use]
      extern crate wasm_bindgen;
    }
}

extern crate uuid;
extern crate time;
extern crate rand;

pub mod tree;
pub use tree::{TreeMap};

pub mod vec;
pub use vec::{IndexedVector};

pub mod listmap;
use listmap::{ListMap};

cfg_if! {
    if #[cfg(feature = "wasm")] {
      pub mod wasm;
      use wasm::{log,now,random};
    } else {
      pub mod native;
      use native::{log,now,random};
    }
}

pub mod bench;
pub use bench::{bench_tree_impl};
cfg_if! {
  if #[cfg(feature = "wasm")] {
    pub use bench::{bench_vec_impl};
  }
}

pub mod skip;
pub use skip::{SkipList};

#[cfg(test)]
mod tests {
    #[test]
    fn bench_test() {
      ::bench_tree_impl(10000);
    }
}
