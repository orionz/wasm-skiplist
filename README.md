
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
2. js does not have destructors / cant use immutable patterns

### developer issues
1. some basic services not there (rand,stdout,systime)
3. rust is not high velocity

### happy things
1. rust-wasm is really easy to implement
2. rust is super fun
