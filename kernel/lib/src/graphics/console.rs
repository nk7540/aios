use core::mem::MaybeUninit;
use core::fmt;

use spin::mutex::SpinMutex;

use super::{frame_buffer::{Vector2D, PixelWriter, FrameBuffer, PixelColor, self}, font::{self, Font}};

const CONSOLE_BG_COLOR: PixelColor = PixelColor { r: 0, g: 0, b: 0 };
const CONSOLE_FG_COLOR: PixelColor = PixelColor { r: 255, g: 255, b: 255 };
const SHINONOME_FONT: &[u8] = include_bytes!("../../resources/hankaku.bin") as &[u8];

pub static CONSOLE: SpinMutex<Option<Console>> = SpinMutex::new(None);

pub fn init(resolution: (usize, usize)) {
    let font = font::Font::new(SHINONOME_FONT);

    let mut locked_console = CONSOLE.lock();
    *locked_console = Some(Console::new(resolution, font));
    frame_buffer::lock_pixel_writer(|w| {
        locked_console.as_ref().unwrap().flush(w);  
    });
}

pub struct Console<'a> {
    resolution: (usize, usize),
    rows: usize,
    columns: usize,
    cursor: Vector2D<usize>,
    font: Font<'a>,
}
impl<'a> Console<'a> {
    pub fn new(resolution: (usize, usize), font: Font<'a>) -> Self {
        let char_size = font.char_size(' ');
        Self {
            resolution,
            rows: resolution.0 / char_size.x as usize,
            columns: resolution.1 / char_size.y as usize,
            cursor: Vector2D::new(0, 0),
            font
        }
    }
    pub fn flush(&self, pixel_writer: PixelWriter) {
        for y in 0..self.resolution.1 {
            for x in 0..self.resolution.0 {
                pixel_writer.draw_pixel(
                    Vector2D::new(x as isize, y as isize), CONSOLE_BG_COLOR);
            }
        }
    }
    pub fn put_string(&self, pixel_writer: PixelWriter, s: &str) {
        self.font.draw_str(pixel_writer, Vector2D::new(0, 0),
            CONSOLE_FG_COLOR, CONSOLE_BG_COLOR, s);
    }
}

impl fmt::Write for Console<'_> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        frame_buffer::lock_pixel_writer(|writer| {
            self.put_string(writer, s);
        });
        Ok(())
    }
}

#[macro_export]
macro_rules! println {
    ($( $t:tt )*) => {{
        use core::fmt::Write;
        let mut locked_console = $crate::graphics::console::CONSOLE.lock();
        let console = locked_console.as_mut().unwrap();
        writeln!(console, $( $t )*).unwrap();
    }};
}
