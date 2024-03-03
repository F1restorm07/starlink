pub trait View {
    /// get the view's bounding rect area
    fn bounds(&self) -> crate::types::Rect;
    /// move the view's origin over by `pt` pixels
    fn translate(&mut self, pt: crate::types::Point);
    /// get the view's layout strategy, the defult implementation returns `Unit::Fill`
    fn layout_strategy(&self) -> crate::types::Unit { crate::types::Unit::Fill }
    fn size(&self) -> crate::types::Size { self.bounds().size }
}

// TODO: move these two traits into a separate file
pub trait Transform {
    /// move the view's origin over by `pt` pixels
    fn translate(&mut self, pt: crate::types::Point);
    // /// rotate the view's origin by `deg` degrees
    // fn rotate(&mut self, deg: u16);
}
 pub trait Bounds {
    fn bounds(&self) -> crate::types::Rect;
}

impl<T> View for T
    where T: Transform + Bounds
{
    fn bounds(&self) -> crate::types::Rect { self.bounds() }
    fn translate(&mut self, pt: crate::types::Point) { self.translate(pt); }
}
