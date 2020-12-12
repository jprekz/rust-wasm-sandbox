#![cfg(target_arch = "wasm32")]

use wasm_bindgen::prelude::*;

mod main;

#[wasm_bindgen(start)]
pub fn start() {
    main::main();
}
