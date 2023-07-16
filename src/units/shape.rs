use std::{
    fmt::{Debug, Display},
    hash::Hash,
};

use crate::{Coord, rectangle::Rectangle, Size};

pub trait Shape<C: Coord, S: Size>:
    Debug + Display + Clone + Copy + PartialEq + Eq + Hash + Default + IntoIterator
{
    fn for_each<F: FnMut(C)>(self, f: F);
    fn aabb(self) -> Rectangle<C, S>;
}
