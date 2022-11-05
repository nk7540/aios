#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct PixelColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Copy, Clone)]
pub struct Coord<T>(pub T, pub T);
