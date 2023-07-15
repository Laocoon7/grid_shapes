use crate::Coord;

use super::LinearIter;

#[derive(Debug, Clone)]
pub struct TunnelVerticalHorizontalLineIter<C: Coord> {
    start: C,
    x: LinearIter,
    y: LinearIter,
    end: C,
}

impl<C: Coord> TunnelVerticalHorizontalLineIter<C> {
    pub fn new(start: C, end: C) -> Self {
        Self {
            start,
            x: LinearIter::new(start.x(), end.x()),
            y: LinearIter::new(start.y(), end.y()),
            end,
        }
    }
}

impl<C: Coord> Iterator for TunnelVerticalHorizontalLineIter<C> {
    type Item = C;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(y) = self.y.next() {
            Some(C::new(self.start.x(), y))
        } else if let Some(x) = self.x.next() {
            Some(C::new(x, self.end.y()))
        } else {
            None
        }
    }
}
