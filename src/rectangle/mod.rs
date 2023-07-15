use std::fmt::{Debug, Display};

use crate::{
    iters::rectangle::{RectangleBorderIter, RectangleIter},
    units::{Coord, Shape, Size},
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Rectangle<C: Coord, S: Size> {
    pub position: C,
    pub size: S,
}

// Constructors
impl<C: Coord, S: Size> Rectangle<C, S> {
    pub fn from_size(position: C, size: S) -> Self {
        Self { position, size }
    }

    pub fn from_corners(position0: C, position1: C) -> Self {
        Self::new(position0.x(), position0.y(), position1.x(), position1.y())
    }

    pub fn new(x0: i32, y0: i32, x1: i32, y1: i32) -> Self {
        let min = C::new(x0.min(x1), y0.min(y1));
        let max = C::new(x0.max(x1), y0.max(y1));
        let size = S::new(
            (max.x() - min.x()).abs() as u32 + 1,
            (max.y() - min.y()).abs() as u32 + 1,
        );
        Self::from_size(min, size)
    }
}

// Implementation
impl<C: Coord, S: Size> Rectangle<C, S> {
    pub fn size(self) -> S {
        self.size
    }

    pub fn width(self) -> u32 {
        self.size.width()
    }

    pub fn height(self) -> u32 {
        self.size.height()
    }

    pub fn min(self) -> C {
        self.position
    }

    pub fn max(self) -> C {
        C::new(self.right(), self.top())
    }

    pub fn left(self) -> i32 {
        self.position.x()
    }

    pub fn right(self) -> i32 {
        self.position.x() + self.width() as i32 - 1
    }

    pub fn top(self) -> i32 {
        self.position.y() + self.height() as i32 - 1
    }

    pub fn bottom(self) -> i32 {
        self.position.y()
    }

    pub fn is_square(self) -> bool {
        self.width() == self.height()
    }

    pub fn intersects(self, other: Self) -> bool {
        self.left() <= other.right()
            && self.right() >= other.left()
            && self.bottom() <= other.top()
            && self.top() >= other.bottom()
    }
}

// Iterators
impl<C: Coord, S: Size> Rectangle<C, S> {
    pub fn border_iter(self) -> RectangleBorderIter<C> {
        RectangleBorderIter::new(self.position, self.size)
    }

    pub fn for_each_border<F: FnMut(C)>(self, mut f: F) {
        for coord in self.border_iter() {
            f(coord);
        }
    }
}

// Shape
impl<C: Coord, S: Size> Shape<C> for Rectangle<C, S> {
    fn for_each<F: FnMut(C)>(self, mut f: F) {
        for coord in self {
            f(coord);
        }
    }
}

impl<C: Coord, S: Size> IntoIterator for Rectangle<C, S> {
    type IntoIter = RectangleIter<C>;
    type Item = C;
    fn into_iter(self) -> Self::IntoIter {
        RectangleIter::new(self.position, self.size)
    }
}

impl<C: Coord, S: Size> Default for Rectangle<C, S> {
    fn default() -> Self {
        Self {
            position: Coord::new(0, 0),
            size: Size::new(1, 1),
        }
    }
}

impl<C: Coord, S: Size> Debug for Rectangle<C, S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Rectangle {{ position: ({}, {}), size: ({}, {}) }}",
            self.position.x(),
            self.position.y(),
            self.size.width(),
            self.size.height()
        )
    }
}

impl<C: Coord, S: Size> Display for Rectangle<C, S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Rectangle {{\n\tposition: ({}, {}),\n\tsize: ({}, {})\n}}",
            self.position.x(),
            self.position.y(),
            self.size.width(),
            self.size.height()
        )
    }
}

#[cfg(feature = "coord_2d")]
#[cfg(test)]
mod tests {
    use super::*;
    use coord_2d::{Coord, Size};

    fn size() -> Size {
        Size::new(5, 10)
    }

    fn min() -> Coord {
        Coord::new(-2, 0)
    }

    fn max() -> Coord {
        Coord::new(2, 9)
    }

    fn rect_from_size() -> Rectangle<Coord, Size> {
        Rectangle::from_size(min(), size())
    }

    fn rect_from_corners() -> Rectangle<Coord, Size> {
        Rectangle::<Coord, Size>::from_corners(min(), max())
    }

    fn rect_new() -> Rectangle<Coord, Size> {
        Rectangle::<Coord, Size>::new(min().x, min().y, max().x, max().y)
    }

    #[test]
    fn test_from_size() {
        let rect = rect_from_size();
        assert_eq!(rect.right(), max().x);
        assert_eq!(rect.top(), max().y);
    }

    #[test]
    fn test_from_corners() {
        let rect = rect_from_corners();
        assert_eq!(rect.size(), size());
    }

    #[test]
    fn test_new() {
        let rect = rect_new();
        assert_eq!(rect.size(), size());
    }

    #[test]
    fn test_size() {
        let rect = rect_from_size();
        assert_eq!(rect.size(), size());
    }

    #[test]
    fn test_width() {
        let rect = rect_from_size();
        assert_eq!(rect.width(), size().width());
    }

    #[test]
    fn test_height() {
        let rect = rect_from_size();
        assert_eq!(rect.height(), size().height());
    }

    #[test]
    fn test_min() {
        let rect = rect_from_size();
        assert_eq!(rect.min(), min());
    }

    #[test]
    fn test_max() {
        let rect = rect_from_size();
        assert_eq!(rect.max(), max());
    }

    #[test]
    fn test_left() {
        let rect = rect_from_size();
        assert_eq!(rect.left(), min().x);
    }

    #[test]
    fn test_right() {
        let rect = rect_from_size();
        assert_eq!(rect.right(), max().x);
    }

    #[test]
    fn test_top() {
        let rect = rect_from_size();
        assert_eq!(rect.top(), max().y);
    }

    #[test]
    fn test_bottom() {
        let rect = rect_from_size();
        assert_eq!(rect.bottom(), min().y);
    }

    #[test]
    fn test_is_square() {
        let rect = Rectangle::<Coord, Size>::new(0, 0, 10, 10);
        assert!(rect.is_square());

        let rect = Rectangle::<Coord, Size>::new(0, 0, 10, 20);
        assert!(!rect.is_square());
    }

    #[test]
    fn test_intersects() {
        let rect1 = Rectangle::<Coord, Size>::new(0, 0, 10, 10);
        let rect2 = Rectangle::<Coord, Size>::new(5, 5, 15, 15);
        assert!(rect1.intersects(rect2));

        let rect1 = Rectangle::<Coord, Size>::new(0, 0, 10, 10);
        let rect2 = Rectangle::<Coord, Size>::new(11, 11, 20, 20);
        assert!(!rect1.intersects(rect2));
    }

    #[test]
    fn test_for_each() {
        let rect = Rectangle::<Coord, Size>::new(0, 0, 1, 1);
        let mut points = Vec::new();
        rect.for_each(|point| points.push(point));
        assert_eq!(
            points,
            vec![
                Coord::new(0, 0),
                Coord::new(1, 0),
                Coord::new(0, 1),
                Coord::new(1, 1)
            ]
        );
    }

    #[test]
    fn test_into_iter() {
        let rect = Rectangle::<Coord, Size>::new(0, 0, 1, 1);
        let points: Vec<Coord> = rect.into_iter().collect();
        assert_eq!(
            points,
            vec![
                Coord::new(0, 0),
                Coord::new(1, 0),
                Coord::new(0, 1),
                Coord::new(1, 1),
            ]
        )
    }
}
