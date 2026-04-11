#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("[PANIC] {}", _info);
    loop {}
}


#[unsafe(no_mangle)]
pub extern "C"  fn _start() -> ! {
    loop{}
}