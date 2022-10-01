use super::frame_buffer::{PixelDrawer, Vector2D, PixelColor};

const SHINONOME_FONT: &[u8] = include_bytes!("../../resources/hankaku.bin") as &[u8];

pub struct ShinonomeFont;
impl ShinonomeFont {
    pub fn draw_char(&self, pixel_drawer: PixelDrawer, pos: Vector2D<isize>,
        fg: PixelColor, bg: PixelColor, c: char)
    {
        let mut c = c as usize;
        if SHINONOME_FONT.len() < c {
            c = b'?' as usize;
        }
        for dy in 0..16 {
            let row = SHINONOME_FONT[c * 16 + dy as usize];
            for dx in 0..8 {
                if row & (0x80 >> dx) != 0 {
                    pixel_drawer.draw_pixel(pos + Vector2D::new(dx, dy), fg);
                } else {
                    pixel_drawer.draw_pixel(pos + Vector2D::new(dx, dy), bg);
                }
            }
        }
    }
}
