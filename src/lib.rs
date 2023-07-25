#[cfg(feature = "extentions")]
pub mod grid_extentions;

pub mod shapes;

pub mod prelude {
    // Trait
    pub use crate::shapes::Shape;
    
    // Circles
    pub use crate::shapes::Circle;

    // Lines
    pub use crate::shapes::Line;

    // Rectangles
    pub use crate::shapes::Rectangle;

    // Extensions
    #[cfg(feature = "extentions")]
    pub use crate::grid_extentions::*;
}
