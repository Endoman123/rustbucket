# Rustbucket
A spiritual successor to Hold my Coffee, written in Rust.

# Building
If you want to run in WASM, you need to add the `wasm32-unknown-unknown` target.
```
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-name rustbucket --out-dir target/wasm --target web target/wasm32-unknown-unknown/build/rustbucket.wasm
```
