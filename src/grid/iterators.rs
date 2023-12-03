//! Iterator types for iterating over a `Grid` and its positions.

use crate::{grid::Grid, vector::Vector};

use std::{
    iter::Zip,
    slice::{Iter, IterMut},
    vec::IntoIter,
};

impl<T> Grid<T> {
    /// Returns an iterator over references to the values in the grid, in row-major order.
    ///
    /// # Examples
    ///
    /// ```
    /// use grid::prelude::*;
    ///
    /// let mut grid: Grid<u8> = Grid::new(5, 5, 2);
    /// grid[v(1, 0)] = 3;
    ///
    /// let mut iter = grid.iter();
    ///
    /// assert_eq!(iter.next(), Some(&2));
    /// assert_eq!(iter.next(), Some(&3));
    ///
    /// let mut sum = 0;
    /// for value in grid.iter() {
    ///     sum += *value;
    /// }
    ///
    /// assert_eq!(sum, 51);
    /// assert_eq!(grid.iter().sum::<u8>(), 51);
    /// ```
    #[inline(always)]
    pub fn iter(&self) -> Iter<T> {
        self.raw.iter()
    }

    /// Returns an iterator over mutable references to the values in the grid, in row-major order.
    ///
    /// # Examples
    ///
    /// ```
    /// use grid::prelude::*;
    ///
    /// let mut grid: Grid<u8> = Grid::new(8, 10, 0);
    /// grid[v(1, 0)] = 2;
    ///
    /// let mut iter = grid.iter_mut();
    ///
    /// assert_eq!(iter.next(), Some(&mut 0));
    /// assert_eq!(iter.next(), Some(&mut 2));
    ///
    /// for value in grid.iter_mut() {
    ///     *value += 1;
    /// }
    ///
    /// assert_eq!(grid[v(1, 0)], 3);
    /// assert_eq!(grid[v(3, 5)], 1);
    /// ```
    #[inline(always)]
    pub fn iter_mut(&mut self) -> IterMut<T> {
        self.raw.iter_mut()
    }

    /// Returns an iterator over every position that can be used to index into the grid, in row-major order.
    ///
    /// # Examples
    ///
    /// ```
    /// use grid::prelude::*;
    ///
    /// let mut grid: Grid<u8> = Grid::new(3, 2, 0);
    ///
    /// let mut pos = grid.positions();
    ///
    /// assert_eq!(pos.next(), Some(v(0, 0)));
    /// assert_eq!(pos.next(), Some(v(1, 0)));
    /// assert_eq!(pos.next(), Some(v(2, 0)));
    /// assert_eq!(pos.next(), Some(v(0, 1)));
    /// assert_eq!(pos.next(), Some(v(1, 1)));
    /// assert_eq!(pos.next(), Some(v(2, 1)));
    /// assert_eq!(pos.next(), None);
    ///
    /// let mut pos = grid.positions();
    ///
    /// for value in grid.iter() {
    ///     assert_eq!(*value, grid[pos.next().unwrap()]);
    /// }
    /// ```
    pub fn positions(&self) -> Positions {
        Positions {
            pos: Vector::new(0, 0),
            dim: self.dim,
        }
    }

    /// Returns an iterator over every position and value in the grid, in row-major order.
    ///
    /// Values from this iterator come in the form of a tuple containing the position and a reference to the value:
    /// `(Vector, &T)`
    ///
    /// # Examples
    ///
    /// ```
    /// use grid::prelude::*;
    ///
    /// let grid: Grid<i64> = Grid::from_fn(8, 10, |pos| pos.x * 2 + pos.y);
    ///
    /// for (pos, value) in grid.iter_positions() {
    ///     assert_eq!(grid[pos], *value);
    ///     assert_eq!(*value, pos.x * 2 + pos.y);
    /// }
    /// ```
    pub fn iter_positions(&self) -> PositionIter<T> {
        PositionIter::new(self.positions().zip(self.iter()))
    }

    /// Returns an iterator over every position and value in the grid, in row-major order.
    ///
    /// Values from this iterator come in the form of a tuple containing the position and a mutable reference to the value:
    /// `(Vector, &mut T)`
    ///
    /// # Examples
    ///
    /// ```
    /// use grid::prelude::*;
    ///
    /// let mut grid: Grid<i64> = Grid::new(8, 10, 3);
    ///
    /// for (pos, value) in grid.iter_mut_positions() {
    ///     *value = pos.x * pos.y;
    /// }
    ///
    /// assert_eq!(grid[v(2, 3)], 6);
    /// assert_eq!(grid[v(7, 9)], 63);
    /// ```
    pub fn iter_mut_positions(&mut self) -> PositionIterMut<T> {
        PositionIterMut::new(self.positions().zip(self.iter_mut()))
    }

    /// Returns an iterator over every position and value in the grid, in row-major order, consuming the grid.
    ///
    /// Values from this iterator come in the form of a tuple containing the position and the value:
    /// `(Vector, T)`
    ///
    /// # Examples
    ///
    /// ```
    /// use grid::prelude::*;
    ///
    /// let grid: Grid<u8> = Grid::new(8, 10, 6);
    ///
    /// for (pos, value) in grid.into_iter_positions() {
    ///     assert_eq!(value, 6);
    ///     println!("{}: {}", pos, value);
    /// }
    /// ```
    pub fn into_iter_positions(self) -> PositionIntoIter<T> {
        PositionIntoIter::new(self.positions().zip(self.into_iter()))
    }
}

impl<T> IntoIterator for Grid<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    /// Returns an iterator over the values in the grid, in row-major order, consuming the grid.
    ///
    /// # Examples
    ///
    /// ```
    /// use grid::prelude::*;
    ///
    /// let mut grid: Grid<u8> = Grid::new(5, 5, 7);
    /// grid[v(1, 0)] = 5;
    ///
    /// let mut iter = grid.clone().into_iter();
    ///
    /// assert_eq!(iter.next(), Some(7));
    /// assert_eq!(iter.next(), Some(5));
    ///
    /// let mut sum = 0;
    /// for value in grid {
    ///     sum += value;
    /// }
    ///
    /// assert_eq!(sum, 173);
    /// ```
    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.raw.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a Grid<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    /// Returns an iterator over references to the values in the grid, in row-major order.
    ///
    /// # Examples
    ///
    /// ```
    /// use grid::prelude::*;
    ///
    /// let mut grid: Grid<u8> = Grid::new(5, 5, 2);
    /// grid[v(1, 0)] = 3;
    ///
    /// let mut sum = 0;
    /// for value in &grid {
    ///     sum += *value;
    /// }
    ///
    /// assert_eq!(sum, 51);
    /// ```
    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Grid<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    /// Returns an iterator over mutable references to the values in the grid, in row-major order.
    ///
    /// # Examples
    ///
    /// ```
    /// use grid::prelude::*;
    ///
    /// let mut grid: Grid<u8> = Grid::new(8, 10, 0);
    /// grid[v(1, 0)] = 2;
    ///
    /// for value in &mut grid {
    ///     *value += 1;
    /// }
    ///
    /// assert_eq!(grid[v(1, 0)], 3);
    /// assert_eq!(grid[v(3, 5)], 1);
    /// ```
    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

/// An iterator over every position that can be used to index into the grid, in row-major order.
///
/// # Examples
///
/// ```
/// use grid::prelude::*;
///
/// let mut grid: Grid<u8> = Grid::new(3, 2, 0);
///
/// let mut pos = grid.positions();
///
/// assert_eq!(pos.next(), Some(v(0, 0)));
/// assert_eq!(pos.next(), Some(v(1, 0)));
/// assert_eq!(pos.next(), Some(v(2, 0)));
/// assert_eq!(pos.next(), Some(v(0, 1)));
/// assert_eq!(pos.next(), Some(v(1, 1)));
/// assert_eq!(pos.next(), Some(v(2, 1)));
/// assert_eq!(pos.next(), None);
///
/// let mut pos = grid.positions();
///
/// for value in grid.iter() {
///     assert_eq!(*value, grid[pos.next().unwrap()]);
/// }
/// ```
#[derive(Clone, Copy)]
pub struct Positions {
    pos: Vector,
    dim: Vector,
}

impl Iterator for Positions {
    type Item = Vector;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos.y != self.dim.y {
            let pos = self.pos;
            self.pos.x += 1;
            if self.pos.x == self.dim.x {
                self.pos.x = 0;
                self.pos.y += 1;
            }
            return Some(pos);
        }
        None
    }
}

/// An iterator over every position and value in the grid, in row-major order.
///
/// Values from this iterator come in the form of a tuple containing the position and a reference to the value:
/// `(Vector, &T)`
///
/// # Examples
///
/// ```
/// use grid::prelude::*;
///
/// let grid: Grid<i64> = Grid::from_fn(8, 10, |pos| pos.x * 2 + pos.y);
///
/// for (pos, value) in grid.iter_positions() {
///     assert_eq!(grid[pos], *value);
///     assert_eq!(*value, pos.x * 2 + pos.y);
/// }
/// ```
pub struct PositionIter<'a, T> {
    iter: Zip<Positions, Iter<'a, T>>,
}

impl<'a, T> PositionIter<'a, T> {
    #[inline(always)]
    fn new(iter: Zip<Positions, Iter<'a, T>>) -> Self {
        Self { iter }
    }
}

impl<'a, T> Iterator for PositionIter<'a, T> {
    type Item = (Vector, &'a T);

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

/// An iterator over every position and value in the grid, in row-major order.
///
/// Values from this iterator come in the form of a tuple containing the position and a mutable reference to the value:
/// `(Vector, &mut T)`
///
/// # Examples
///
/// ```
/// use grid::prelude::*;
///
/// let mut grid: Grid<i64> = Grid::new(8, 10, 3);
///
/// for (pos, value) in grid.iter_mut_positions() {
///     *value = pos.x * pos.y;
/// }
///
/// assert_eq!(grid[v(2, 3)], 6);
/// assert_eq!(grid[v(7, 9)], 63);
/// ```
pub struct PositionIterMut<'a, T> {
    iter: Zip<Positions, IterMut<'a, T>>,
}

impl<'a, T> PositionIterMut<'a, T> {
    #[inline(always)]
    fn new(iter: Zip<Positions, IterMut<'a, T>>) -> Self {
        Self { iter }
    }
}

impl<'a, T> Iterator for PositionIterMut<'a, T> {
    type Item = (Vector, &'a mut T);

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

/// An iterator over every position and value in the grid, in row-major order.
///
/// Values from this iterator come in the form of a tuple containing the position and the value:
/// `(Vector, T)`
///
/// # Examples
///
/// ```
/// use grid::prelude::*;
///
/// let grid: Grid<u8> = Grid::new(8, 10, 6);
///
/// for (pos, value) in grid.into_iter_positions() {
///     assert_eq!(value, 6);
///     println!("{}: {}", pos, value);
/// }
/// ```
pub struct PositionIntoIter<T> {
    iter: Zip<Positions, IntoIter<T>>,
}

impl<T> PositionIntoIter<T> {
    #[inline(always)]
    fn new(iter: Zip<Positions, IntoIter<T>>) -> Self {
        Self { iter }
    }
}

impl<T> Iterator for PositionIntoIter<T> {
    type Item = (Vector, T);

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}
