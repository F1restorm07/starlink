#[derive(Debug, Clone)]
pub enum Unit {
    /// fill all of the available space in the size or parent
    Fill,
    /// a specific size `width, height` in pixels
    Pixels(u32, u32),
    /// anchor to a side or corner
    Anchor, // TODO: add an AnchorPoint enum
    /// use a fraction of the space (only supported in grid environments)
    Fraction(u8, u8),
}

#[derive(Debug, Clone)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

#[derive(Debug)]
pub struct Rect {
    /// the top-left point of the rect
    pub origin: Point,
    pub size: Size,
}
#[derive(Debug)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}
#[derive(Debug)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

impl Rect {
    pub fn new(origin: Point, size: Size) -> Self { Self { origin, size } }
    pub fn zero() -> Self { Self::new(Point::new(0, 0), Size::zero())}
}
impl Point {
    pub fn new(x: u32, y: u32) -> Self { Self { x, y } }
}
impl Size {
    pub fn new(width: u32, height: u32) -> Self { Self { width, height } }
    pub fn zero() -> Self { Self { width: 0, height: 0 } }
}
