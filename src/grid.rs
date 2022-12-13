pub mod iterators;

use crate::Vector;

use std::{
    fmt,
    ops::{Index, IndexMut},
};

/// A simple generic heap-allocated 2D grid struct indexed by `Vector`.
///
/// For a position `Vector { x, y }` in the grid:
/// * `x` determines which column the position is in
/// * `y` determines which row the position is in
///
/// There are `width` columns and `height` rows in the grid, and the grid's iterators traverse it in row-major order.
///
/// `Grid<T>` implements the [`Debug`] trait if `T` implements the [`Display`] trait.
///
/// # Examples
///
/// ```
/// use grid::{Grid, vct};
///
/// let mut grid: Grid<u8> = Grid::new(8, 10, 0);
///
/// grid[vct!(1, 0)] = 1;
/// grid[vct!(3, 5)] = 2;
///
/// assert_eq!(grid[vct!(3, 5)], 2);
/// assert_eq!(grid[vct!(1, 0)], 1);
/// assert_eq!(grid[vct!(6, 4)], 0);
///
/// println!("{:?}", grid);
/// ```
#[derive(PartialEq, Eq, Clone, Hash)]
pub struct Grid<T> {
    data: Vec<T>,
    dim: Vector,
}

impl<T: Clone> Grid<T> {
    /// Constructs a new `Grid<T>` with the given dimensions, initialising all values to `value`.
    ///
    /// Requires that `T` implements the [`Clone`] trait.
    ///
    /// # Examples
    ///
    /// ```
    /// use grid::{Grid, vct};
    ///
    /// let grid: Grid<u8> = Grid::new(8, 10, 1);
    ///
    /// assert_eq!(grid[vct!(2, 4)], 1);
    /// assert_eq!(grid[vct!(7, 3)], 1);
    /// ```
    pub fn new(width: i64, height: i64, value: T) -> Self {
        let size = size(width, height);
        let mut data = Vec::with_capacity(size);
        data.resize(size, value);
        Self {
            data,
            dim: Vector::new(width, height),
        }
    }
}

impl<T: Default> Grid<T> {
    /// Constructs a new `Grid<T>` with the given dimensions, initialising all values to their default value.
    ///
    /// Requires that `T` implements the [`Default`] trait.
    ///
    /// # Examples
    ///
    /// ```
    /// use grid::{Grid, vct};
    ///
    /// let grid: Grid<u8> = Grid::default(9, 3);
    ///
    /// assert_eq!(grid[vct!(5, 1)], 0);
    /// assert_eq!(grid[vct!(6, 2)], 0);
    /// ```
    pub fn default(width: i64, height: i64) -> Self {
        let size = size(width, height);
        let mut data = Vec::with_capacity(size);
        data.resize_with(size, Default::default);
        Self {
            data,
            dim: Vector::new(width, height),
        }
    }
}

impl<T> Grid<T> {
    /// Constructs a new `Grid<T>` with the given dimensions, computing all initial values from the closure `f`.
    ///
    /// # Examples
    ///
    /// ```
    /// use grid::{Grid, vct};
    ///
    /// let grid: Grid<u8> = Grid::from_simple_fn(8, 10, || 2);
    ///
    /// assert_eq!(grid[vct!(5, 3)], 2);
    /// assert_eq!(grid[vct!(1, 8)], 2);
    /// ```
    pub fn from_simple_fn<F>(width: i64, height: i64, f: F) -> Self
    where
        F: FnMut() -> T,
    {
        let size = size(width, height);
        let mut data = Vec::with_capacity(size);
        data.resize_with(size, f);
        Self {
            data,
            dim: Vector::new(width, height),
        }
    }
    /// Constructs a new `Grid<T>` with the given dimensions, computing all initial values from the closure `f` which maps each position to a value.
    ///
    /// # Examples
    ///
    /// ```
    /// use grid::{Grid, vct};
    ///
    /// let grid: Grid<i64> = Grid::from_fn(8, 10, |pos| pos.x + pos.y);
    ///
    /// assert_eq!(grid[vct!(5, 3)], 8);
    /// assert_eq!(grid[vct!(7, 9)], 16);
    /// ```
    pub fn from_fn<F>(width: i64, height: i64, mut f: F) -> Self
    where
        F: FnMut(Vector) -> T,
    {
        let mut data = Vec::with_capacity(size(width, height));
        for y in 0..height {
            for x in 0..width {
                data.push(f(Vector::new(x, y)));
            }
        }
        Self {
            data,
            dim: Vector::new(width, height),
        }
    }

    /// Returns the width of the grid.
    ///
    /// # Examples
    ///
    /// ```
    /// use grid::Grid;
    ///
    /// let grid: Grid<u8> = Grid::new(8, 10, 9);
    ///
    /// assert_eq!(grid.width(), 8);
    /// ```
    #[inline]
    pub fn width(&self) -> i64 {
        self.dim.x
    }

    /// Returns the height of the grid.
    ///
    /// # Examples
    ///
    /// ```
    /// use grid::Grid;
    ///
    /// let grid: Grid<u8> = Grid::new(8, 10, 10);
    ///
    /// assert_eq!(grid.height(), 10);
    /// ```
    #[inline]
    pub fn height(&self) -> i64 {
        self.dim.y
    }

    /// Returns the dimensions of the grid.
    ///
    /// # Examples
    ///
    /// ```
    /// use grid::{Grid, vct};
    ///
    /// let grid: Grid<u8> = Grid::new(8, 10, 11);
    ///
    /// assert_eq!(grid.dim(), vct!(8, 10));
    /// ```
    #[inline]
    pub fn dim(&self) -> Vector {
        self.dim
    }

    /// Returns a reference to the value at the given position of the grid, or `None` if out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use grid::{Grid, vct};
    ///
    /// let mut grid: Grid<u8> = Grid::new(8, 10, 3);
    ///
    /// grid[vct!(1, 1)] = 4;
    ///
    /// assert_eq!(grid.get(vct!(5, 2)), Some(&3));
    /// assert_eq!(grid.get(vct!(1, 1)), Some(&4));
    /// assert_eq!(grid.get(vct!(8, 6)), None);
    /// assert_eq!(grid.get(vct!(4, 10)), None);
    /// assert_eq!(grid.get(vct!(-2, 3)), None);
    /// ```
    pub fn get(&self, pos: Vector) -> Option<&T> {
        Some(&self.data[self.get_index(pos)?])
    }

    /// Returns a mutable reference to the value at the given position of the grid, or `None` if out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use grid::{Grid, vct};
    ///
    /// let mut grid: Grid<u8> = Grid::new(8, 10, 4);
    ///
    /// grid[vct!(5, 3)] = 2;
    ///
    /// assert_eq!(grid.get_mut(vct!(5, 3)), Some(&mut 2));
    /// assert_eq!(grid.get_mut(vct!(0, 0)), Some(&mut 4));
    /// assert_eq!(grid.get_mut(vct!(1, 10)), None);
    /// assert_eq!(grid.get_mut(vct!(9, 7)), None);
    /// assert_eq!(grid.get_mut(vct!(4, -1)), None);
    /// ```
    pub fn get_mut(&mut self, pos: Vector) -> Option<&mut T> {
        let index = self.get_index(pos)?;
        Some(&mut self.data[index])
    }

    /// Sets the value at the given position of the grid.
    ///
    /// Returns the old value at that position, or `None` if out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use grid::{Grid, vct};
    ///
    /// let mut grid: Grid<u8> = Grid::new(8, 10, 5);
    ///
    /// assert_eq!(grid.set(vct!(2, 3), 7), Some(5));
    /// assert_eq!(grid.set(vct!(9, 12), 1), None);
    /// assert_eq!(grid.set(vct!(-4, -7), 3), None);
    ///
    /// assert_eq!(grid[vct!(2, 3)], 7);
    /// ```
    pub fn set(&mut self, pos: Vector, value: T) -> Option<T> {
        Some(std::mem::replace(self.get_mut(pos)?, value))
    }

    /// Returns `true` if the given position is within the bounds of the grid, or `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use grid::{Grid, vct};
    ///
    /// let grid: Grid<u8> = Grid::new(15, 14, 11);
    ///
    /// assert_eq!(grid.in_bounds(vct!(0, 0)), true);
    /// assert_eq!(grid.in_bounds(vct!(10, 4)), true);
    /// assert_eq!(grid.in_bounds(vct!(15, 2)), false);
    /// assert_eq!(grid.in_bounds(vct!(3, 17)), false);
    /// assert_eq!(grid.in_bounds(vct!(-1, 5)), false);
    /// assert_eq!(grid.in_bounds(vct!(-15, -14)), false);
    /// ```
    pub fn in_bounds(&self, pos: Vector) -> bool {
        (0..self.width()).contains(&pos.x) && (0..self.height()).contains(&pos.y)
    }

    fn get_index(&self, pos: Vector) -> Option<usize> {
        self.in_bounds(pos)
            .then(|| pos.x as usize + ((pos.y as usize) * (self.width() as usize)))
    }

    /// Maps the values of an existing grid to create a new grid with the same dimensions.
    ///
    /// The closure `f` takes a reference to a value.
    ///
    /// # Examples
    ///
    /// ```
    /// use grid::{Grid, vct};
    ///
    /// let grid_a: Grid<u8> = Grid::new(15, 14, 11);
    ///
    /// let grid_b = grid_a.map(|value| *value + 2);
    ///
    /// assert_eq!(grid_b[vct!(2, 3)], 13);
    ///
    /// let grid_c = grid_b.map(ToString::to_string);
    ///
    /// assert_eq!(&grid_c[vct!(2, 3)], "13");
    /// ```
    pub fn map<F, U>(&self, mut f: F) -> Grid<U>
    where
        F: FnMut(&T) -> U,
    {
        let mut data = Vec::with_capacity(self.data.capacity());
        for value in self {
            data.push(f(value));
        }
        Grid {
            data,
            dim: self.dim,
        }
    }

    /// Maps the values of an existing grid to create a new grid with the same dimensions.
    ///
    /// The closure `f` takes a position in the grid and a reference to the value at that position.
    ///
    /// # Examples
    ///
    /// ```
    /// use grid::{Grid, vct};
    ///
    /// let grid_a: Grid<i64> = Grid::new(5, 6, 3);
    ///
    /// let grid_b = grid_a.pos_map(|pos, value| *value + pos.x);
    ///
    /// assert_eq!(grid_b[vct!(1, 4)], 4);
    /// assert_eq!(grid_b[vct!(3, 0)], 6);
    ///
    /// let grid_c = grid_b.pos_map(|pos, value| *value + pos.y);
    ///
    /// assert_eq!(grid_c[vct!(1, 4)], 8);
    /// ```
    pub fn pos_map<F, U>(&self, mut f: F) -> Grid<U>
    where
        F: FnMut(Vector, &T) -> U,
    {
        let mut data = Vec::with_capacity(self.data.capacity());
        for (pos, value) in self.iter_positions() {
            data.push(f(pos, value));
        }
        Grid {
            data,
            dim: self.dim,
        }
    }
}

impl<T> Index<Vector> for Grid<T> {
    type Output = T;

    fn index(&self, index: Vector) -> &Self::Output {
        self.get(index).expect("position out of bounds")
    }
}

impl<T> IndexMut<Vector> for Grid<T> {
    fn index_mut(&mut self, index: Vector) -> &mut Self::Output {
        self.get_mut(index).expect("position out of bounds")
    }
}

impl<T: fmt::Display> fmt::Debug for Grid<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let strings = self.map(ToString::to_string);
        let longest = strings.iter().map(String::len).max().unwrap();

        writeln!(f, "{}x{}", self.width(), self.height())?;

        for y in 0..strings.height() {
            for x in 0..strings.width() {
                let s = &strings[Vector::new(x, y)];
                write!(f, "{}{s}", " ".repeat(longest - s.len()))?;
                if x != strings.width() - 1 {
                    write!(f, ",")?;
                }
            }
            if y != strings.height() - 1 {
                writeln!(f)?;
            }
        }

        Ok(())
    }
}

fn size(width: i64, height: i64) -> usize {
    if width <= 0 || height <= 0 {
        panic!("dimensions must be positive");
    }
    (width as usize)
        .checked_mul(height as usize)
        .expect("dimensions too large")
}
