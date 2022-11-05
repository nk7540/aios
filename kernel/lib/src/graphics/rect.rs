use derive_new::new;

use super::frame_buffer::Coord;

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash, new)]
pub struct Rect {
    pub x: isize,
    pub y: isize,
    pub w: usize,
    pub h: usize,
}

impl Rect {
    pub fn is_contained(self, pos: Coord<isize>) -> bool {
        self.x <= pos.0
            && pos.0 < self.x + self.w as isize
            && self.y <= pos.1
            && pos.1 < self.y + self.h as isize
    }
}
