use crate::{Coord, Size};

#[derive(Debug, Clone)]
pub struct RectangleIter<C: Coord> {
    offset: C,
    max_offset: C,
    position: C,
}

impl<C: Coord> RectangleIter<C> {
    pub fn new<S: Size>(position: C, size: S) -> Self {
        let max_offset = C::new(
            position.x() + size.width() as i32,
            position.y() + size.height() as i32,
        );

        Self {
            offset: C::new(0, 0),
            max_offset,
            position,
        }
    }
}

impl<C: Coord> Iterator for RectangleIter<C> {
    type Item = C;
    fn next(&mut self) -> Option<Self::Item> {
        if self.offset.y() >= self.max_offset.y() {
            return None;
        }

        let p = self.offset;

        self.offset.set_x(self.offset.x() + 1);

        if self.offset.x() >= self.max_offset.x() {
            self.offset.set_x(0);
            self.offset.set_y(self.offset.y() + 1);
        }

        Some(C::new(self.position.x() + p.x(), self.position.y() + p.y()))
    }
}

#[derive(Debug, Clone)]
pub struct RectangleBorderIter<C: Coord> {
    offset: C,
    max_offset: C,
    position: C,
}

impl<C: Coord> RectangleBorderIter<C> {
    pub fn new<S: Size>(position: C, size: S) -> Self {
        let max_offset = C::new(
            position.x() + size.width() as i32,
            position.y() + size.height() as i32,
        );

        Self {
            offset: C::new(0, 0),
            max_offset,
            position,
        }
    }
}

impl<C: Coord> Iterator for RectangleBorderIter<C> {
    type Item = C;
    fn next(&mut self) -> Option<Self::Item> {
        if self.offset.y() >= self.max_offset.y() {
            return None;
        }

        let p = self.offset;

        if self.offset.y() == 0 || self.offset.y() == self.max_offset.y() {
            self.offset.set_x(self.offset.x() + 1);

            if self.offset.x() >= self.max_offset.x() {
                self.offset.set_x(0);
                self.offset.set_y(self.offset.y() + 1);
            }
        } else {
            if self.offset.x() == 0 {
                self.offset.set_x(self.max_offset.x() - 1);
            } else {
                self.offset.set_x(0);
                self.offset.set_y(self.offset.y() + 1);
            }
        }

        Some(C::new(self.position.x() + p.x(), self.position.y() + p.y()))
    }
}
