cargo build --release --target wasm32-unknown-unknown
wasm-snip -o html/one_person_reversi.wasm target/wasm32-unknown-unknown/release/one_person_reversi.wasm
