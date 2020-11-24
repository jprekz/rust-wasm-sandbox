準備(Optional)
```
cargo install --git 'https://github.com/alexcrichton/wasm-gc'
```

ビルド
```
cargo build --target=wasm32-unknown-unknown --release
wasm-gc .\target\wasm32-unknown-unknown\release\rust-wasm-sandbox.wasm -o .\static\index.wasm
```
