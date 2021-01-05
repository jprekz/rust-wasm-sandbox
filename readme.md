## ビルド
各ディレクトリ内で
- nativeの場合 `$ cargo run`
- webの場合 `$ wasm-pack build --target web` のち適当なwebサーバーを立てて開く

## メモ
- WebGLしたい
- とりあえず依存無しでWebGLの呼び出しを試みた（scratch_test）
  - 参考：https://zerogram.info/?p=2633
- "Rust 側で用意したメモリをそっくりそのまま Canvas に挿入する"ことは割と簡単にできそう
  （参考：http://nmi.jp/2018-03-19-WebAssembly-with-Rust)
  - ネイティブで同じことできそうなやつ：https://github.com/rust-windowing/winit-blit
  - ハードウェアアクセラレーションなしでクロスプラットフォームなアプリ書くなら [`winit`] と
  上記手法の組み合わせが最小構成かも
- [`glow`] でクロスプラットフォームなウィンドウ初期化と描画テスト（glow-test）
  - webの場合は [`winit`] で初期化とイベント管理ができる。GLコンテキストは
  `web_sys::WebGl2RenderingContext` を直接 [`glow`] に渡すことができる
  - nativeの場合は [`glutin`] で初期化とイベント管理を行う。 [`glutin`] はGLコンテキストを
  取得するライブラリで `wasm32-unknown-unknown` では動作しない。 [`glutin`] は [`winit`] を
  `pub use` しているので中身は同じなのだが……
- [`winit`] 等のラッパー [`blinds`] と [`glow`] のラッパー [`golem`] のテスト（golem_test）
  - 最近いろいろ変わってるっぽくてなんとも
- [`glutin`] / [`winit`] と [`golem`] を [`nannou`] 風のAPIで雑にラップしてみたテスト（wrapper_test）
  - wrapper_test はライブラリで wrapper_test_bin は wrapper_test を使ってみたやつ
  - wrapper_test_bin のソースはぱっと見いい感じではある
- glTF形式の3Dモデルを雑に表示してみるテスト（gltf_test）
  - web環境でファイルをfetchしようとすると非同期処理になるのでinit関数をasyncにしたい
    - やはり [`blinds`] 使えばいいのでは？
    - 現状の実装ではファイルを `include_bytes!` してる

[`winit`]: https://github.com/rust-windowing/winit
[`glow`]: https://github.com/grovesNL/glow
[`glutin`]: https://github.com/rust-windowing/glutin
[`blinds`]: https://github.com/ryanisaacg/blinds
[`golem`]: https://github.com/ryanisaacg/golem
[`nannou`]: https://github.com/nannou-org/nannou

## その他リンク
- "Introduction - Rust and WebAssembly"
  https://rustwasm.github.io/docs/book/introduction.html
- "rustwasm/wasm-pack: 📦✨ your favorite rust -> wasm workflow tool!"
  https://github.com/rustwasm/wasm-pack
