//! A simple generic heap-allocated 2D grid library.
//!
//! For a position `(x, y)` in the grid:
//! * `x`, the first value, determines which column the position is in
//! * `y`, the second value, determines which row the position is in
//!
//! There are `width` columns and `height` rows in the grid, and the grid's iterators traverse it in row-major order.
//!
//! Implements the [`Debug`] trait if `T` implements the [`Display`] trait.
//!
//! Indexable mutably and immutably by `(T, T)` if `T: TryInto<usize>`.
//!
//! # Examples
//!
//! ```
//! use grid::Grid;
//!
//! let mut grid: Grid<u8> = Grid::new(8, 10, 0);
//!
//! grid[(1, 0)] = 1;
//! grid[(3, 5)] = 2;
//!
//! assert_eq!(grid[(3, 5)], 2);
//! assert_eq!(grid[(1, 0)], 1);
//! assert_eq!(grid[(6, 4)], 0);
//!
//! println!("{:?}", grid);
//! ```

pub mod iterators;

use std::{
    fmt,
    ops::{Index, IndexMut},
};

/// A simple generic heap-allocated 2D grid struct.
///
/// For a position `(x, y)` in the grid:
/// * `x`, the first value, determines which column the position is in
/// * `y`, the second value, determines which row the position is in
///
/// There are `width` columns and `height` rows in the grid, and the grid's iterators traverse it in row-major order.
///
/// Implements the [`Debug`] trait if `T` implements the [`Display`] trait.
///
/// Indexable mutably and immutably by `(T, T)` if `T: TryInto<usize>`.
///
/// # Examples
///
/// ```
/// use grid::Grid;
///
/// let mut grid: Grid<u8> = Grid::new(8, 10, 0);
///
/// grid[(1, 0)] = 1;
/// grid[(3, 5)] = 2;
///
/// assert_eq!(grid[(3, 5)], 2);
/// assert_eq!(grid[(1, 0)], 1);
/// assert_eq!(grid[(6, 4)], 0);
///
/// println!("{:?}", grid);
/// ```
#[derive(PartialEq, Eq, Clone, Hash)]
pub struct Grid<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl<T: Clone> Grid<T> {
    /// Constructs a new `Grid<T>` with the given dimensions, initialising all values to `value`.
    ///
    /// Requires that `T` implements the [`Clone`] trait.
    ///
    /// # Examples
    ///
    /// ```
    /// use grid::Grid;
    ///
    /// let grid: Grid<u8> = Grid::new(8, 10, 1);
    ///
    /// assert_eq!(grid[(2, 4)], 1);
    /// assert_eq!(grid[(7, 3)], 1);
    /// ```
    pub fn new(width: usize, height: usize, value: T) -> Grid<T> {
        let size = size(width, height);
        let mut data = Vec::with_capacity(size);
        data.resize(size, value);
        Grid {
            data,
            width,
            height,
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
    /// use grid::Grid;
    ///
    /// let grid: Grid<u8> = Grid::default(9, 3);
    ///
    /// assert_eq!(grid[(5, 1)], 0);
    /// assert_eq!(grid[(6, 2)], 0);
    /// ```
    pub fn default(width: usize, height: usize) -> Grid<T> {
        let size = size(width, height);
        let mut data = Vec::with_capacity(size);
        data.resize_with(size, Default::default);
        Grid {
            data,
            width,
            height,
        }
    }
}

impl<T> Grid<T> {
    /// Constructs a new `Grid<T>` with the given dimensions, computing all initial values from the closure `f`.
    ///
    /// If `T` implements the [`Default`] trait, `Default::default` can be passed in.
    ///
    /// # Examples
    ///
    /// ```
    /// use grid::Grid;
    ///
    /// let grid: Grid<u8> = Grid::from_simple_fn(8, 10, || 2);
    ///
    /// assert_eq!(grid[(5, 3)], 2);
    /// assert_eq!(grid[(1, 8)], 2);
    ///
    /// let grid: Grid<u8> = Grid::from_simple_fn(8, 10, Default::default);
    ///
    /// assert_eq!(grid[(0, 3)], 0);
    /// assert_eq!(grid[(6, 4)], 0);
    /// ```
    pub fn from_simple_fn<F>(width: usize, height: usize, f: F) -> Grid<T>
    where
        F: FnMut() -> T,
    {
        let size = size(width, height);
        let mut data = Vec::with_capacity(size);
        data.resize_with(size, f);
        Grid {
            data,
            width,
            height,
        }
    }
    /// Constructs a new `Grid<T>` with the given dimensions, computing all initial values from the closure `f` which maps each position to a value.
    ///
    /// # Examples
    ///
    /// ```
    /// use grid::Grid;
    ///
    /// let grid: Grid<usize> = Grid::from_fn(8, 10, |(x, y)| x + y);
    ///
    /// assert_eq!(grid[(5, 3)], 8);
    /// assert_eq!(grid[(7, 9)], 16);
    /// ```
    pub fn from_fn<F>(width: usize, height: usize, mut f: F) -> Grid<T>
    where
        F: FnMut((usize, usize)) -> T,
    {
        let mut data = Vec::with_capacity(width.checked_mul(height).expect("dimensions too large"));
        for y in 0..height {
            for x in 0..width {
                data.push(f((x, y)));
            }
        }
        Grid {
            data,
            width,
            height,
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
    pub fn width(&self) -> usize {
        self.width
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
    pub fn height(&self) -> usize {
        self.height
    }

    /// Returns a reference to the value at the given position of the grid, or `None` if out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use grid::Grid;
    ///
    /// let mut grid: Grid<u8> = Grid::new(8, 10, 3);
    ///
    /// grid[(1, 1)] = 4;
    ///
    /// assert_eq!(grid.get((5, 2)), Some(&3));
    /// assert_eq!(grid.get((1, 1)), Some(&4));
    /// assert_eq!(grid.get((8, 6)), None);
    /// assert_eq!(grid.get((4, 10)), None);
    /// ```
    pub fn get<C: TryInto<usize>>(&self, pos: (C, C)) -> Option<&T> {
        let (x, y) = self.get_pos(pos)?;
        Some(&self.data[x + y * self.width])
    }

    /// Returns a mutable reference to the value at the given position of the grid, or `None` if out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use grid::Grid;
    ///
    /// let mut grid: Grid<u8> = Grid::new(8, 10, 4);
    ///
    /// grid[(5, 3)] = 2;
    ///
    /// assert_eq!(grid.get_mut((5, 3)), Some(&mut 2));
    /// assert_eq!(grid.get_mut((0, 0)), Some(&mut 4));
    /// assert_eq!(grid.get_mut((1, 10)), None);
    /// assert_eq!(grid.get_mut((9, 7)), None);
    /// ```
    pub fn get_mut<C: TryInto<usize>>(&mut self, pos: (C, C)) -> Option<&mut T> {
        let (x, y) = self.get_pos(pos)?;
        Some(&mut self.data[x + y * self.width])
    }

    /// Sets the value at the given position of the grid.
    ///
    /// Returns the old value at that position, or `None` if out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use grid::Grid;
    ///
    /// let mut grid: Grid<u8> = Grid::new(8, 10, 5);
    ///
    /// assert_eq!(grid.set((2, 3), 7), Some(5));
    /// assert_eq!(grid.set((9, 12), 1), None);
    ///
    /// assert_eq!(grid[(2, 3)], 7);
    /// ```
    pub fn set<C: TryInto<usize>>(&mut self, pos: (C, C), value: T) -> Option<T> {
        let (x, y) = self.get_pos(pos)?;
        Some(std::mem::replace(&mut self.data[x + y * self.width], value))
    }

    /// Returns `true` if the given position is within the bounds of the grid, or `false` if not.
    ///
    /// # Examples
    ///
    /// ```
    /// use grid::Grid;
    ///
    /// let grid: Grid<u8> = Grid::new(15, 14, 11);
    ///
    /// assert_eq!(grid.in_bounds((0, 0)), true);
    /// assert_eq!(grid.in_bounds((10, 4)), true);
    /// assert_eq!(grid.in_bounds((15, 2)), false);
    /// assert_eq!(grid.in_bounds((3, 17)), false);
    /// assert_eq!(grid.in_bounds((-1, 5)), false);
    /// ```
    pub fn in_bounds<C: TryInto<usize>>(&self, pos: (C, C)) -> bool {
        self.get_pos(pos).is_some()
    }

    fn get_pos<C: TryInto<usize>>(&self, pos: (C, C)) -> Option<(usize, usize)> {
        let (x, y): (usize, usize) = (pos.0.try_into().ok()?, pos.1.try_into().ok()?);
        if x >= self.width || y >= self.height {
            return None;
        }
        Some((x, y))
    }
}

impl<T, C: TryInto<usize>> Index<(C, C)> for Grid<T> {
    type Output = T;

    fn index(&self, index: (C, C)) -> &Self::Output {
        self.get(index).expect("position out of bounds")
    }
}

impl<T, C: TryInto<usize>> IndexMut<(C, C)> for Grid<T> {
    fn index_mut(&mut self, index: (C, C)) -> &mut Self::Output {
        self.get_mut(index).expect("position out of bounds")
    }
}

impl<T: fmt::Display> fmt::Debug for Grid<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let mut longest = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                longest = longest.max(self[(x, y)].to_string().len());
            }
        }

        writeln!(f, "{}x{}", self.width, self.height)?;

        for y in 0..self.height {
            for x in 0..self.width {
                let str = self[(x, y)].to_string();
                write!(f, "{}{}", " ".repeat(longest - str.len()), str)?;
                if x != self.width - 1 {
                    write!(f, ",")?;
                }
            }
            if y != self.height - 1 {
                writeln!(f)?;
            }
        }

        Ok(())
    }
}

fn size(width: usize, height: usize) -> usize {
    width.checked_mul(height).expect("dimensions too large")
}
