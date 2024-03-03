#[derive(Debug, Clone)]
pub enum LayoutUnit {
    /// fill all of the available space in the size or parent
    Fill,
    /// a specific size `width, height` in pixels
    Pixels(u32, u32),
    /// anchor to a side or corner
    Anchor, // TODO: add an AnchorPoint enum
    /// use a percentage of the space
    Percent(u8),
    /// set a minimum number of pixels
    Min(u16),
    /// set a maximum number of pixels
    Max(u16),
    /// use a fraction of the space (only supported in grid environments)
    Fraction(u8, u8),
    /// use a flex option (only supported in flex environments)
    FlexPoint,
}

#[derive(Debug, Clone)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone)]
pub struct Rect {
    /// the top-left point of the rect
    pub origin: Point,
    pub size: Size,
}
#[derive(Debug, Clone)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}
#[derive(Debug, Clone)]
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

impl crate::view::Transform for Rect {
    fn translate(&mut self, pt: crate::types::Point) {
        self.origin.x += pt.x; self.origin.y += pt.y;
    }
}

impl core::ops::Add for Rect {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let origin = self.origin;
        let size = self.size + rhs.size;
        Self { origin, size }
    }
}
impl core::ops::Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl core::ops::Add for Size {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            width: self.width + rhs.width,
            height: self.height + rhs.height,
        }
    }
}
