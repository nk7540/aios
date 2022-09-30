#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::fmt::Write;

use uefi::prelude::*;
use uefi::table::boot::MemoryType;

struct MemoryMap {
    map_size: u64,
    map_buffer: *mut u8,
    map_key: u64,
    descriptor_size: u64,
    descriptor_version: u32,
}

const EFI_PAGE_SIZE: u64 = 0x1000;

#[no_mangle]
pub fn efi_main(image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    writeln!(system_table.stdout(), "Hello, World!").unwrap();

    let status: Status;
    let bs = system_table.boot_services();

    let mut buf = [0_u8; 4096 * 4];
    let (_map_key, desc_itr) = bs
        .memory_map(&mut buf)
        .expect("Failed to get memory map");
    desc_itr.for_each(|desc| {
        if desc.ty == MemoryType::CONVENTIONAL {
            writeln!(system_table.stdout(), "{:#x}: {} pages",
                desc.phys_start, desc.page_count).unwrap();
        }
    });


    loop {}
}

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}
