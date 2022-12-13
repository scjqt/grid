A simple 2D grid library, including a generic heap-allocated 2D grid struct and a 2D vector struct.

For a position `Vector { x, y }` in the grid:
* `x` determines which column the position is in
* `y` determines which row the position is in

There are `width` columns and `height` rows in the grid, and the grid's iterators traverse it in row-major order.

# Examples

```
use grid::{Grid, vct};

let mut grid: Grid<u8> = Grid::new(8, 10, 0);

grid[vct!(1, 0)] = 1;
grid[vct!(3, 5)] = 2;

assert_eq!(grid[vct!(3, 5)], 2);
assert_eq!(grid[vct!(1, 0)], 1);
assert_eq!(grid[vct!(6, 4)], 0);

println!("{:?}", grid);
```

```
use grid::Vector;

let mut v = Vector::new(1, 2);

while v.x < 5 {
    v += Vector::new(1, 3);
}

v *= 2;

assert_eq!(v, Vector::new(10, 28));
```