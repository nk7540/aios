#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

mod graphics;
mod memory;
use bootloader::{entry_point, BootInfo, boot_info::Optional};
use memory::memmap::{MemoryMap, MemoryType};
use x86_64::VirtAddr;
use core::{arch::asm, mem};

use crate::{graphics::{frame_buffer, console}, memory::paging::active_level_4_table};

// This macro just creates a function named _start, which the linker will use as the entry point.
// The function must have the signature fn(&'static mut BootInfo) -> !.
entry_point!(kernel_main);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    let frame_buffer = mem::replace(&mut boot_info.framebuffer, Optional::None)
        .into_option().unwrap();
    frame_buffer::init(frame_buffer);
    console::init();
    println!("Hello, {}!", "AIOS");
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset.into_option().unwrap());
    loop {unsafe {asm!("hlt")}}
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo<'_>) -> ! {
    println!("{}", info);
    loop {unsafe {asm!("hlt")}}
}
