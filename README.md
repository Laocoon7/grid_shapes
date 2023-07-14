# Grid Shapes

grid_shapes was born from the need of a dedicated shapes library for drawing shapes on a grid. (eg. drawing a room in a roguelike dungeon)

There are many shapes libraries but most use floating point math and I needed something simple. Each shape is defined inclusively for example a rectangle with min and max coordinates of (0,0) and (4,4) would return a size of [5,5] as both (0,0) and (4,4) are considered legitimate positions in the rectangle.

Currently the included shapes are:
Rectangle
Line
Circle

This list will grow over time, and the functions will become more standardized across each of them.

Each shape is a generic struct using traits to implement the Coord and Size. And the crate "coord_2d" can provide a default implementation with the "coord_2d" feature.

Accessing the coords which make up each shape is preferably done through iterators. A helper function `(shape).for_each(|coord| {})` has been provided.

I am open to suggestions and pr's.