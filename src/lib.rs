#![feature(lang_items)]
#![no_std]

mod vga_buffer;

use core::panic::PanicInfo;
use vga_buffer::*;

#[no_mangle]
pub extern fn rust_main() {
    let mut writer = Writer::new();

    writer.write_string("Hello, this is a test");
//    let loopTime = 100000000;
//    let mut counter = 0;
    loop {
//        if counter == loopTime {
//            writer.write_string("Hello, this is a test");
//            counter = 0;
//        }
//        counter += 1;
    }
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern fn eh_personality() {}

#[panic_handler]
pub extern fn panic(_info : &PanicInfo) -> ! { loop{} }


