# Grid Shapes

###Features
["extentions"] - enables some grid extentions
["serde"] - enables serialization

grid_shapes was born from the need of a dedicated shapes library for drawing shapes on a grid. (eg. drawing a room in a roguelike dungeon)

There are many shapes libraries but most use floating point math and I needed something simple and grid based.

Each shape is defined inclusively, for example a rectangle with min and max coordinates of (0,0) and (4,4) would return a size of [5,5] as both (0,0) and (4,4) are considered legitimate positions in the rectangle.

###Currently the included shapes are:
*Circle
*Line
*Rectangle

All shapes should `impl Shape` and `Shape` should remain object safe.

Accessing the coords which make up each shape is preferably done through iterators. A helper function `(Shape).for_each(|coord| {})` has been provided.

I am open to suggestions and pr's.

##ChangeLog
As of `0.1.3` we are now using `coord_2d`. This removes any generic shapes based on a `Coord` or `Size` trait, but allows `trait Shape` to become object safe.