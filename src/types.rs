#[derive(Debug, Clone)]
pub enum Unit {
    /// a specific size `width, height` in pixels
    Pixels(u32, u32),
    /// fill all of the available space in the size or parent
    Fill,
}

#[derive(Debug, Clone)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

pub struct Rect {
    /// the top-left point of the rect
    pub origin: Point,
    pub size: Size,
}

pub struct Point {
    pub x: u32,
    pub y: u32,
}

pub struct Size {
    pub width: u32,
    pub height: u32,
}
