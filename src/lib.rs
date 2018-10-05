#![feature(lang_items)]
#![no_std]

extern crate volatile;
extern crate multiboot2;

mod vga_buffer;

use core::panic::PanicInfo;
use vga_buffer::*;
use core::fmt::Write;

#[no_mangle]
pub extern fn rust_main(multiboot_info_address : usize) {
    let mut writer = Writer::new();

    let boot_info = unsafe { multiboot2::load(multiboot_info_address) };
    let memory_map_tag = boot_info.memory_map_tag()
        .expect("Memory map tag required");
    writer.write_line("Memory areas:");
    writer.new_line();
    for area in memory_map_tag.memory_areas() {
        writeln!(writer, "    start: 0x{:x}, end: 0x{:x}, size: {:x}",
            area.start_address(),
            area.end_address(),
            area.size());
    }

    let elf_sections_tag = boot_info.elf_sections_tag()
        .expect("Elf-sections tag required");
    writer.write_line("Kernel sections: ");
    writer.new_line();
    for section in elf_sections_tag.sections() {
        writeln!(writer, "    addr: 0x{:x}, size: 0x{:x}, flags: 0x{:x}",
            section.start_address(),
            section.size(),
            section.flags());
    }
    loop {}
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern fn eh_personality() {}

#[panic_handler]
pub extern fn panic(info : &PanicInfo) -> ! {
    let mut writer = Writer::new();
    writeln!(writer, "{}", info);
    loop{}
}


