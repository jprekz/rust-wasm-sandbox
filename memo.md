フルスクラッチでwasmしたい

ビルド
```
cargo build --target=wasm32-unknown-unknown --release
cp .\target\wasm32-unknown-unknown\release\rust-mywasm.wasm .\static\index.wasm
```

簡易サーバー
```
cargo run -p server
```

メモ
- WebGLしたい
- "Rust 側で用意したメモリをそっくりそのまま Canvas に挿入する"ことは割と簡単にできそう
  （参考：http://nmi.jp/2018-03-19-WebAssembly-with-Rust)

その他リンク
- "Introduction - Rust and WebAssembly"
  https://rustwasm.github.io/docs/book/introduction.html
- "Rust+Wasm+WebGLはじめました　環境構築=>Hello World =>画面クリアまで（Windows10） – ZeroGram"
  https://zerogram.info/?p=2633
