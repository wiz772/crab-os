#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    let all_char_bytes: &[u8; 29] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ!@#";
    {
        for byte in all_char_bytes {
            let mut writer = crate::vga_buffer::WRITER.lock();

            for _ in 0..5 {
                writer.fill_with_char(*byte);
            }
            
            drop(writer); 

            for _ in 0..50_000 {
                core::hint::spin_loop();
            }
        }
    }
    crate::vga_buffer::WRITER.lock().clear_screen();
    println!("[PANIC] {}", _info);
    loop {}
}


#[unsafe(no_mangle)]
pub extern "C"  fn _start() -> ! {
    loop{}
}