#![no_std]
#![no_main]
#![feature(abi_efiapi)]

use core::ops::{Deref,DerefMut};
use core::fmt::Write;

use uefi::prelude::*;
use uefi::proto::media::file::{Directory, File, FileMode, FileAttribute};
use uefi::table::boot::MemoryType;

struct MemoryMap {
    map_size: u64,
    map_buffer: *mut u8,
    map_key: u64,
    descriptor_size: u64,
    descriptor_version: u32,
}

const EFI_PAGE_SIZE: u64 = 0x1000;

fn open_root_dir(bs: &BootServices, image_handle: Handle) -> Directory {
    use uefi::proto::loaded_image::LoadedImage;
    use uefi::proto::media::fs::SimpleFileSystem;

    let loaded_image = bs
        .open_protocol_exclusive::<LoadedImage>(image_handle).unwrap();
    let mut file_system = bs
        .open_protocol_exclusive::<SimpleFileSystem>(loaded_image.deref().device()).unwrap();
    file_system.deref_mut().open_volume().unwrap()
}

#[entry]
pub fn efi_main(image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();
    writeln!(system_table.stdout(), "Hello, World!").unwrap();

    // let bs = system_table.boot_services().clone();

    let mut buf = [0_u8; 4096 * 4];
    let (_map_key, desc_itr) = system_table.boot_services()
        .memory_map(&mut buf)
        .expect("Failed to get memory map");
    desc_itr.for_each(|desc| {
        if desc.ty == MemoryType::CONVENTIONAL {
            writeln!(system_table.stdout(), "{:#x}: {} pages",
                desc.phys_start, desc.page_count).unwrap();
        }
    });

    let mut root_dir = open_root_dir(system_table.boot_services(), image_handle);
    let file = root_dir.open(
        cstr16!("kernel.elf"),
        FileMode::Read,
        FileAttribute::READ_ONLY
    ).expect("Failed to open file");
    writeln!(system_table.stdout(), "File opened.").unwrap();
    file.close();
    writeln!(system_table.stdout(), "File closed.").unwrap();

    loop {}
}
