#![no_std] // Disable standard library linking
#![no_main] // Disable rust's entry point
use core::panic::PanicInfo;

// Byte array representing a string
static HELLO: &[u8] = b"Quack World!";

// Entry point
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // VGA text buffer memory pointer
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            // Set character byte
            *vga_buffer.offset(i as isize * 2) = byte;
            // Set color byte (cyan)
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    // Infinite loop for diverging function
    loop {}
}

// Called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // Infinite loop for diverging function
    loop {}
}
