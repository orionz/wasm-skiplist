
all: ./dist/index.js ./neon/native/index.node

./dist/index.js: src/lib.rs ./index.js Cargo.toml src/wasm.rs src/native.rs src/tree.rs src/vec.rs src/skip.rs
	npm run build-debug && webpack

./neon/native/index.node: ./neon/native/src/lib.rs
	cd neon; neon build; cd ..
native:
	Cargo test -q -- --nocapture

test: all
	mocha

bench: all
	node benchmark/bench.js
