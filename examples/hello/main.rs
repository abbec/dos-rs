#![no_std]

use dos_rs::*;

#[no_mangle]
fn start() {
    dos::exit(main());
}

fn main() -> u8 {
    dos::display_string("Hello from Rust in protected mode!$");
    return 0;
}
