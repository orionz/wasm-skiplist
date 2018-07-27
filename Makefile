./dist/index.js: src/lib.rs ./index.js Cargo.toml src/wasm.rs src/native.rs
	npm run build-debug && webpack

native:
	Cargo test -- --nocapture

test: ./dist/index.js
	mocha

bench: ./dist/index.js
	node benchmark/bench.js
