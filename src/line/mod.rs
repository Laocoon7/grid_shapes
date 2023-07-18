use std::{fmt::{Debug, Display}, marker::PhantomData};

use crate::{
    iters::line::{
        BresenhamLineIter, TunnelHorizontalVerticalLineIter, TunnelVerticalHorizontalLineIter,
    },
    units::Shape,
    Coord, rectangle::Rectangle, Size,
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
/// Represents a Line on a grid
pub struct Line<C: Coord, S: Size> {
    pub start: C,
    pub end: C,

    _phantom: PhantomData<S>,
}

// Constructors
impl<C: Coord, S: Size> Line<C, S> {
    /// Creates a new Line
    pub fn new(start: C, end: C) -> Self {
        Self { start, end, _phantom: PhantomData }
    }
}

// Implementation
impl<C: Coord, S: Size> Line<C, S> {
    /// Get the total length of the line
    pub fn len(self) -> u32 {
        (self.end.x() - self.start.x())
            .abs()
            .max((self.end.y() - self.start.y()).abs()) as u32
            + 1
    }

    /// Get the starting point of the line
    pub fn start(self) -> C {
        self.start
    }

    /// Get the ending point of the line
    pub fn end(self) -> C {
        self.end
    }
}

// Iterator
impl<C: Coord, S: Size> Line<C, S> {
    /// Provides an iterator over a line horizontal and then vertical reaching the ending
    pub fn tunnel_horizontal_vertical_iter(self) -> TunnelHorizontalVerticalLineIter<C> {
        TunnelHorizontalVerticalLineIter::new(self.start, self.end)
    }

    /// Provides an iterator over a line vertical and then horizontal reaching the ending
    pub fn tunnel_vertical_horizontal_iter(self) -> TunnelVerticalHorizontalLineIter<C> {
        TunnelVerticalHorizontalLineIter::new(self.start, self.end)
    }

    /// Calls `f` for each Coord in the tunnel
    pub fn for_each_tunnel_horizontal_vertical<F: FnMut(C)>(self, mut f: F) {
        for coord in self.tunnel_horizontal_vertical_iter() {
            f(coord);
        }
    }

    /// Calls `f` for each Coord in the tunnel
    pub fn for_each_tunnel_vertical_horizontal<F: FnMut(C)>(self, mut f: F) {
        for coord in self.tunnel_vertical_horizontal_iter() {
            f(coord);
        }
    }
}

// Shape
impl<C: Coord, S: Size> Shape<C, S> for Line<C, S> {
    fn for_each<F: FnMut(C)>(self, mut f: F) {
        for coord in self {
            f(coord);
        }
    }

    fn aabb(self) -> Rectangle<C, S> {
        Rectangle::from_corners(self.start, self.end)
    }
}

impl<C: Coord, S: Size> IntoIterator for Line<C, S> {
    type IntoIter = BresenhamLineIter<C>;
    type Item = C;
    fn into_iter(self) -> Self::IntoIter {
        BresenhamLineIter::new(self.start, self.end)
    }
}

impl<C: Coord, S: Size> Default for Line<C, S> {
    fn default() -> Self {
        Self {
            start: C::new(0, 0),
            end: C::new(1, 0),
            _phantom: PhantomData,
        }
    }
}

impl<C: Coord, S: Size> Debug for Line<C, S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Line {{ start: ({}, {}), end: ({}, {}) }}",
            self.start.x(),
            self.start.y(),
            self.end.x(),
            self.end.y()
        )
    }
}

impl<C: Coord, S: Size> Display for Line<C, S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Line {{\n\tstart: ({}, {}),\n\tend: ({}, {}),\n}}",
            self.start.x(),
            self.start.y(),
            self.end.x(),
            self.end.y()
        )
    }
}

#[cfg(feature = "coord_2d")]
#[cfg(test)]
mod test {
    use super::*;
    use coord_2d::{Coord, Size};

    fn start() -> Coord {
        Coord::new(0, 0)
    }

    fn end() -> Coord {
        Coord::new(2, 2)
    }

    fn len() -> u32 {
        3
    }

    fn line_new() -> Line<Coord, Size> {
        Line::new(start(), end())
    }

    #[test]
    fn test_new() {
        let line = line_new();
        assert_eq!(line.start(), start());
        assert_eq!(line.end(), end());
    }

    #[test]
    fn test_len() {
        let line = line_new();
        assert_eq!(line.len(), len())
    }

    #[test]
    fn test_for_each() {
        let line = line_new();
        let mut points = Vec::new();
        line.for_each(|point| points.push(point));
        assert_eq!(
            points,
            vec![Coord::new(0, 0), Coord::new(1, 1), Coord::new(2, 2)]
        );
    }

    #[test]
    fn test_into_iter() {
        let line = line_new();
        let points: Vec<Coord> = line.into_iter().collect();
        assert_eq!(
            points,
            vec![Coord::new(0, 0), Coord::new(1, 1), Coord::new(2, 2)]
        );
    }
}
