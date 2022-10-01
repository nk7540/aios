use derive_new::new;
use core::{slice::from_raw_parts_mut};

use crate::error::Error;

use super::rect::Rect;

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

pub fn init(mut fb: FrameBuffer) -> PixelDrawer {
    fb.init_rect();
    PixelDrawer::new(fb)
}

#[derive(Eq, PartialEq, Clone, Copy)]
#[repr(C)]
pub struct FrameBuffer {
    pub buffer: *mut u8,            // raw framebuffer pointer
    pub size: usize,                // framebuffer size in bytes
    pub resolution: (usize, usize), // (horizontal, vertical) resolution
    pub pixel_format: PixelFormat,  // format of the frame buffer
    pub stride: usize,              // number of pixels per scanline.

    // Extra fields
    pub rect: Rect,
}

impl FrameBuffer {
    pub fn width(&self)  -> usize { self.resolution.0 }
    pub fn height(&self) -> usize { self.resolution.1 }
    fn init_rect(&mut self) {
        self.rect = Rect::new(0, 0, self.width(), self.height());
    }
    fn buf_at(&self, pos: Vector2D<isize>) -> Result<&mut [u8], Error> {
        if self.rect.is_contained(pos) {
            let off = (pos.y as usize * self.stride + pos.x as usize) * 4;
            let addr = self.buffer as usize + off;
            unsafe { Ok(from_raw_parts_mut(addr as *mut u8, 4)) }
        } else {
            Err(Error::new())
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, new)]
pub struct Vector2D<T> {
    pub x: T,
    pub y: T,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct PixelColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Copy, Clone)]
pub struct PixelDrawer {
    frame_buffer: FrameBuffer,
    draw_pixel_fn: fn(&Self, buf: *mut u8, color: PixelColor) -> (),
}

impl PixelDrawer {
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
    pub fn draw_pixel(&self, pos: Vector2D<isize>, color: PixelColor) {
        let off = (pos.y * self.frame_buffer.stride as isize + pos.x) * 4;
        let buf = unsafe { self.frame_buffer.buffer.offset(off) };
        (self.draw_pixel_fn)(self, buf, color);
    }
    pub fn width(&self)  -> usize { self.frame_buffer.width() }
    pub fn height(&self) -> usize { self.frame_buffer.height() }
}
