mod units;
pub use self::units::Coord;
pub use self::units::Size;

pub mod line;
pub mod rectangle;
pub mod circle;

pub mod prelude {
    // Rectangles
    #[cfg(feature = "coord_2d")]
    pub type Rectangle = crate::rectangle::Rectangle<coord_2d::Coord, coord_2d::Size>;
    #[cfg(not(feature = "coord_2d"))]
    pub use crate::rectangle::Rectangle;

    // Lines
    #[cfg(feature = "coord_2d")]
    pub type Line = crate::line::Line<coord_2d::Coord>;
    #[cfg(not(feature = "coord_2d"))]
    pub use crate::line::Line;

    // Circles
    #[cfg(feature = "coord_2d")]
    pub type Circle = crate::circle::Circle<coord_2d::Coord>;
    #[cfg(not(feature = "coord_2d"))]
    pub use crate::circle::Circle;
}
