#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct PixelColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}


// cf: https://docs.rs/cursive_core/0.3.5/src/cursive_core/xy.rs.html#7
/// A generic structure with a value for each axis.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(clippy::upper_case_acronyms)]
pub struct XY<T> {
    /// X-axis value
    pub x: T,
    /// Y-axis value
    pub y: T,
}

impl<T> XY<T> {
    /// Creates a new `XY` from the given values.
    pub fn new(x: T, y: T) -> Self {
        XY { x, y }
    }
}
