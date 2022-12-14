//! A simple 2D grid library, including a generic heap-allocated 2D grid struct and a 2D vector struct.
//!
//! For a position `Vector { x, y }` in the grid:
//! * `x` determines which column the position is in
//! * `y` determines which row the position is in
//!
//! There are `width` columns and `height` rows in the grid, and the grid's iterators traverse it in row-major order.
//!
//! # Examples
//!
//! ```
//! use grid::prelude::*;
//!
//! let mut grid: Grid<u8> = Grid::new(5, 6, 3);
//!
//! assert_eq!(grid.width(), 5);
//! assert_eq!(grid.height(), 6);
//!
//! grid[v!(1, 0)] = 1;
//! grid[v!(3, 5)] = 2;
//!
//! assert_eq!(grid[v!(3, 5)], 2);
//! assert_eq!(grid[v!(1, 0)], 1);
//! assert_eq!(grid[v!(2, 4)], 3);
//!
//! println!("{:?}", grid);
//!
//! let mut pos = Vector::new(1, 2);
//! let mut offset = EAST;
//!
//! while grid.in_bounds(pos) {
//!     pos += offset;
//! }
//!
//! assert_eq!(pos, v!(5, 2));
//! ```

pub mod grid;
pub mod vector;

pub mod prelude {
    pub use crate::grid::Grid;
    pub use crate::vector::constants::*;
    pub use crate::vector::v;
    pub use crate::vector::Vector;
}
