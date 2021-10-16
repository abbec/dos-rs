#![no_std]

use dos_rs::*;

#[dos_rs::main]
fn main() -> u8 {
    println!("Hello from Rust in protected mode!");
    println!("Mem info: {:?}", dpmi::get_free_mem_info());

    0
}
