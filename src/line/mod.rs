use crate::{Coord, iters::line::BresenhamLineIter};

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
    pub fn len(self) -> u32 {
        (self.end.x() - self.start.x()).abs().max((self.end.y() - self.start.y()).abs()) as u32 + 1
    }

    pub fn start(self) -> C {
        self.start
    }

    pub fn end(self) -> C {
        self.end
    }
}

// Iterators
impl<C: Coord> Line<C> {
    pub fn for_each<F: FnMut(C)>(self, mut f: F) {
        for coord in self {
            f(coord);
        }
    }
}

impl<C: Coord> IntoIterator for Line<C> {
    type IntoIter = BresenhamLineIter<C>;
    type Item = C;
    fn into_iter(self) -> Self::IntoIter {
        BresenhamLineIter::new(self.start, self.end)
    }
}

impl<C: Coord> Default for Line<C> {
    fn default() -> Self {
        Self {
            start: C::new(0, 0),
            end: C::new(1, 0),
        }
    }
}

#[cfg(feature = "coord_2d")]
#[cfg(test)]
mod test {
    use super::*;
    use coord_2d::Coord;

    fn start() -> Coord {
        Coord::new(0, 0)
    }

    fn end() -> Coord {
        Coord::new(2, 2)
    }

    fn len() -> u32 {
        3
    }

    fn line_new() -> Line<Coord> {
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
            vec![
                Coord::new(0, 0),
                Coord::new(1, 1),
                Coord::new(2, 2)
            ]
        );
    }

    #[test]
    fn test_into_iter() {
        let line = line_new();
        let points: Vec<Coord> = line.into_iter().collect();
        assert_eq!(
            points,
            vec![
                Coord::new(0, 0),
                Coord::new(1, 1),
                Coord::new(2, 2)
            ]
        );
    }

}