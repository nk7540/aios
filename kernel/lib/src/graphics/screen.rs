use super::frame_buffer::{PixelDrawer, Vector2D, PixelColor, FrameBuffer};

pub const DESKTOP_BG_COLOR: PixelColor = PixelColor {
    r: 45,
    g: 118,
    b: 237,
};

pub fn init(pixel_drawer: PixelDrawer) {
    fill_rect(pixel_drawer, Vector2D::new(0, 0),
        Vector2D::new(pixel_drawer.width(), pixel_drawer.height()), DESKTOP_BG_COLOR);
}

pub fn fill_rect(pixel_drawer: PixelDrawer, pos: Vector2D<isize>, size: Vector2D<usize>, color: PixelColor) {
    for dy in 0..size.y {
        for dx in 0..size.x {
            pixel_drawer.draw_pixel(Vector2D::new(pos.x + dx as isize, pos.y + dy as isize), color)
        }
    }
}
