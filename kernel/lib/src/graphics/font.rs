use super::frame_buffer::{PixelWriter, Coord, PixelColor};

pub struct Font<'a> { regular: &'a[u8] }
impl<'a> Font<'a> {
    pub fn new(regular: &'a[u8]) -> Self { Self { regular } }
    pub fn char_size(&self) -> Coord<isize> {
        Coord(8 + 2, 16 + 2) // monospaced
    }
    pub fn draw_char(&self, pixel_writer: &PixelWriter, pos: Coord<isize>,
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
                    pixel_writer.draw_pixel(Coord(pos.0 + dx, pos.1 + dy), fg);
                } else {
                    pixel_writer.draw_pixel(Coord(pos.0 + dx, pos.1 + dy), bg);
                }
            }
        }
    }
}
