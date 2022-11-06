#![no_std]
#![no_main]

#[macro_use]
extern crate lib;

use lib::graphics::frame_buffer::{FrameBuffer};
use lib::graphics::{frame_buffer, console};
use lib::memory::memmap::{MemoryMap, MemoryType};
use core::arch::asm;

#[no_mangle]
extern "C" fn kernel_main(frame_buf: &FrameBuffer, mmap: &MemoryMap) {
    frame_buffer::init(*frame_buf);
    console::init(frame_buf.resolution);
    println!("Hello, {}!", "AIOS");
    for d in mmap.descriptors() {
        if d.ty == MemoryType::CONVENTIONAL {
            println!(
                "addr={:#10x}-{:#10x}, pages = {:#10x}, kind = {:?}",
                d.phys_start,
                d.phys_start + 4096 * d.page_count - 1,
                d.page_count,
                d.ty,
            )
        }
    }
    loop {unsafe {asm!("hlt")}}
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo<'_>) -> ! {
    println!("{}", info);
    loop {unsafe {asm!("hlt")}}
}
