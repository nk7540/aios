use spin::{Mutex, Once, MutexGuard};

use super::common::{PixelColor, Coord};

#[derive(Eq, PartialEq, Clone, Copy)]
pub enum PixelFormat {
    /// Each pixel is 32-bit long, with 24-bit RGB, and the last byte is reserved.
    Rgb = 0,
    /// Each pixel is 32-bit long, with 24-bit BGR, and the last byte is reserved.
    Bgr,
    /// Custom pixel format, check the associated bitmask.
    Bitmask,
    /// The graphics mode does not support drawing directly to the frame buffer.
    ///
    /// This means you will have to use the `blt` function which will
    /// convert the graphics data to the device's internal pixel format.
    BltOnly,
    // SAFETY: UEFI also defines a PixelFormatMax variant, and states that all
    //         valid enum values are guaranteed to be smaller. Since that is the
    //         case, adding a new enum variant would be a breaking change, so it
    //         is safe to model this C enum as a Rust enum.
}

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

// align as repr(C) to accept the exact argument passed by loader
#[derive(Eq, PartialEq, Clone, Copy)]
#[repr(C)]
pub struct FrameBuffer {
    pub buffer: *mut u8,            // raw framebuffer pointer
    pub size: usize,                // framebuffer size in bytes
    pub resolution: (usize, usize), // (horizontal, vertical) resolution
    pub pixel_format: PixelFormat,  // format of the frame buffer
    pub stride: usize,              // number of pixels per scanline.
}
unsafe impl Send for FrameBuffer {}

impl FrameBuffer {
    pub fn width(&self)  -> usize { self.resolution.0 }
    pub fn height(&self) -> usize { self.resolution.1 }
}


#[derive(Copy, Clone)]
pub struct PixelWriter {
    frame_buffer: FrameBuffer,
    draw_pixel_fn: fn(&Self, buf: *mut u8, color: PixelColor) -> (),
}

impl PixelWriter {
    fn new(frame_buffer: FrameBuffer) -> Self {
        Self {
            frame_buffer,
            draw_pixel_fn: match frame_buffer.pixel_format {
                PixelFormat::Rgb => Self::draw_pixel_rgb,
                PixelFormat::Bgr => Self::draw_pixel_bgr,
                _ => unimplemented!(),
            },
        }
    }
    fn draw_pixel_rgb(&self, buf: *mut u8, color: PixelColor) {
        unsafe {
            *buf.add(0) = color.r;
            *buf.add(1) = color.g;
            *buf.add(2) = color.b;
        }
    }
    fn draw_pixel_bgr(&self, buf: *mut u8, color: PixelColor) {
        unsafe {
            *buf.add(0) = color.b;
            *buf.add(1) = color.g;
            *buf.add(2) = color.r;
        }
    }
    pub fn draw_pixel(&self, pos: Coord<isize>, color: PixelColor) {
        let off = (pos.1 * self.frame_buffer.stride as isize + pos.0) * 4;
        let buf = unsafe { self.frame_buffer.buffer.offset(off) };
        (self.draw_pixel_fn)(self, buf, color);
    }
    pub fn width(&self)  -> usize { self.frame_buffer.width() }
    pub fn height(&self) -> usize { self.frame_buffer.height() }
}
