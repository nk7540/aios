use bootloader::boot_info::{FrameBuffer, PixelFormat};
use spin::{Mutex, Once, MutexGuard};

use super::common::{PixelColor, XY};

// spin::Once for lazy init, spin::Mutex for interior mutability with Sync on bare metal
pub static PIXEL_WRITER: Once<Mutex<PixelWriter>> = Once::new();

pub fn init(frame_buffer: FrameBuffer) {
    PIXEL_WRITER.call_once(|| Mutex::new(PixelWriter::new(frame_buffer)));
}

pub fn lock_pixel_writer<F: FnMut(MutexGuard<PixelWriter>)>(mut f: F) {
    let pixel_writer = PIXEL_WRITER.get()
        .expect("frame_buffer::lock_pixel_writer is called before frame_buffer::init");
    f(pixel_writer.lock())
}

pub struct PixelWriter {
    frame_buffer: FrameBuffer,
    draw_pixel_fn: fn(buf: &mut [u8], off: usize, color: PixelColor) -> (),
}

impl PixelWriter {
    fn new(frame_buffer: FrameBuffer) -> Self {
        let pixel_format = frame_buffer.info().pixel_format;
        Self {
            frame_buffer,
            draw_pixel_fn: match pixel_format {
                PixelFormat::RGB => Self::draw_pixel_rgb,
                PixelFormat::BGR => Self::draw_pixel_bgr,
                _ => unimplemented!(),
            },
        }
    }
    fn draw_pixel_rgb(buf: &mut [u8], off: usize, color: PixelColor) {
        buf[off] = color.r;
        buf[off + 1] = color.g;
        buf[off + 2] = color.b;
    }
    fn draw_pixel_bgr(buf: &mut [u8], off: usize, color: PixelColor) {
        buf[off] = color.b;
        buf[off + 1] = color.g;
        buf[off + 2] = color.r;
    }
    pub fn draw_pixel(&mut self, pos: XY<usize>, color: PixelColor) {
        let off = {
            (pos.y * self.frame_buffer.info().stride + pos.x) * 4
        };
        // let buf = unsafe { self.frame_buffer.buffer().offset(off as isize) };
        // (self.draw_pixel_fn)(self, buf, color);
        let draw_pixel_fn = self.draw_pixel_fn.clone();
        (draw_pixel_fn)(self.frame_buffer.buffer_mut(), off, color);

    }
}
