use std::io::prelude::*;
use std::{cell::Cell, fs::File};
use wasmer::{imports, Instance, MemoryView, Module, Store};

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vec2 {
    x: i32,
    y: i32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let module_wat = {
        let mut f = File::open("../target/wasm32-unknown-unknown/release/plugin_test.wasm")?;
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer)?;
        buffer
    };

    let store = Store::default();
    let module = Module::new(&store, &module_wat)?;
    let import_object = imports! {};
    let instance = Instance::new(&module, &import_object)?;

    println!("exports:");
    for (name, ext) in instance.exports.iter() {
        println!("\t{}: {:?}", name, ext.ty());
    }

    let new = instance
        .exports
        .get_native_function::<(i32, i32, i32), ()>("new")?;

    let add = instance
        .exports
        .get_native_function::<(i32, i32, i32), ()>("add")?;
    let malloc = instance
        .exports
        .get_native_function::<(i32, i32), i32>("malloc")?;
    let free = instance
        .exports
        .get_native_function::<(i32, i32, i32), ()>("free")?;
    let memory = instance.exports.get_memory("memory")?;

    let p_a = malloc.call(2, 4)?;
    let _ = new.call(p_a, 1, 2)?;
    let p_b = malloc.call(2, 4)?;
    let _ = new.call(p_b, 3, 5)?;
    let p_r = malloc.call(2, 4)?;
    let _ = add.call(p_r, p_a, p_b)?;

    let view: MemoryView<u8> = memory.view();
    for byte in view[p_a as usize..p_a as usize + 8].iter().map(Cell::get) {
        print!("{:02X} ", byte);
    }
    println!();
    for byte in view[p_b as usize..p_b as usize + 8].iter().map(Cell::get) {
        print!("{:02X} ", byte);
    }
    println!();
    for byte in view[p_r as usize..p_r as usize + 8].iter().map(Cell::get) {
        print!("{:02X} ", byte);
    }
    println!();

    let _ = free.call(p_a, 2, 4)?;
    let _ = free.call(p_b, 2, 4)?;
    let _ = free.call(p_r, 2, 4)?;

    Ok(())
}
