#![no_std]
#![no_main]

mod graphics;
mod memory;
use bootloader::{entry_point, BootInfo, boot_info::Optional};
use memory::memmap::{MemoryMap, MemoryType};
use core::{arch::asm, mem};

use crate::graphics::{frame_buffer, console};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    let frame_buffer = mem::replace(&mut boot_info.framebuffer, Optional::None)
        .into_option().unwrap();
    frame_buffer::init(frame_buffer);
    console::init();
    println!("Hello, {}!", "AIOS");
    loop {unsafe {asm!("hlt")}}
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo<'_>) -> ! {
    println!("{}", info);
    loop {unsafe {asm!("hlt")}}
}
