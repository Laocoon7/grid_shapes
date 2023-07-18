use std::{
    fmt::{Debug, Display},
    hash::Hash,
};

use crate::{Coord, rectangle::Rectangle, Size};

pub trait Shape<C: Coord, S: Size>:
    Debug + Display + Clone + Copy + PartialEq + Eq + Hash + Default + IntoIterator
{
    /// Calls `f` for each Coord in the shape
    fn for_each<F: FnMut(C)>(self, f: F);
    
    /// Returns an axis aligned bounding box containing the shape
    fn aabb(self) -> Rectangle<C, S>;
}
