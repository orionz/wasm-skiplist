./dist/index.js: src/lib.rs ./index.js Cargo.toml
	npm run build-debug && webpack

native: ./dist/index.js
	Cargo test -- --nocapture

test: ./dist/index.js
	mocha

bench: ./dist/index.js
	node benchmark/bench.js
