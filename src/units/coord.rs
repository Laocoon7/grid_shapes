use std::{fmt::Debug, hash::Hash};

/// Implementation for describing a position
pub trait Coord: Debug + Clone + Copy + PartialEq + Eq + Default + PartialOrd + Ord + Hash {
    /// Returns a new `Coord` object
    fn new(x: i32, y: i32) -> Self;

    /// Returns the `x` position of the coordinate
    fn x(self) -> i32;
    /// Sets the 'x' position of the coordinate
    fn set_x(&mut self, x: i32);
    /// Returns the `y` position of the coordinate
    fn y(self) -> i32;
    /// Sets the 'y' position of the coordinate
    fn set_y(&mut self, x: i32);
}

#[cfg(feature = "coord_2d")]
impl Coord for coord_2d::Coord {
    fn new(x: i32, y: i32) -> Self {
        Self::new(x, y)
    }

    fn x(self) -> i32 {
        self.x
    }

    fn set_x(&mut self, x: i32) {
        self.x = x;
    }

    fn y(self) -> i32 {
        self.y
    }

    fn set_y(&mut self, y: i32) {
        self.y = y;
    }
}
