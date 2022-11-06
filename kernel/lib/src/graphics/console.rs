use core::{fmt::Write, str::from_utf8};
use core::fmt;

use spin::{Mutex, Once, MutexGuard};

use super::common::XY;
use super::{frame_buffer::{PixelWriter, self}, font::{self, Font}, common::PixelColor};

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

const ROWS: usize = 25;
const COLUMNS: usize = 80;
pub struct Console<'a> {
    resolution: (usize, usize),
    cursor: XY<usize>,
    buf: [[u8; COLUMNS]; ROWS],
    font: Font<'a>,
}
impl<'a> Console<'a> {
    pub fn new(resolution: (usize, usize), font: Font<'a>) -> Self {
        Self {
            resolution,
            cursor: XY::new(0, 0),
            buf: [[0; COLUMNS]; ROWS],
            font
        }
    }
    pub fn flush(&self, pixel_writer: &PixelWriter) {
        for y in 0..self.resolution.1 {
            for x in 0..self.resolution.0 {
                pixel_writer.draw_pixel(XY::new(x, y), CONSOLE_BG_COLOR);
            }
        }
    }
    pub fn put_string(&mut self, pixel_writer: &PixelWriter, s: &str) {
        for c in s.chars() {
            if c == '\n' {
                self.newline(pixel_writer);
            } else if self.cursor.x < COLUMNS - 1 {
                let pos = XY::new(self.cursor.x * self.font.char_size().x,
                    self.cursor.y * self.font.char_size().y);
                self.font.draw_char(pixel_writer, pos,
                    CONSOLE_FG_COLOR, CONSOLE_BG_COLOR, c);
                self.buf[self.cursor.y][self.cursor.x] = c as u8;
                self.cursor.x += 1;
            }
        }
    }
    fn newline(&mut self, pixel_writer: &PixelWriter) {
        self.cursor.x = 0;
        if self.cursor.y < ROWS - 1 {
            self.cursor.y += 1;
        } else {
            self.flush(pixel_writer);
            for row in 0..ROWS {
                self.buf[row] = self.buf[row+1];
                let buf = self.buf.clone(); // to borrow self as mut in next line
                self.put_string(pixel_writer, from_utf8(&buf[row]).unwrap());
            }
            self.buf[ROWS-1] = [0; COLUMNS];
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

#[doc(hidden)]
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
