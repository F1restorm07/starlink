// ??: should there be a way to collect multiple views into a single view
//  \__ would there need to be a way to output multiple layout strategies (or instead a single group strategy)?
//      \__ if there was a way to putout multiple strategies, would they only apply to witin a group?

pub trait View {
    /// get the view's bounding rect area
    fn bounds(&self) -> crate::types::Rect;
    /// move the view's origin over by `pt` pixels
    fn translate(&mut self, pt: crate::types::Point);
    /// get the view's layout strategy, the defult implementation returns `LayoutUnit::Fill`
    fn layout(&self) -> crate::types::LayoutUnit { crate::types::LayoutUnit::Fill }
    fn size(&self) -> crate::types::Size { self.bounds().size }
}

// TODO: move these two traits into a separate file
pub trait Transform {
    /// move the view's origin over down and right by `pt` pixels
    // TODO: support moving up and left (subtraction)
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
