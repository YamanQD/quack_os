#![no_std] // Disable standard library linking
#![no_main] // Disable rust's entry point
use core::panic::PanicInfo;

mod vga_buffer;

// Entry point
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("qUaCk {}", "quack");

    // Infinite loop for diverging function
    loop {}
}

// Called on panic
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    // Infinite loop for diverging function
    loop {}
}
