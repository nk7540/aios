#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::fmt::Write;

use uefi::data_types::Handle;
use uefi::table::{Boot, SystemTable};
use uefi::Status;

#[no_mangle]
pub fn efi_main(image_handle: uefi::Handle, mut system_table: SystemTable<Boot>) -> Status {
    system_table.stdout().write_str("Hello, World!");

    loop {}
}

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}
