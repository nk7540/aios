use derive_new::new;

use super::frame_buffer::Vector2D;

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash, new)]
pub struct Rect {
    pub x: isize,
    pub y: isize,
    pub w: usize,
    pub h: usize,
}

impl Rect {
    pub fn is_contained(self, pos: Vector2D<isize>) -> bool {
        self.x <= pos.x
            && pos.x < self.x + self.w as isize
            && self.y <= pos.y
            && pos.y < self.y + self.h as isize
    }
}
