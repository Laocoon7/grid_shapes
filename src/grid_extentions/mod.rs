use coord_2d::Coord;
use grid_2d::Grid;

use crate::{prelude::*, Shape};

pub trait ShapeGridExtensions<T> {
    fn copy_from_shape(&mut self, shape: impl Shape<Coord>, offset: Coord, value: T);
    fn copy_from_rectangle_border(&mut self, rectangle: Rectangle, offset: Coord, value: T);
    fn copy_from_circle_circumference(&mut self, circle: Circle, offset: Coord, value: T);
    fn copy_from_line_tunnel_horizontal_vertical(&mut self, line: Line, offset: Coord, value: T);
    fn copy_from_line_tunnel_vertical_horizontal(&mut self, line: Line, offset: Coord, value: T);
}

impl<T: Copy> ShapeGridExtensions<T> for Grid<T> {
    fn copy_from_shape(&mut self, shape: impl Shape<Coord>, offset: Coord, value: T) {
        shape.for_each(|coord| {
            let position = Coord::new(coord.x + offset.x, coord.y + offset.y);
            if let Some(grid_value) = self.get_mut(position) {
                *grid_value = value;
            }
        });
    }

    fn copy_from_rectangle_border(&mut self, rectangle: Rectangle, offset: Coord, value: T) {
        rectangle.for_each(|coord| {
            let position = Coord::new(coord.x + offset.x, coord.y + offset.y);
            if let Some(grid_value) = self.get_mut(position) {
                *grid_value = value;
            }
        });
    }

    fn copy_from_circle_circumference(&mut self, circle: Circle, offset: Coord, value: T) {
        circle.for_each_circumference(|coord| {
            let position = Coord::new(coord.x + offset.x, coord.y + offset.y);
            if let Some(grid_value) = self.get_mut(position) {
                *grid_value = value;
            }
        });
    }

    fn copy_from_line_tunnel_horizontal_vertical(&mut self, line: Line, offset: Coord, value: T) {
        line.for_each_tunnel_horizontal_vertical(|coord| {
            let position = Coord::new(coord.x + offset.x, coord.y + offset.y);
            if let Some(grid_value) = self.get_mut(position) {
                *grid_value = value;
            }
        });
    }

    fn copy_from_line_tunnel_vertical_horizontal(&mut self, line: Line, offset: Coord, value: T) {
        line.for_each_tunnel_vertical_horizontal(|coord| {
            let position = Coord::new(coord.x + offset.x, coord.y + offset.y);
            if let Some(grid_value) = self.get_mut(position) {
                *grid_value = value;
            }
        });
    }
}
