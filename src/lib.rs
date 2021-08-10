#![no_std]
#![feature(asm)]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

pub mod dos {
    /// Display a string on the console
    ///
    /// Note: The string must be terminated with '$'.
    pub fn display_string(s: &str) {
        // TODO: the assert causes "bad relocation type specified"
        // when linking, not sure why yet
        // assert!(s.is_ascii());
        let stringptr = s.as_ptr();
        unsafe {
            asm!("mov ah, 9",
                 "int 0x21",
                 in("edx") stringptr
            );
        }
    }

    /// Exit the program and return control back to DOS with
    /// the specified exit code
    pub fn exit(code: u8) {
        let exitcode = 0x4c00i16 | code as i16;
        unsafe {
            asm!("int 0x21", in("ax") exitcode);
        }
    }
}
