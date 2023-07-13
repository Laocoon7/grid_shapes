/// Implementation for describing a position
pub trait Coord: Clone + Copy + PartialEq + Eq + Default + PartialOrd + Ord {
    /// Returns a new `Coord` object
    fn new(x: i32, y: i32) -> Self;

    /// Returns the `x` position of the coordinate
    fn x(self) -> i32;
    /// Returns the `y` position of the coordinate
    fn y(self) -> i32;
}

/// Implementation for describing a 2D space
pub trait Size: Clone + Copy + PartialEq + Eq + Default + PartialOrd + Ord {
    /// Returns a new `Size` object
    fn new(width: u32, height: u32) -> Self;

    /// Returns the `width` of the space
    fn width(self) -> u32;
    /// Returns the `height` of the space
    fn height(self) -> u32;
}

#[cfg(feature = "coord_2d")]
impl Coord for coord_2d::Coord {
    fn new(x: i32, y: i32) -> Self {
        Self::new(x, y)
    }

    fn x(self) -> i32 {
        self.x
    }

    fn y(self) -> i32 {
        self.y
    }
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