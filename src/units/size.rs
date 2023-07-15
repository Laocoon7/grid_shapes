use std::{fmt::Debug, hash::Hash};

/// Implementation for describing a 2D space
pub trait Size: Debug + Clone + Copy + PartialEq + Eq + Default + PartialOrd + Ord + Hash {
    /// Returns a new `Size` object
    fn new(width: u32, height: u32) -> Self;

    /// Returns the `width` of the space
    fn width(self) -> u32;
    /// Returns the `height` of the space
    fn height(self) -> u32;
}

#[cfg(feature = "coord_2d")]
impl Size for coord_2d::Size {
    fn new(width: u32, height: u32) -> Self {
        Self::new(width, height)
    }

    fn width(self) -> u32 {
        self.width()
    }

    fn height(self) -> u32 {
        self.height()
    }
}
