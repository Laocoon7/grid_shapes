use std::{
    fmt::{Debug, Display},
    hash::Hash,
};

use crate::Coord;

pub trait Shape<C: Coord>:
    Debug + Display + Clone + Copy + PartialEq + Eq + Hash + Default + IntoIterator
{
    fn for_each<F: FnMut(C)>(self, f: F);
}
