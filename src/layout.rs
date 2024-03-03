use crate::types::{ Orientation, Rect, LayoutUnit };
use crate::view::{ View, Transform };

use alloc::vec::Vec;

pub struct Layout<V: View> {
    views: Vec<V>,
    orientation: Orientation,
    available_space: Rect,
    group_layout: Option<LayoutUnit>,
}

impl<V: View> Layout<V> {
    pub fn new(views: impl IntoIterator<Item = V>, orientation: Orientation, available_space: Rect) -> Self {
        Self { views: views.into_iter().collect(), orientation, available_space, group_layout: None }
    }
    pub fn horizontal(views: impl IntoIterator<Item = V>, available_space: Rect) -> Self {
        Self {
            views: views.into_iter().collect(),
            available_space,
            orientation: Orientation::Horizontal,
            group_layout: None
        }
    }
    pub fn vertical(views: impl IntoIterator<Item = V>, available_space: Rect) -> Self {
        Self {
            views: views.into_iter().collect(),
            available_space,
            orientation: Orientation::Vertical,
            group_layout: None
        }
    }
    pub fn add_views(&mut self, views: impl IntoIterator<Item = V>) { self.views.extend(views); }
    pub fn orientation(&mut self, orientation: Orientation) { self.orientation = orientation; }
    pub fn available_space(&mut self, space: Rect) { self.available_space = space; }
    pub fn add_group_layout(&mut self, layout: LayoutUnit) { self.group_layout = Some(layout); }
}

impl<V: View> Default for Layout<V> {
    fn default() -> Self {
        Self::new([], Orientation::Horizontal, Rect::zero())
    }
}


// this should enable nested layouts
impl<V: View> View for Layout<V> {
    fn bounds(&self) -> crate::types::Rect {
        self.views.iter().fold(Rect::zero(), |l, r| l + r.bounds())
    }
    fn translate(&mut self, pt: crate::types::Point) { self.available_space.translate(pt); }
    fn layout(&self) -> crate::types::LayoutUnit {
        match self.group_layout.clone() {
            Some(layout) => layout,
            None => LayoutUnit::Fill
        }
    }
}
