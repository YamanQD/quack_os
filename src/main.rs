#![no_std] // Disable standard library linking
#![no_main] // Disable rust's entry point
use core::panic::PanicInfo;

mod vga_buffer;

// Entry point
#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga_buffer::test_write();

    // Infinite loop for diverging function
    loop {}
}

// Called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // Infinite loop for diverging function
    loop {}
}
