#![no_std]
#![feature(asm)]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

#[macro_export]
macro_rules! println {
    () => {
        $crate::dos::output_character('\n');
    };
    ($($arg:tt)*) => {{
        let mut writer = $crate::dos::Writer;
        core::fmt::write(&mut writer, format_args!($($arg)*)).unwrap();
        $crate::dos::output_character('\n');
    }};
}

pub mod dos {

    use core::fmt::Write;

    /// Display a string on the console
    ///
    /// Note: The string must be terminated with '$'.
    pub fn display_string(s: &str) {
        assert!(s.ends_with('$'), "String needs to be delimited with '$'");
        unsafe {
            asm!("mov ah, 9",
                 "int 0x21",
                 in("edx") s.as_ptr()
            );
        }
    }

    /// Output a single character to standard output
    pub fn output_character(c: char) {
        unsafe {
            asm!("mov ah, 2",
             "int 0x21",
             in("dl") c as u8
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

    pub struct Writer;

    impl Write for Writer {
        fn write_str(&mut self, s: &str) -> Result<(), core::fmt::Error> {
            // TODO: can prob use direct I/O here? faster?
            for c in s.chars() {
                output_character(c)
            }

            Ok(())
        }
    }
}

pub mod dpmi {

    #[repr(C)]
    #[derive(Debug, Default)]
    /// Representation of free memory information
    /// as returned by DPMI call 500h
    pub struct FreeMemInfo {
        /// Size of the largest free block in bytes
        pub largest_free_block_size: u32,

        /// Maximum unlocked page allocation in pages
        pub max_unlocked_page_alloc: u32,

        /// Maximum locked page allocation in pages
        pub max_locked_page_alloc: u32,

        /// Linear address space size in pages
        pub address_space_size: u32,

        /// Total number of unlocked pages
        pub num_unlocked_pages: u32,

        /// Total number of free pages
        pub num_free_pages: u32,

        /// Total number of physical pages
        pub num_physical_pages: u32,

        /// Free linear address space in pages
        pub free_address_space_pages: u32,

        /// Size of paging file/partition in pages
        pub page_file_size: u32,

        _reserved: [u8; 12],
    }

    /// Retrieve free memory information
    ///
    /// http://www.delorie.com/djgpp/doc/dpmi/api/310500.html
    pub fn get_free_mem_info() -> FreeMemInfo {
        let mut r = FreeMemInfo::default();

        unsafe {
            asm!("push es",
                 "push ds",
                 "pop es",
                 "mov ax, 0x500",
                 "int 0x31",
                 "pop es",
                 "sbb ax,ax",
                 in("edi") (&mut r)
            );
        }

        r
    }
}
