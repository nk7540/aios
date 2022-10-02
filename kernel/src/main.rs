#![no_std]
#![no_main]

#[macro_use]
use lib::println;

use lib::graphics::frame_buffer::{FrameBuffer};
use lib::graphics::{frame_buffer, console};
use core::arch::asm;

#[no_mangle]
extern "C" fn kernel_main(frame_buffer: &FrameBuffer) {
    frame_buffer::init(*frame_buffer);
    console::init(frame_buffer.resolution);
    println!("Hello, World!");
    loop {unsafe {asm!("hlt")}}
}

#[panic_handler]
fn panic(_panic: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}
