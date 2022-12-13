use crate::Grid;

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
    /// use grid::Grid;
    ///
    /// let mut grid: Grid<u8> = Grid::new(5, 5, 2);
    /// grid[(1, 0)] = 3;
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
    pub fn iter(&self) -> Iter<T> {
        self.data.iter()
    }

    /// Returns an iterator over mutable references to the values in the grid, in row-major order.
    ///
    /// # Examples
    ///
    /// ```
    /// use grid::Grid;
    ///
    /// let mut grid: Grid<u8> = Grid::new(8, 10, 0);
    /// grid[(1, 0)] = 2;
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
    /// assert_eq!(grid[(1, 0)], 3);
    /// assert_eq!(grid[(3, 5)], 1);
    /// ```
    pub fn iter_mut(&mut self) -> IterMut<T> {
        self.data.iter_mut()
    }

    /// Returns an iterator over every position that can be used to index into the grid, in row-major order.
    ///
    /// # Examples
    ///
    /// ```
    /// use grid::Grid;
    ///
    /// let mut grid: Grid<u8> = Grid::new(3, 2, 0);
    ///
    /// let mut pos = grid.positions();
    ///
    /// assert_eq!(pos.next(), Some((0, 0)));
    /// assert_eq!(pos.next(), Some((1, 0)));
    /// assert_eq!(pos.next(), Some((2, 0)));
    /// assert_eq!(pos.next(), Some((0, 1)));
    /// assert_eq!(pos.next(), Some((1, 1)));
    /// assert_eq!(pos.next(), Some((2, 1)));
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
            x: 0,
            y: 0,
            width: self.width,
            height: self.height,
        }
    }

    /// Returns an iterator over every position and value in the grid, in row-major order.
    ///
    /// Values from this iterator come in the form of a tuple containing the position and a reference to the value:
    /// `((usize, usize), &T)`
    ///
    /// # Examples
    ///
    /// ```
    /// use grid::Grid;
    ///
    /// let grid: Grid<usize> = Grid::from_fn(8, 10, |(x, y)| x * 2 + y);
    ///
    /// for ((x, y), value) in grid.iter_positions() {
    ///     assert_eq!(grid[(x, y)], *value);
    ///     assert_eq!(value, &(x * 2 + y));
    /// }
    /// ```
    pub fn iter_positions(&self) -> PositionIter<T> {
        PositionIter::new(self.positions().zip(self.iter()))
    }

    /// Returns an iterator over every position and value in the grid, in row-major order.
    ///
    /// Values from this iterator come in the form of a tuple containing the position and a mutable reference to the value:
    /// `((usize, usize), &mut T)`
    ///
    /// # Examples
    ///
    /// ```
    /// use grid::Grid;
    ///
    /// let mut grid: Grid<usize> = Grid::new(8, 10, 3);
    ///
    /// for ((x, y), value) in grid.iter_mut_positions() {
    ///     *value = x * y;
    /// }
    ///
    /// assert_eq!(grid[(2, 3)], 6);
    /// assert_eq!(grid[(7, 9)], 63);
    /// ```
    pub fn iter_mut_positions(&mut self) -> PositionIterMut<T> {
        PositionIterMut::new(self.positions().zip(self.iter_mut()))
    }

    /// Returns an iterator over every position and value in the grid, in row-major order, consuming the grid.
    ///
    /// Values from this iterator come in the form of a tuple containing the position and the value:
    /// `((usize, usize), T)`
    ///
    /// # Examples
    ///
    /// ```
    /// use grid::Grid;
    ///
    /// let grid: Grid<u8> = Grid::new(8, 10, 6);
    ///
    /// for ((x, y), value) in grid.into_iter_positions() {
    ///     assert_eq!(value, 6);
    ///     println!("({}, {}): {}", x, y, value);
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
    /// use grid::Grid;
    ///
    /// let mut grid: Grid<u8> = Grid::new(5, 5, 7);
    /// grid[(1, 0)] = 5;
    ///
    /// let mut iter = grid.clone().into_iter();
    ///
    /// assert_eq!(iter.next(), Some(7));
    /// assert_eq!(iter.next(), Some(5));
    ///
    /// let mut sum = 0;
    /// for value in grid.into_iter() {
    ///     sum += value;
    /// }
    ///
    /// assert_eq!(sum, 173);
    /// ```
    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
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
    /// use grid::Grid;
    ///
    /// let mut grid: Grid<u8> = Grid::new(5, 5, 2);
    /// grid[(1, 0)] = 3;
    ///
    /// let mut sum = 0;
    /// for value in &grid {
    ///     sum += *value;
    /// }
    ///
    /// assert_eq!(sum, 51);
    /// ```
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
    /// use grid::Grid;
    ///
    /// let mut grid: Grid<u8> = Grid::new(8, 10, 0);
    /// grid[(1, 0)] = 2;
    ///
    /// for value in &mut grid {
    ///     *value += 1;
    /// }
    ///
    /// assert_eq!(grid[(1, 0)], 3);
    /// assert_eq!(grid[(3, 5)], 1);
    /// ```
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

/// An iterator over every position that can be used to index into an grid, in row-major order.
///
/// # Examples
///
/// ```
/// use grid::Grid;
///
/// let mut grid: Grid<u8> = Grid::new(3, 2, 0);
///
/// let mut pos = grid.positions();
///
/// assert_eq!(pos.next(), Some((0, 0)));
/// assert_eq!(pos.next(), Some((1, 0)));
/// assert_eq!(pos.next(), Some((2, 0)));
/// assert_eq!(pos.next(), Some((0, 1)));
/// assert_eq!(pos.next(), Some((1, 1)));
/// assert_eq!(pos.next(), Some((2, 1)));
/// assert_eq!(pos.next(), None);
///
/// let mut pos = grid.positions();
///
/// for value in grid.iter() {
///     assert_eq!(*value, grid[pos.next().unwrap()]);
/// }
/// ```
pub struct Positions {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

impl Iterator for Positions {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.y != self.height {
            let pos = (self.x, self.y);
            self.x += 1;
            if self.x == self.width {
                self.x = 0;
                self.y += 1;
            }
            return Some(pos);
        }
        None
    }
}

/// An iterator over every position and value in an grid, in row-major order.
///
/// Values from this iterator come in the form of a tuple containing the position and a reference to the value:
/// `((usize, usize), &T)`
///
/// # Examples
///
/// ```
/// use grid::Grid;
///
/// let grid: Grid<usize> = Grid::from_fn(8, 10, |(x, y)| x * 2 + y);
///
/// for ((x, y), value) in grid.iter_positions() {
///     assert_eq!(grid[(x, y)], *value);
///     assert_eq!(value, &(x * 2 + y));
/// }
/// ```
pub struct PositionIter<'a, T> {
    iter: Zip<Positions, Iter<'a, T>>,
}

impl<'a, T> PositionIter<'a, T> {
    fn new(iter: Zip<Positions, Iter<'a, T>>) -> PositionIter<'a, T> {
        PositionIter { iter }
    }
}

impl<'a, T> Iterator for PositionIter<'a, T> {
    type Item = ((usize, usize), &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

/// An iterator over every position and value in an grid, in row-major order.
///
/// Values from this iterator come in the form of a tuple containing the position and a mutable reference to the value:
/// `((usize, usize), &mut T)`
///
/// # Examples
///
/// ```
/// use grid::Grid;
///
/// let mut grid: Grid<usize> = Grid::new(8, 10, 3);
///
/// for ((x, y), value) in grid.iter_mut_positions() {
///     *value = x * y;
/// }
///
/// assert_eq!(grid[(2, 3)], 6);
/// assert_eq!(grid[(7, 9)], 63);
/// ```
pub struct PositionIterMut<'a, T> {
    iter: Zip<Positions, IterMut<'a, T>>,
}

impl<'a, T> PositionIterMut<'a, T> {
    fn new(iter: Zip<Positions, IterMut<'a, T>>) -> PositionIterMut<'a, T> {
        PositionIterMut { iter }
    }
}

impl<'a, T> Iterator for PositionIterMut<'a, T> {
    type Item = ((usize, usize), &'a mut T);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

/// An iterator over every position and value in an grid, in row-major order.
///
/// Values from this iterator come in the form of a tuple containing the position and the value:
/// `((usize, usize), T)`
///
/// # Examples
///
/// ```
/// use grid::Grid;
///
/// let grid: Grid<u8> = Grid::new(8, 10, 6);
///
/// for ((x, y), value) in grid.into_iter_positions() {
///     assert_eq!(value, 6);
///     println!("({}, {}): {}", x, y, value);
/// }
/// ```
pub struct PositionIntoIter<T> {
    iter: Zip<Positions, IntoIter<T>>,
}

impl<T> PositionIntoIter<T> {
    fn new(iter: Zip<Positions, IntoIter<T>>) -> PositionIntoIter<T> {
        PositionIntoIter { iter }
    }
}

impl<T> Iterator for PositionIntoIter<T> {
    type Item = ((usize, usize), T);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}
