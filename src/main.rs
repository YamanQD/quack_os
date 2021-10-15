#![no_std] // Disable standard library linking
#![no_main] // Disable rust's entry point
use core::panic::PanicInfo;

// Called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Entry point
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}
