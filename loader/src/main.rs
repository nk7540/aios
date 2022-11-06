#![no_std]
#![no_main]
#![feature(abi_efiapi)]

use core::mem::transmute;
use core::ops::{Deref,DerefMut};
use core::ptr::{copy_nonoverlapping, write_bytes};

mod elf;

use uefi::prelude::*;
use uefi::proto::console::gop::PixelFormat;
use uefi::proto::{console,media,loaded_image};
use uefi::proto::media::file::{Directory, File, FileMode, FileAttribute, FileInfo};
use uefi::table::boot::{AllocateType,MemoryType};

#[macro_use]
extern crate alloc;
use log::{info,debug};

// Loose coupling between loader and kernel
// Only entry point function type (e.g. args) is shared in the form of "spec"
// Do not share "actual files" for common definition
#[repr(C)]
struct MemoryMap {
    mmap_buf: *const u8,
    descriptor_count: u64,
}

#[repr(C)]
struct FrameBuffer {
    buffer: *mut u8,
    size: usize,
    resolution: (usize, usize),
    pixel_format: PixelFormat,
    stride: usize,
}

//
// Open the root directory on the same volume as the code is read
//
fn open_root_dir(bs: &BootServices, image_handle: Handle) -> Directory {
    // Open the Loaded Image Protocol of the Image Handle (to get its Device Handle)
    let loaded_image = bs
        .open_protocol_exclusive::<loaded_image::LoadedImage>(image_handle).unwrap();
    // Open the Simple File System Protocol of the Device Handle obtained above
    // You can get the File Protocol instance using OpenVolume provided by the SFSP.
    let mut file_system = bs
        .open_protocol_exclusive::<media::fs::SimpleFileSystem>(loaded_image.deref().device())
        .unwrap();
    file_system.deref_mut().open_volume().unwrap()
}

//
// Get frame buffer
//
fn get_frame_buffer(bs: &BootServices) -> FrameBuffer {
    let gop_handle = bs
        .get_handle_for_protocol::<console::gop::GraphicsOutput>().unwrap();
    let mut gop = bs
        .open_protocol_exclusive::<console::gop::GraphicsOutput>(gop_handle).unwrap();
    
    FrameBuffer {
        buffer: gop.frame_buffer().as_mut_ptr(),
        size: gop.frame_buffer().size(),
        resolution: gop.current_mode_info().resolution(),
        pixel_format: gop.current_mode_info().pixel_format(),
        stride: gop.current_mode_info().stride(),
    }
}

//
// Load ELF-formatted EXEC file
// 
fn load_elf(bs: &BootServices, buf: &mut [u8]) -> usize {
    let ehdr = unsafe {
        let ehdr_ptr = buf.as_ptr() as *const elf::Elf64_Ehdr;
        &*ehdr_ptr
    };
    debug!("e_entry: {:#x}", ehdr.e_entry);

    // Loop for program headers, find LOAD type segments and copy them to proper address
    // Set 0 in specified memory space that doesn't have corresponding file contents
    // It becomes an error if the specified memory space is not available
    let buf_addr = buf.as_ptr() as u64;
    for i in 0..ehdr.e_phnum {
        let phdr_addr = buf_addr + ehdr.e_phoff + u64::from(i * ehdr.e_phentsize);
        let phdr = unsafe { &*(phdr_addr as *const elf::Elf64_Phdr) };
        debug!("p_type: {}", phdr.p_type);

        if phdr.p_type != 1 { continue };
        bs.allocate_pages(
            AllocateType::Address(phdr.p_vaddr),
            MemoryType::LOADER_DATA,
            ((phdr.p_memsz + 0xfff) / 0x1000) as usize,
        ).unwrap();
        debug!("page allocated for segment {}", i + 1);

        unsafe {
            copy_nonoverlapping(
                (buf_addr + phdr.p_offset) as *const u8,
                phdr.p_vaddr as *mut u8,
                u64::from(phdr.p_memsz) as usize,
            );
            if phdr.p_filesz < phdr.p_memsz {
                write_bytes(
                    (phdr.p_vaddr as u64 + phdr.p_filesz) as *mut u8,
                    0,
                    (phdr.p_memsz - phdr.p_filesz) as usize
                );
            }
        }
        debug!("segment {} copied.", i + 1);
    }

    ehdr.e_entry
}

#[entry]
pub fn efi_main(image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();
    system_table.stdout().reset(false).unwrap();
    info!("Loader initialized.");

    let bs = system_table.boot_services();

    let mut memmap_buf = [0_u8; 4096 * 4];
    {
        let (_map_key, desc_itr) = bs
            .memory_map(&mut memmap_buf)
            .expect("Failed to get memory map");
        desc_itr.for_each(|desc| {
            if desc.ty == MemoryType::CONVENTIONAL {
                info!("{:#x}: {} pages", desc.phys_start, desc.page_count);
            }
        });
    }

    let frame_buffer = get_frame_buffer(bs);

    let mut root_dir = open_root_dir(bs, image_handle);
    let mut kernel_file = root_dir.open(
        cstr16!("kernel.elf"),
        FileMode::Read,
        FileAttribute::READ_ONLY
    ).expect("Failed to open file");
    debug!("File opened.");

    let kernel_file_info = kernel_file.get_boxed_info::<FileInfo>().unwrap();
    let mut kernel_buf = vec![0; kernel_file_info.as_ref().file_size() as usize];
    kernel_file.into_regular_file().unwrap().read(&mut kernel_buf);
    let entry_point_addr = load_elf(bs, &mut kernel_buf);

    let descriptor_count: u64;
    {
        bs.memory_map(&mut memmap_buf);
        let (_, desc_itr) = system_table.exit_boot_services(image_handle, &mut memmap_buf)
            .expect("Failed to exit boot services");
        descriptor_count = desc_itr.count() as u64;
    }
    let memmap = MemoryMap { mmap_buf: memmap_buf.as_ptr(), descriptor_count };

    type EntryPoint = extern "sysv64" fn(&FrameBuffer, &MemoryMap) -> ();
    let entry_point: EntryPoint = unsafe { transmute(entry_point_addr) };
    entry_point(&frame_buffer, &memmap);

    loop {}
}
