cargo build --release --target wasm32-unknown-unknown 
wasm-bindgen --out-dir ./out/ --target web ./target/wasm32-unknown-unknown/release/qzchess.wasm 
# python -m http.server 8000