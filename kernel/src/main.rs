#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
fn kernel_main() {
    loop {}
}

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}
