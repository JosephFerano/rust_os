#![feature(lang_items)]
#![no_std]

extern crate volatile;

mod vga_buffer;

use core::panic::PanicInfo;
use vga_buffer::*;

#[no_mangle]
pub extern fn rust_main(multiboot_info_address : usize) {
    use core::fmt::Write;
    let mut writer = Writer::new();
    let total_size = unsafe { &mut *(multiboot_info_address as *mut u32) };
    let reserved = unsafe { &mut *((multiboot_info_address * 4) as *mut u32) };
//    write!(writer, "Total Size: {}", total_size);
//    writer.write_line("");
    write!(writer, "Reserved: {}", reserved);
//    buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },

    loop {}
//    let loopTime = 100000000;
//    let mut counter = 0;
//    loop {
//        if counter == loopTime {
//            writer.write_line("Hello, this is a test");
//            counter = 0;
//        }
//        counter += 1;
//    }
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern fn eh_personality() {}

#[panic_handler]
pub extern fn panic(_info : &PanicInfo) -> ! { loop{} }


