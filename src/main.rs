#![no_std] // Disable standard library linking
#![no_main] // Disable rust's entry point
use core::panic::PanicInfo;

mod vga_buffer;

// Entry point
#[no_mangle]
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;
    use vga_buffer::WRITER;

    WRITER.lock().write_str("Quack quack!\n").unwrap();
    write!(WRITER.lock(), "100/3 = {}", 100.0 / 3.0).unwrap();

    // Infinite loop for diverging function
    loop {}
}

// Called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // Infinite loop for diverging function
    loop {}
}
