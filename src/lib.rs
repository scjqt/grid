//! A simple generic heap-allocated 2D array library.
//!
//! For a position `(x, y)` in the array:
//! * `x`, the first value, determines which column the position is in
//! * `y`, the second value, determines which row the position is in
//!
//! There are `width` columns and `height` rows in the array, and the array's iterators traverse it in row-major order.
//!
//! Implements the [`Debug`] trait if `T` implements the [`Display`] trait.
//!
//! Indexable mutably and immutably by `(T, T)` if `T: TryInto<usize>`.
//!
//! # Examples
//!
//! ```
//! use array2d::Array2D;
//!
//! let mut arr: Array2D<u8> = Array2D::new(8, 10, 0);
//!
//! arr[(1, 0)] = 1;
//! arr[(3, 5)] = 2;
//!
//! assert_eq!(arr[(3, 5)], 2);
//! assert_eq!(arr[(1, 0)], 1);
//! assert_eq!(arr[(6, 4)], 0);
//!
//! println!("{:?}", arr);
//! ```

pub mod iterators;

use std::{
    fmt,
    ops::{Index, IndexMut},
};

/// A simple generic heap-allocated 2D array struct.
///
/// For a position `(x, y)` in the array:
/// * `x`, the first value, determines which column the position is in
/// * `y`, the second value, determines which row the position is in
///
/// There are `width` columns and `height` rows in the array, and the array's iterators traverse it in row-major order.
///
/// Implements the [`Debug`] trait if `T` implements the [`Display`] trait.
///
/// Indexable mutably and immutably by `(T, T)` if `T: TryInto<usize>`.
///
/// # Examples
///
/// ```
/// use array2d::Array2D;
///
/// let mut arr: Array2D<u8> = Array2D::new(8, 10, 0);
///
/// arr[(1, 0)] = 1;
/// arr[(3, 5)] = 2;
///
/// assert_eq!(arr[(3, 5)], 2);
/// assert_eq!(arr[(1, 0)], 1);
/// assert_eq!(arr[(6, 4)], 0);
///
/// println!("{:?}", arr);
/// ```
#[derive(PartialEq, Eq, Clone, Hash)]
pub struct Array2D<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl<T: Clone> Array2D<T> {
    /// Constructs a new `Array2D<T>` with the given dimensions, initialising all values to `value`.
    ///
    /// Requires that `T` implements the [`Clone`] trait.
    ///
    /// # Examples
    ///
    /// ```
    /// use array2d::Array2D;
    ///
    /// let arr: Array2D<u8> = Array2D::new(8, 10, 1);
    ///
    /// assert_eq!(arr[(2, 4)], 1);
    /// assert_eq!(arr[(7, 3)], 1);
    /// ```
    pub fn new(width: usize, height: usize, value: T) -> Array2D<T> {
        let size = size(width, height);
        let mut data = Vec::with_capacity(size);
        data.resize(size, value);
        Array2D {
            data,
            width,
            height,
        }
    }
}

impl<T: Default> Array2D<T> {
    /// Constructs a new `Array2D<T>` with the given dimensions, initialising all values to their default value.
    ///
    /// Requires that `T` implements the [`Default`] trait.
    ///
    /// # Examples
    ///
    /// ```
    /// use array2d::Array2D;
    ///
    /// let arr: Array2D<u8> = Array2D::default(9, 3);
    ///
    /// assert_eq!(arr[(5, 1)], 0);
    /// assert_eq!(arr[(6, 2)], 0);
    /// ```
    pub fn default(width: usize, height: usize) -> Array2D<T> {
        let size = size(width, height);
        let mut data = Vec::with_capacity(size);
        data.resize_with(size, Default::default);
        Array2D {
            data,
            width,
            height,
        }
    }
}

impl<T> Array2D<T> {
    /// Constructs a new `Array2D<T>` with the given dimensions, computing all initial values from the closure `f`.
    ///
    /// If `T` implements the [`Default`] trait, `Default::default` can be passed in.
    ///
    /// # Examples
    ///
    /// ```
    /// use array2d::Array2D;
    ///
    /// let arr: Array2D<u8> = Array2D::from_simple_fn(8, 10, || 2);
    ///
    /// assert_eq!(arr[(5, 3)], 2);
    /// assert_eq!(arr[(1, 8)], 2);
    ///
    /// let arr: Array2D<u8> = Array2D::from_simple_fn(8, 10, Default::default);
    ///
    /// assert_eq!(arr[(0, 3)], 0);
    /// assert_eq!(arr[(6, 4)], 0);
    /// ```
    pub fn from_simple_fn<F>(width: usize, height: usize, f: F) -> Array2D<T>
    where
        F: FnMut() -> T,
    {
        let size = size(width, height);
        let mut data = Vec::with_capacity(size);
        data.resize_with(size, f);
        Array2D {
            data,
            width,
            height,
        }
    }
    /// Constructs a new `Array2D<T>` with the given dimensions, computing all initial values from the closure `f` which maps each position to a value.
    ///
    /// # Examples
    ///
    /// ```
    /// use array2d::Array2D;
    ///
    /// let arr: Array2D<usize> = Array2D::from_fn(8, 10, |(x, y)| x + y);
    ///
    /// assert_eq!(arr[(5, 3)], 8);
    /// assert_eq!(arr[(7, 9)], 16);
    /// ```
    pub fn from_fn<F>(width: usize, height: usize, mut f: F) -> Array2D<T>
    where
        F: FnMut((usize, usize)) -> T,
    {
        let mut data = Vec::with_capacity(width.checked_mul(height).expect("dimensions too large"));
        for y in 0..height {
            for x in 0..width {
                data.push(f((x, y)));
            }
        }
        Array2D {
            data,
            width,
            height,
        }
    }

    /// Returns the width of the array.
    ///
    /// # Examples
    ///
    /// ```
    /// use array2d::Array2D;
    ///
    /// let arr: Array2D<u8> = Array2D::new(8, 10, 9);
    ///
    /// assert_eq!(arr.width(), 8);
    /// ```
    pub fn width(&self) -> usize {
        self.width
    }

    /// Returns the height of the array.
    ///
    /// # Examples
    ///
    /// ```
    /// use array2d::Array2D;
    ///
    /// let arr: Array2D<u8> = Array2D::new(8, 10, 10);
    ///
    /// assert_eq!(arr.height(), 10);
    /// ```
    pub fn height(&self) -> usize {
        self.height
    }

    /// Returns a reference to the value at the given position of the array, or `None` if out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use array2d::Array2D;
    ///
    /// let mut arr: Array2D<u8> = Array2D::new(8, 10, 3);
    ///
    /// arr[(1, 1)] = 4;
    ///
    /// assert_eq!(arr.get((5, 2)), Some(&3));
    /// assert_eq!(arr.get((1, 1)), Some(&4));
    /// assert_eq!(arr.get((8, 6)), None);
    /// assert_eq!(arr.get((4, 10)), None);
    /// ```
    pub fn get<C: TryInto<usize>>(&self, pos: (C, C)) -> Option<&T> {
        let (x, y) = self.get_pos(pos)?;
        Some(&self.data[x + y * self.width])
    }

    /// Returns a mutable reference to the value at the given position of the array, or `None` if out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use array2d::Array2D;
    ///
    /// let mut arr: Array2D<u8> = Array2D::new(8, 10, 4);
    ///
    /// arr[(5, 3)] = 2;
    ///
    /// assert_eq!(arr.get_mut((5, 3)), Some(&mut 2));
    /// assert_eq!(arr.get_mut((0, 0)), Some(&mut 4));
    /// assert_eq!(arr.get_mut((1, 10)), None);
    /// assert_eq!(arr.get_mut((9, 7)), None);
    /// ```
    pub fn get_mut<C: TryInto<usize>>(&mut self, pos: (C, C)) -> Option<&mut T> {
        let (x, y) = self.get_pos(pos)?;
        Some(&mut self.data[x + y * self.width])
    }

    /// Sets the value at the given position of the array.
    ///
    /// Returns the old value at that position, or `None` if out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use array2d::Array2D;
    ///
    /// let mut arr: Array2D<u8> = Array2D::new(8, 10, 5);
    ///
    /// assert_eq!(arr.set((2, 3), 7), Some(5));
    /// assert_eq!(arr.set((9, 12), 1), None);
    ///
    /// assert_eq!(arr[(2, 3)], 7);
    /// ```
    pub fn set<C: TryInto<usize>>(&mut self, pos: (C, C), value: T) -> Option<T> {
        let (x, y) = self.get_pos(pos)?;
        Some(std::mem::replace(&mut self.data[x + y * self.width], value))
    }

    /// Returns `true` if the given position is within the bounds of the array, or `false` if not.
    ///
    /// # Examples
    ///
    /// ```
    /// use array2d::Array2D;
    ///
    /// let arr: Array2D<u8> = Array2D::new(15, 14, 11);
    ///
    /// assert_eq!(arr.in_bounds((0, 0)), true);
    /// assert_eq!(arr.in_bounds((10, 4)), true);
    /// assert_eq!(arr.in_bounds((15, 2)), false);
    /// assert_eq!(arr.in_bounds((3, 17)), false);
    /// assert_eq!(arr.in_bounds((-1, 5)), false);
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

impl<T, C: TryInto<usize>> Index<(C, C)> for Array2D<T> {
    type Output = T;

    fn index(&self, index: (C, C)) -> &Self::Output {
        self.get(index).expect("position out of bounds")
    }
}

impl<T, C: TryInto<usize>> IndexMut<(C, C)> for Array2D<T> {
    fn index_mut(&mut self, index: (C, C)) -> &mut Self::Output {
        self.get_mut(index).expect("position out of bounds")
    }
}

impl<T: fmt::Display> fmt::Debug for Array2D<T> {
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
