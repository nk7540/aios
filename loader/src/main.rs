#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
fn efi_main() {
    loop {}
}

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}
