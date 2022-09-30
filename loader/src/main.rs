#![no_std]
#![no_main]
#![feature(abi_efiapi)]

use core::ops::{Deref,DerefMut};
use core::fmt::Write;

use uefi::prelude::*;
use uefi::proto::console::text;
use uefi::proto::media::file::{Directory, File, FileMode, FileAttribute, FileInfo};
use uefi::table::boot::MemoryType;

#[macro_use]
extern crate alloc;

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

type Elf64_Addr = usize;
type Elf64_Off = u64;
type Elf64_XWord = u64;
type Elf64_Word = u32;
type Elf64_Half = u16;

#[repr(C)]
struct Elf64_Ehdr {
    a: [u8; 24],
    e_entry: Elf64_Addr,
    e_phoff: Elf64_Off,
    b: [u8; 14],
    e_phentsize: Elf64_Half,
    e_phnum: Elf64_Half,
}

struct Elf64_Phdr {
    p_type: Elf64_Word,
    p_flags: Elf64_Word,
    p_offset: Elf64_Off,
    p_vaddr: Elf64_Addr,
    p_paddr: Elf64_Addr,
    p_filesz: Elf64_XWord,
    p_memsz: Elf64_XWord,
    p_align: Elf64_XWord,
}

fn load_elf(buf: &mut [u8], stdout: &mut text::Output) {
    let ehdr_ptr = buf.as_ptr() as *const Elf64_Ehdr;
    let ehdr = unsafe { &*ehdr_ptr };
    writeln!(stdout, "e_entry: {:#x}", ehdr.e_entry);
}

#[entry]
pub fn efi_main(image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();
    system_table.stdout().reset(false).unwrap();
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
    let mut kernel_file = root_dir.open(
        cstr16!("kernel.elf"),
        FileMode::Read,
        FileAttribute::READ_ONLY
    ).expect("Failed to open file");
    writeln!(system_table.stdout(), "File opened.").unwrap();

    let kernel_file_info = kernel_file.get_boxed_info::<FileInfo>().unwrap();
    let mut buf = vec![0; kernel_file_info.as_ref().file_size() as usize];
    kernel_file.into_regular_file().unwrap().read(&mut buf);
    load_elf(&mut buf, system_table.stdout());

    writeln!(system_table.stdout(), "File read.").unwrap();

    loop {}
}
