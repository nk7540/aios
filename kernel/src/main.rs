#![no_std]
#![no_main]

#[macro_use]
extern crate lib;

use lib::graphics::frame_buffer::{FrameBuffer};
use lib::graphics::{frame_buffer, console};
use core::arch::asm;

#[no_mangle]
extern "C" fn kernel_main(frame_buffer: &FrameBuffer) {
    frame_buffer::init(*frame_buffer);
    console::init(frame_buffer.resolution);
    println!("Hello, {}!", "AIOS");
    loop {unsafe {asm!("hlt")}}
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo<'_>) -> ! {
    println!("{}", info);
    loop {unsafe {asm!("hlt")}}
}
