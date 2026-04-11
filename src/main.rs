#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}


#[unsafe(no_mangle)]
pub extern "C"  fn _start() -> ! {
    screen_fill!('c');
    crate::vga_buffer::WRITER.lock().clear_screen();
    loop{}
}