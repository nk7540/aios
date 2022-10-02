use super::frame_buffer::{PixelWriter, Vector2D, PixelColor};

pub struct Font<'a> { regular: &'a[u8] }
impl<'a> Font<'a> {
    pub fn new(regular: &'a[u8]) -> Self { Self { regular } }
    pub fn char_size(&self, _ch: char) -> Vector2D<isize> {
        Vector2D::new(8 + 2, 16 + 2) // monospaced
    }
    pub fn draw_char(&self, pixel_writer: PixelWriter, pos: Vector2D<isize>,
        fg: PixelColor, bg: PixelColor, c: char)
    {
        let mut c = c as usize;
        if self.regular.len() < c {
            c = b'?' as usize;
        }
        for dy in 0..16 {
            let row = self.regular[c * 16 + dy as usize];
            for dx in 0..8 {
                if row & (0x80 >> dx) != 0 {
                    pixel_writer.draw_pixel(pos + Vector2D::new(dx, dy), fg);
                } else {
                    pixel_writer.draw_pixel(pos + Vector2D::new(dx, dy), bg);
                }
            }
        }
    }
    pub fn draw_str(&self, pixel_writer: PixelWriter, pos: Vector2D<isize>,
        fg: PixelColor, bg: PixelColor, s: &str)
    {
        let mut dx = 0;
        for c in s.chars() {
            self.draw_char(pixel_writer, pos + Vector2D::new(dx, 0), fg, bg, c);
            dx += self.char_size(c).x;
        }
    }
}
