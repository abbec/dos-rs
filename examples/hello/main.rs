#![no_std]

use dos_rs::*;

// TODO: make proc macro so below main becomes
// #[dos::main]
// fn main() -> u8 { ... }
#[no_mangle]
fn start() {
    dos::exit(main());
}

fn main() -> u8 {
    println!("Hello from Rust in protected mode!");
    println!("Mem info: {:?}", dpmi::get_free_mem_info());

    0
}
