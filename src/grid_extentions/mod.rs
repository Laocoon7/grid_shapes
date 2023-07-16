use coord_2d::{Coord, Size};
use grid_2d::Grid;

use crate::{prelude::*, Shape};

pub trait ShapeGridExtensions<T> {
    fn get_from_shape(&self, shape: impl Shape<Coord, Size>, unused_value: T) -> Self;
    fn set_from_shape(&mut self, shape: impl Shape<Coord, Size>, value: T);
    fn get_from_shape_offset(&self, shape: impl Shape<Coord, Size>, offset: Coord, unused_value: T) -> Self;
    fn set_from_shape_offset(&mut self, shape: impl Shape<Coord, Size>, offset: Coord, value: T);
    
    fn get_from_rectangle_border(&self, rectangle: Rectangle, unused_value: T) -> Self;
    fn set_from_rectangle_border(&mut self, rectangle: Rectangle, value: T);
    fn get_from_rectangle_border_offset(&self, rectangle: Rectangle, offset: Coord, unused_value: T) -> Self;
    fn set_from_rectangle_border_offset(&mut self, rectangle: Rectangle, offset: Coord, value: T);

    fn get_from_circle_circumference(&self, circle: Circle, unused_value: T) -> Self;
    fn set_from_circle_circumference(&mut self, circle: Circle, value: T);
    fn get_from_circle_circumference_offset(&self, circle: Circle, offset: Coord, unused_value: T) -> Self;
    fn set_from_circle_circumference_offset(&mut self, circle: Circle, offset: Coord, value: T);

    fn get_from_line_tunnel_horizontal_vertical(&self, line: Line, unused_value: T) -> Self;
    fn set_from_line_tunnel_horizontal_vertical(&mut self, line: Line, value: T);
    fn get_from_line_tunnel_horizontal_vertical_offset(&self, line: Line, offset: Coord, unused_value: T) -> Self;
    fn set_from_line_tunnel_horizontal_vertical_offset(&mut self, line: Line, offset: Coord, value: T);

    fn get_from_line_tunnel_vertical_horizontal(&self, line: Line, unused_value: T) -> Self;
    fn set_from_line_tunnel_vertical_horizontal(&mut self, line: Line, value: T);
    fn get_from_line_tunnel_vertical_horizontal_offset(&self, line: Line, offset: Coord, unused_value: T) -> Self;
    fn set_from_line_tunnel_vertical_horizontal_offset(&mut self, line: Line, offset: Coord, value: T);
}

impl<T: Copy> ShapeGridExtensions<T> for Grid<T> {
    fn get_from_shape(&self, shape: impl Shape<Coord, Size>, unused_value: T) -> Self {
        let rectangle = shape.aabb();
        let mut grid = Grid::new_copy(rectangle.size, unused_value);

        shape.for_each(|coord| {
            let grid_position = Coord::new(coord.x - rectangle.left(), coord.y - rectangle.bottom());
            if let Some(grid_value) = self.get(coord) {
                *grid.get_checked_mut(grid_position) = *grid_value;
            }
        });

        grid
    }

    fn set_from_shape(&mut self, shape: impl Shape<Coord, Size>, value: T) {
        shape.for_each(|coord| {
            if let Some(grid_value) = self.get_mut(coord) {
                *grid_value = value;
            }
        });
    }

    fn get_from_shape_offset(&self, shape: impl Shape<Coord, Size>, offset: Coord, unused_value: T) -> Self {
        let rectangle = shape.aabb();
        let mut grid = Grid::new_copy(rectangle.size, unused_value);

        shape.for_each(|coord| {
            let grid_position = Coord::new(coord.x - rectangle.left(), coord.y - rectangle.bottom());
            let position = Coord::new(coord.x + offset.x, coord.y + offset.y);
            if let Some(grid_value) = self.get(position) {
                *grid.get_checked_mut(grid_position) = *grid_value;
            }
        });

        grid
    }
    
    fn set_from_shape_offset(&mut self, shape: impl Shape<Coord, Size>, offset: Coord, value: T) {
        shape.for_each(|coord| {
            let position = Coord::new(coord.x + offset.x, coord.y + offset.y);
            if let Some(grid_value) = self.get_mut(position) {
                *grid_value = value;
            }
        });
    }

    fn get_from_rectangle_border(&self, rectangle: Rectangle, unused_value: T) -> Self {
        let mut grid = Grid::new_copy(rectangle.size, unused_value);

        rectangle.for_each_border(|coord| {
            let grid_position = Coord::new(coord.x - rectangle.left(), coord.y - rectangle.bottom());
            if let Some(grid_value) = self.get(coord) {
                *grid.get_checked_mut(grid_position) = *grid_value;
            }
        });

        grid
    }

    fn set_from_rectangle_border(&mut self, rectangle: Rectangle, value: T) {
        rectangle.for_each_border(|coord| {
            if let Some(grid_value) = self.get_mut(coord) {
                *grid_value = value;
            }
        });
    }

    fn get_from_rectangle_border_offset(&self, rectangle: Rectangle, offset: Coord, unused_value: T) -> Self {
        let mut grid = Grid::new_copy(rectangle.size, unused_value);

        rectangle.for_each_border(|coord| {
            let grid_position = Coord::new(coord.x - rectangle.left(), coord.y - rectangle.bottom());
            let position = Coord::new(coord.x + offset.x, coord.y + offset.y);
            if let Some(grid_value) = self.get(position) {
                *grid.get_checked_mut(grid_position) = *grid_value;
            }
        });

        grid
    }

    fn set_from_rectangle_border_offset(&mut self, rectangle: Rectangle, offset: Coord, value: T) {
        rectangle.for_each_border(|coord| {
            let position = Coord::new(coord.x + offset.x, coord.y + offset.y);
            if let Some(grid_value) = self.get_mut(position) {
                *grid_value = value;
            }
        });
    }

    fn get_from_circle_circumference(&self, circle: Circle, unused_value: T) -> Self {
        let rectangle = circle.aabb();
        let mut grid = Grid::new_copy(rectangle.size, unused_value);

        circle.for_each_circumference(|coord| {
            let grid_position = Coord::new(coord.x - rectangle.left(), coord.y - rectangle.bottom());
            if let Some(grid_value) = self.get(coord) {
                *grid.get_checked_mut(grid_position) = *grid_value;
            }
        });

        grid
    }

    fn set_from_circle_circumference(&mut self, circle: Circle, value: T) {
        circle.for_each_circumference(|coord| {
            if let Some(grid_value) = self.get_mut(coord) {
                *grid_value = value;
            }
        });
    }

    fn get_from_circle_circumference_offset(&self, circle: Circle, offset: Coord, unused_value: T) -> Self {
        let rectangle = circle.aabb();
        let mut grid = Grid::new_copy(rectangle.size, unused_value);

        circle.for_each_circumference(|coord| {
            let grid_position = Coord::new(coord.x - rectangle.left(), coord.y - rectangle.bottom());
            let position = Coord::new(coord.x + offset.x, coord.y + offset.y);
            if let Some(grid_value) = self.get(position) {
                *grid.get_checked_mut(grid_position) = *grid_value;
            }
        });

        grid
    }

    fn set_from_circle_circumference_offset(&mut self, circle: Circle, offset: Coord, value: T) {
        circle.for_each_circumference(|coord| {
            let position = Coord::new(coord.x + offset.x, coord.y + offset.y);
            if let Some(grid_value) = self.get_mut(position) {
                *grid_value = value;
            }
        });
    }

    fn get_from_line_tunnel_horizontal_vertical(&self, line: Line, unused_value: T) -> Self {
        let rectangle = line.aabb();
        let mut grid = Grid::new_copy(rectangle.size, unused_value);

        line.for_each_tunnel_horizontal_vertical(|coord| {
            let grid_position = Coord::new(coord.x - rectangle.left(), coord.y - rectangle.bottom());
            if let Some(grid_value) = self.get(coord) {
                *grid.get_checked_mut(grid_position) = *grid_value;
            }
        });

        grid
    }

    fn set_from_line_tunnel_horizontal_vertical(&mut self, line: Line, value: T) {
        line.for_each_tunnel_horizontal_vertical(|coord| {
            if let Some(grid_value) = self.get_mut(coord) {
                *grid_value = value;
            }
        });
    }

    fn get_from_line_tunnel_horizontal_vertical_offset(&self, line: Line, offset: Coord, unused_value: T) -> Self {
        let rectangle = line.aabb();
        let mut grid = Grid::new_copy(rectangle.size, unused_value);

        line.for_each_tunnel_horizontal_vertical(|coord| {
            let grid_position = Coord::new(coord.x - rectangle.left(), coord.y - rectangle.bottom());
            let position = Coord::new(coord.x + offset.x, coord.y + offset.y);
            if let Some(grid_value) = self.get(position) {
                *grid.get_checked_mut(grid_position) = *grid_value;
            }
        });

        grid
    }

    fn set_from_line_tunnel_horizontal_vertical_offset(&mut self, line: Line, offset: Coord, value: T) {
        line.for_each_tunnel_horizontal_vertical(|coord| {
            let position = Coord::new(coord.x + offset.x, coord.y + offset.y);
            if let Some(grid_value) = self.get_mut(position) {
                *grid_value = value;
            }
        });
    }

    fn get_from_line_tunnel_vertical_horizontal(&self, line: Line, unused_value: T) -> Self {
        let rectangle = line.aabb();
        let mut grid = Grid::new_copy(rectangle.size, unused_value);

        line.for_each_tunnel_vertical_horizontal(|coord| {
            let grid_position = Coord::new(coord.x - rectangle.left(), coord.y - rectangle.bottom());
            if let Some(grid_value) = self.get(coord) {
                *grid.get_checked_mut(grid_position) = *grid_value;
            }
        });

        grid
    }

    fn set_from_line_tunnel_vertical_horizontal(&mut self, line: Line, value: T) {
        line.for_each_tunnel_vertical_horizontal(|coord| {
            if let Some(grid_value) = self.get_mut(coord) {
                *grid_value = value;
            }
        });
    }

    fn get_from_line_tunnel_vertical_horizontal_offset(&self, line: Line, offset: Coord, unused_value: T) -> Self {
        let rectangle = line.aabb();
        let mut grid = Grid::new_copy(rectangle.size, unused_value);

        line.for_each_tunnel_vertical_horizontal(|coord| {
            let grid_position = Coord::new(coord.x - rectangle.left(), coord.y - rectangle.bottom());
            let position = Coord::new(coord.x + offset.x, coord.y + offset.y);
            if let Some(grid_value) = self.get(position) {
                *grid.get_checked_mut(grid_position) = *grid_value;
            }
        });

        grid
    }

    fn set_from_line_tunnel_vertical_horizontal_offset(&mut self, line: Line, offset: Coord, value: T) {
        line.for_each_tunnel_vertical_horizontal(|coord| {
            let position = Coord::new(coord.x + offset.x, coord.y + offset.y);
            if let Some(grid_value) = self.get_mut(position) {
                *grid_value = value;
            }
        });
    }


}
