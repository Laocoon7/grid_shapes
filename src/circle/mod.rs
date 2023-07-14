use crate::{Coord, iters::circle::{CircleIter, CircleCircumferenceIter}};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Circle<C: Coord> {
    pub center: C,
    pub radius: u32,
}

// Constructors
impl<C: Coord> Circle<C> {
    /// Creates a new circle.
    pub fn new(center: C, radius: u32) -> Self {
        Self {
            center,
            radius,
        }
    }
}

// Imnplementation
impl<C: Coord> Circle<C> {
    /// Get the center of the circle
    pub const fn center(self) -> C {
        self.center
    }

    /// Get the left point of the circle
    pub fn left(self) -> C {
        C::new(self.center.x() - self.radius as i32, self.center.y())
    }

    /// Get the right point of the circle
    pub fn right(self) -> C {
        C::new(self.center.x() + self.radius as i32, self.center.y())
    }

    /// Get the top point of the circle
    pub fn top(self) -> C {
        C::new(self.center.x(), self.center.y() + self.radius as i32)
    }
    
    /// Get the bottom point of the circle
    pub fn bottom(self) -> C {
        C::new(self.center.x(), self.center.y() - self.radius as i32)
    }

    pub fn get_count(self) -> u32 {
        self.into_iter().count() as u32
    }

    pub fn contains(self, position: C) -> bool {
        self.into_iter().find(|&c| c == position).is_some()
    }
}

// Iterators
impl<C: Coord> Circle<C> {
    pub fn circumference_iter(self) -> CircleCircumferenceIter<C> {
        CircleCircumferenceIter::new(self.center, self.radius)
    }

    pub fn for_each<F: FnMut(C)>(self, mut f: F) {
        for coord in self {
            f(coord);
        }
    }

    pub fn for_each_circumference<F: FnMut(C)>(self, mut f: F) {
        for coord in self.circumference_iter() {
            f(coord);
        }
    }
}

impl<C: Coord> IntoIterator for Circle<C> {
    type IntoIter = CircleIter<C>;
    type Item = C;
    fn into_iter(self) -> Self::IntoIter {
        CircleIter::new(self.center, self.radius)
    }
}

impl<C: Coord> Default for Circle<C> {
    fn default() -> Self {
        Self {
            center: C::new(0, 0),
            radius: 1,
        }
    }
}