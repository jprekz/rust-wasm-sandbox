use rust_wasm_sandbox::*;

pub fn main() {
    log(&"Nao Tomori");
    gl_init();
    let mut r = 0.0;

    next_frame(move || {
        r += 0.01;
        if r >= 1.0 {
            r = 0.0;
        }
        gl_color(r, 0.8, 0.9);
    });
}
