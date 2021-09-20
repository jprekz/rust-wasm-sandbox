#![allow(unused)]

#[cfg(target_arch = "wasm32")]
extern "C" {
    fn js_log(ptr: usize, byte_size: usize);
    fn js_gl_init();
    fn js_gl_color(r: f64, g: f64, b: f64);
    fn js_next_frame();
}

pub fn log(s: &str) {
    #[cfg(target_arch = "wasm32")]
    unsafe {
        js_log(s.as_ptr() as usize, s.len());
    }

    #[cfg(not(target_arch = "wasm32"))]
    println!("{}", s);
}

pub fn gl_init() {
    #[cfg(target_arch = "wasm32")]
    unsafe {
        js_gl_init();
    }
}

pub fn gl_color(r: f64, g: f64, b: f64) {
    #[cfg(target_arch = "wasm32")]
    unsafe {
        js_gl_color(r, g, b);
    }
}

pub fn next_frame(f: impl FnMut() + 'static) {
    #[cfg(target_arch = "wasm32")]
    unsafe {
        CALLBACK = Some(Box::new(f));
        js_next_frame();
    }
}
static mut CALLBACK: Option<Box<dyn FnMut()>> = None;
#[no_mangle]
fn callback() {
    #[cfg(target_arch = "wasm32")]
    unsafe {
        if let Some(c) = &mut CALLBACK {
            c();
            js_next_frame();
        }
    }
}
