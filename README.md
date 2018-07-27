
## Playing With Rust/Wasm/Js

```
  $ rustup target add wasm32-unknown-unknown --toolchain nightly
  $ cargo install wasm-bindgen-cli 
  $ npm install
  $ make bench
```

## Question

Can you make a drop-in replacement for a javscript class but written in Rust and running
in wasm?

## Discoveries

### api issues
1. wasm import is async
2. js does not have destructors / cant use immutable patterns and rely on the GC

### development issues
1. some basic services not there (rand,stdout,systime)
2. rust is not high velocity

### happy things
1. rust is super fun
2. rust-wasm is really easy to implement
3. getting rust+wasm to pass the existing skip-list tests was pretty simple

## Todo

1. implement same algorithm as skiplist to compare apples to apples
2. try a neon/ffi spike
3. break out code into different files or projects

