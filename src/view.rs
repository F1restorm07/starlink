pub trait View {
    /// get the view's bounding rect area
    fn bounds(&self) -> crate::types::Rect;
    fn size(&self) -> crate::types::Size { self.bounds().size }
    /// move the view's origin over by `pt` pixels
    fn translate(&mut self, pt: crate::types::Point);
}

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

pub struct Views<'v, V: View> {
    inner: &'v [V],
    offset: usize
}

// ??: is there a better way to manage the lifetimes here
impl<'v, V: View> Iterator for Views<'v, V> {
    type Item = &'v V;
    fn next(&mut self) -> Option<Self::Item> {
        if self.offset >= self.inner.len() { return None; }
        let out = &self.inner[self.offset];
        self.offset+=1;
        Some(out)
    }
}

impl<'v, V: View> From<&'v [V]> for Views<'v, V> {
    fn from(value: &'v [V]) -> Self { Self { inner: value, offset: 0 } }
}
