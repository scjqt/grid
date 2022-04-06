use super::Array2D;
use std::{
    iter::Zip,
    slice::{Iter, IterMut},
    vec::IntoIter,
};

impl<T> Array2D<T> {
    /// Returns an iterator over references to the values in the array, in row-major order.
    ///
    /// # Examples
    ///
    /// ```
    /// use array2d::Array2D;
    ///
    /// let mut arr: Array2D<u8> = Array2D::new(5, 5, 2);
    /// arr[[1, 0]] = 3;
    ///
    /// let mut iter = arr.iter();
    ///
    /// assert_eq!(iter.next(), Some(&2));
    /// assert_eq!(iter.next(), Some(&3));
    ///
    /// let mut sum = 0;
    /// for value in arr.iter() {
    ///     sum += *value;
    /// }
    ///
    /// assert_eq!(sum, 51);
    /// assert_eq!(arr.iter().sum::<u8>(), 51);
    /// ```
    pub fn iter(&self) -> Iter<T> {
        self.data.iter()
    }

    /// Returns an iterator over mutable references to the values in the array, in row-major order.
    ///
    /// # Examples
    ///
    /// ```
    /// use array2d::Array2D;
    ///
    /// let mut arr: Array2D<u8> = Array2D::new(8, 10, 0);
    /// arr[[1, 0]] = 2;
    ///
    /// let mut iter = arr.iter_mut();
    ///
    /// assert_eq!(iter.next(), Some(&mut 0));
    /// assert_eq!(iter.next(), Some(&mut 2));
    ///
    /// for value in arr.iter_mut() {
    ///     *value += 1;
    /// }
    ///
    /// assert_eq!(arr[[1, 0]], 3);
    /// assert_eq!(arr[[3, 5]], 1);
    /// ```
    pub fn iter_mut(&mut self) -> IterMut<T> {
        self.data.iter_mut()
    }

    /// Returns an iterator over the values in the array, in row-major order, consuming the array.
    ///
    /// # Examples
    ///
    /// ```
    /// use array2d::Array2D;
    ///
    /// let mut arr: Array2D<u8> = Array2D::new(5, 5, 7);
    /// arr[[1, 0]] = 5;
    ///
    /// let mut iter = arr.clone().into_iter();
    ///
    /// assert_eq!(iter.next(), Some(7));
    /// assert_eq!(iter.next(), Some(5));
    ///
    /// let mut sum = 0;
    /// for value in arr.into_iter() {
    ///     sum += value;
    /// }
    ///
    /// assert_eq!(sum, 173);
    /// ```
    pub fn into_iter(self) -> IntoIter<T> {
        self.data.into_iter()
    }

    /// Returns an iterator over every position that can be used to index into the array, in row-major order.
    ///
    /// # Examples
    ///
    /// ```
    /// use array2d::Array2D;
    ///
    /// let mut arr: Array2D<u8> = Array2D::new(3, 2, 0);
    ///
    /// let mut pos = arr.positions();
    ///
    /// assert_eq!(pos.next(), Some((0, 0)));
    /// assert_eq!(pos.next(), Some((1, 0)));
    /// assert_eq!(pos.next(), Some((2, 0)));
    /// assert_eq!(pos.next(), Some((0, 1)));
    /// assert_eq!(pos.next(), Some((1, 1)));
    /// assert_eq!(pos.next(), Some((2, 1)));
    /// assert_eq!(pos.next(), None);
    ///
    /// let mut pos = arr.positions();
    ///
    /// for value in arr.iter() {
    ///     assert_eq!(*value, arr[pos.next().unwrap()]);
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

    /// Returns an iterator over every position and value in the array, in row-major order.
    ///
    /// Values from this iterator come in the form of a tuple containing the position and a reference to the value:
    /// `((usize, usize), &T)`
    ///
    /// # Examples
    ///
    /// ```
    /// use array2d::Array2D;
    ///
    /// let arr: Array2D<usize> = Array2D::from_fn(8, 10, |(x, y)| x * 2 + y);
    ///
    /// for ((x, y), value) in arr.iter_positions() {
    ///     assert_eq!(arr[[x, y]], *value);
    ///     assert_eq!(value, &(x * 2 + y));
    /// }
    /// ```
    pub fn iter_positions(&self) -> PositionIter<T> {
        PositionIter::new(self.positions().zip(self.iter()))
    }

    /// Returns an iterator over every position and value in the array, in row-major order.
    ///
    /// Values from this iterator come in the form of a tuple containing the position and a mutable reference to the value:
    /// `((usize, usize), &mut T)`
    ///
    /// ```
    /// use array2d::Array2D;
    ///
    /// let mut arr: Array2D<usize> = Array2D::new(8, 10, 3);
    ///
    /// for ((x, y), value) in arr.iter_mut_positions() {
    ///     *value = x * y;
    /// }
    ///
    /// assert_eq!(arr[[2, 3]], 6);
    /// assert_eq!(arr[[7, 9]], 63);
    /// ```
    pub fn iter_mut_positions(&mut self) -> PositionIterMut<T> {
        PositionIterMut::new(self.positions().zip(self.iter_mut()))
    }

    /// Returns an iterator over every position and value in the array, in row-major order, consuming the array.
    ///
    /// Values from this iterator come in the form of a tuple containing the position and the value:
    /// `((usize, usize), T)`
    ///
    /// ```
    /// use array2d::Array2D;
    ///
    /// let arr: Array2D<u8> = Array2D::new(8, 10, 6);
    ///
    /// for ((x, y), value) in arr.into_iter_positions() {
    ///     assert_eq!(value, 6);
    ///     println!("({}, {}): {}", x, y, value);
    /// }
    /// ```
    pub fn into_iter_positions(self) -> PositionIntoIter<T> {
        PositionIntoIter::new(self.positions().zip(self.into_iter()))
    }
}

/// An iterator over every position that can be used to index into an array, in row-major order.
///
/// # Examples
///
/// ```
/// use array2d::Array2D;
///
/// let mut arr: Array2D<u8> = Array2D::new(3, 2, 0);
///
/// let mut pos = arr.positions();
///
/// assert_eq!(pos.next(), Some((0, 0)));
/// assert_eq!(pos.next(), Some((1, 0)));
/// assert_eq!(pos.next(), Some((2, 0)));
/// assert_eq!(pos.next(), Some((0, 1)));
/// assert_eq!(pos.next(), Some((1, 1)));
/// assert_eq!(pos.next(), Some((2, 1)));
/// assert_eq!(pos.next(), None);
///
/// let mut pos = arr.positions();
///
/// for value in arr.iter() {
///     assert_eq!(*value, arr[pos.next().unwrap()]);
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

/// An iterator over every position and value in an array, in row-major order.
///
/// Values from this iterator come in the form of a tuple containing the position and a reference to the value:
/// `((usize, usize), &T)`
///
/// # Examples
///
/// ```
/// use array2d::Array2D;
///
/// let arr: Array2D<usize> = Array2D::from_fn(8, 10, |(x, y)| x * 2 + y);
///
/// for ((x, y), value) in arr.iter_positions() {
///     assert_eq!(arr[[x, y]], *value);
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

/// An iterator over every position and value in an array, in row-major order.
///
/// Values from this iterator come in the form of a tuple containing the position and a mutable reference to the value:
/// `((usize, usize), &mut T)`
///
/// ```
/// use array2d::Array2D;
///
/// let mut arr: Array2D<usize> = Array2D::new(8, 10, 3);
///
/// for ((x, y), value) in arr.iter_mut_positions() {
///     *value = x * y;
/// }
///
/// assert_eq!(arr[[2, 3]], 6);
/// assert_eq!(arr[[7, 9]], 63);
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

/// An iterator over every position and value in an array, in row-major order.
///
/// Values from this iterator come in the form of a tuple containing the position and the value:
/// `((usize, usize), T)`
///
/// ```
/// use array2d::Array2D;
///
/// let arr: Array2D<u8> = Array2D::new(8, 10, 6);
///
/// for ((x, y), value) in arr.into_iter_positions() {
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
