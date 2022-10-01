#![no_std]
#![no_main]

use lib::graphics::frame_buffer::{FrameBuffer};
use lib::graphics::{frame_buffer, screen};
use core::arch::asm;

#[no_mangle]
extern "C" fn kernel_main(frame_buffer: &FrameBuffer) {
    let pixel_drawer = frame_buffer::init(*frame_buffer);
    screen::init(pixel_drawer);
    loop {unsafe {asm!("hlt")}}
}

#[panic_handler]
fn panic(_panic: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}
