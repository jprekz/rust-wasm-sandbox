#[cfg(target_arch = "wasm32")]
extern "C" {
    fn js_log(ptr: usize, byte_size: usize);
}

pub fn log(s: &str) {
    #[cfg(target_arch = "wasm32")]
    unsafe {
        js_log(s.as_ptr() as usize, s.len());
    }

    #[cfg(not(target_arch = "wasm32"))]
    println!("{}", s);
}
