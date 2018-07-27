./dist/index.js: src/lib.rs ./index.js Cargo.toml src/wasm.rs src/native.rs src/tree.rs src/vec.rs src/skip.rs
	npm run build-debug && webpack

native:
	Cargo test -q -- --nocapture

test: ./dist/index.js
	mocha

bench: ./dist/index.js
	node benchmark/bench.js
