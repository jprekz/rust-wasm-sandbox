#[repr(C)]
pub struct Vec2 {
    x: i32,
    y: i32,
}

#[no_mangle]
pub fn new(x: i32, y: i32) -> Vec2 {
    Vec2 { x, y }
}

#[no_mangle]
pub fn add(a: &Vec2, b: &Vec2) -> Vec2 {
    Vec2 {
        x: a.x + b.x,
        y: a.y + b.y,
    }
}

// https://devblog.arcana.rs/how-to-make-plugins-system-with-rust-and-webassembly

/// Export this function from WASM module.
/// It would allow host to allocate guest's memory.
///
/// # Safety
///
/// This function is FFI-safe wrapper for standard function `alloc::alloc::alloc`.
/// Same safety principles applies.
#[no_mangle]
pub unsafe fn malloc(size: usize, align: usize) -> *mut u8 {
    let layout = std::alloc::Layout::from_size_align(size, align).unwrap();
    std::alloc::alloc(layout)
}

/// Export this function from WASM module.
/// It would allow host to deallocate guest's memory.
///
/// # Safety
///
/// This function is FFI-safe wrapper for standard function `alloc::alloc::dealloc`.
/// Same safety principles applies.
#[no_mangle]
pub unsafe fn free(ptr: *mut u8, size: usize, align: usize) {
    let layout = std::alloc::Layout::from_size_align(size, align).unwrap();
    std::alloc::dealloc(ptr, layout);
}
