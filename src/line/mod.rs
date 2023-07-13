use crate::Coord;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Line<C: Coord> {
    pub start: C,
    pub end: C,
}

// Constructors
impl<C: Coord> Line<C> {
    pub fn new(start: C, end: C) -> Self {
        Self {
            start,
            end,
        }
    }
}

// Implementation
impl<C: Coord> Line<C> {

}

// Iterators
impl<C: Coord> Line<C> {

}

impl<C: Coord> Default for Line<C> {
    fn default() -> Self {
        Self {
            start: C::new(0, 0),
            end: C::new(1, 0),
        }
    }
}