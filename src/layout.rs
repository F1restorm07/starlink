use crate::types::{ Orientation, Rect };
use crate::view::{ View, Views };

pub struct Layout<'l, V: View> {
    views: Views<'l, V>,
    orientation: Orientation,
    available_space: Rect,
}

impl<'l, V: View> Layout<'l, V> {
    pub fn new(views: impl Into<Views<'l, V>>, orientation: Orientation, available_space: Rect) -> Self {
        Self { views: views.into(), orientation, available_space }
    }
    pub fn horizontal(views: impl Into<Views<'l, V>>, available_space: Rect) -> Self {
        Self { views: views.into(), available_space, orientation: Orientation::Horizontal }
    }
    pub fn vertical(views: impl Into<Views<'l, V>>, available_space: Rect) -> Self {
        Self { views: views.into(), available_space, orientation: Orientation::Vertical }
    }
}
