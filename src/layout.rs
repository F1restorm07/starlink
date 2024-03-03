use crate::types::{ Orientation, Rect };
use crate::view::View;

use alloc::vec::Vec;

pub struct Layout<V: View> {
    views: Vec<V>,
    orientation: Orientation,
    available_space: Rect,
}

impl<V: View> Layout<V> {
    pub fn new(views: impl IntoIterator<Item = V>, orientation: Orientation, available_space: Rect) -> Self {
        Self { views: views.into_iter().collect(), orientation, available_space }
    }
    pub fn horizontal(views: impl IntoIterator<Item = V>, available_space: Rect) -> Self {
        Self { views: views.into_iter().collect(), available_space, orientation: Orientation::Horizontal }
    }
    pub fn vertical(views: impl IntoIterator<Item = V>, available_space: Rect) -> Self {
        Self { views: views.into_iter().collect(), available_space, orientation: Orientation::Vertical }
    }
    pub fn views(&mut self, views: impl IntoIterator<Item = V>) { self.views = views.into_iter().collect() }
}

impl<V: View> Default for Layout<V> {
    fn default() -> Self {
        Self::new([], Orientation::Horizontal, Rect::zero())
    }
}
