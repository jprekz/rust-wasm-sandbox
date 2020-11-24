フルスクラッチでwasmテスト

準備
```
$ rustup target add wasm32-unknown-unknown
$ cargo install --git 'https://github.com/alexcrichton/wasm-gc'
```

ビルド
```
$ cd scratch_test
$ cargo build --target=wasm32-unknown-unknown --release
$ wasm-gc ..\target\wasm32-unknown-unknown\release\scratch_test.wasm -o .\static\index.wasm
```

`wasm-gc` は実質deprecatedなのでアレ
