use core::fmt::Write;
use core::fmt;

use spin::{Mutex, Once, MutexGuard};

use super::{frame_buffer::{PixelWriter, self}, font::{self, Font}, common::{PixelColor, Coord}};

const CONSOLE_BG_COLOR: PixelColor = PixelColor { r: 0, g: 0, b: 0 };
const CONSOLE_FG_COLOR: PixelColor = PixelColor { r: 255, g: 255, b: 255 };
const SHINONOME_FONT: &[u8] = include_bytes!("../../resources/hankaku.bin") as &[u8];

// spin::Once for lazy init, spin::Mutex for interior mutability with Sync on bare metal
pub static CONSOLE: Once<Mutex<Console>> = Once::new();

pub fn init(resolution: (usize, usize)) {
    let font = font::Font::new(SHINONOME_FONT);

    CONSOLE.call_once(|| Mutex::new(Console::new(resolution, font)));
    frame_buffer::lock_pixel_writer(|w| {
        lock_console(|console| console.flush(&w))
    });
}

pub fn lock_console<F: Fn(MutexGuard<Console>)>(f: F) {
    let console = CONSOLE.get()
        .expect("console::lock_console is called before console::init");
    f(console.lock())
}

pub struct Console<'a> {
    resolution: (usize, usize),
    rows: usize,
    columns: usize,
    cursor: Coord<isize>,
    font: Font<'a>,
}
impl<'a> Console<'a> {
    pub fn new(resolution: (usize, usize), font: Font<'a>) -> Self {
        let char_size = font.char_size();
        Self {
            resolution,
            rows: resolution.0 / char_size.0 as usize,
            columns: resolution.1 / char_size.1 as usize,
            cursor: Coord(0, 0),
            font
        }
    }
    pub fn flush(&self, pixel_writer: &PixelWriter) {
        for y in 0..self.resolution.1 {
            for x in 0..self.resolution.0 {
                pixel_writer.draw_pixel(
                    Coord(x as isize, y as isize), CONSOLE_BG_COLOR);
            }
        }
    }
    pub fn put_string(&mut self, pixel_writer: &PixelWriter, s: &str) {
        for c in s.chars() {
            self.cursor = Coord(self.cursor.0 + 1, self.cursor.1);
            let pos = Coord(self.cursor.0 * self.font.char_size().0, self.cursor.1);
            self.font.draw_char(pixel_writer, pos,
                CONSOLE_FG_COLOR, CONSOLE_BG_COLOR, c);
        }
    }
}

impl fmt::Write for Console<'_> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        frame_buffer::lock_pixel_writer(|writer| {
            self.put_string(&writer, s);
        });
        Ok(())
    }
}

pub fn _print(args: fmt::Arguments) {
    lock_console(|mut console| console.write_fmt(args).unwrap())
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::graphics::console::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}
