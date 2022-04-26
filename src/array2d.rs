pub mod iterators;

use std::{
    fmt::{self, Debug, Display},
    ops::{Index, IndexMut},
};

/// A simple generic 2D array struct.
///
/// For a position `(x, y)` in the array:
/// * `x`, the first value, determines which column the position is in
/// * `y`, the second value, determines which row the position is in
///
/// There are `width` columns and `height` rows in the array, and the array's iterators traverse it in row-major order.
///
/// Implements the [`Debug`] trait if `T` implements the [`Display`] trait.
///
/// Indexable mutably and immutably by `(usize, usize)` and `[usize; 2]`.
///
/// # Examples
///
/// ```
/// use array2d::Array2D;
///
/// let mut arr: Array2D<u8> = Array2D::new(8, 10, 0);
///
/// arr[[1, 0]] = 1;
/// arr[(3, 5)] = 2;
///
/// assert_eq!(arr[[3, 5]], 2);
/// assert_eq!(arr[(1, 0)], 1);
/// assert_eq!(arr[[6, 4]], 0);
///
/// println!("{:?}", arr);
/// ```
#[derive(PartialEq, Eq, Clone, Hash)]
pub struct Array2D<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Array2D<T>
where
    T: Clone,
{
    /// Constructs a new `Array2D<T>` with the given dimensions, initialising all values to `value`.
    /// Requires that `T` implements the [`Clone`] trait.
    ///
    /// # Examples
    ///
    /// ```
    /// use array2d::Array2D;
    ///
    /// let arr: Array2D<u8> = Array2D::new(8, 10, 1);
    ///
    /// assert_eq!(arr[[2, 4]], 1);
    /// assert_eq!(arr[[7, 3]], 1);
    /// ```
    pub fn new(width: usize, height: usize, value: T) -> Array2D<T> {
        let size = width.checked_mul(height).expect("dimensions too large");
        let mut data = Vec::with_capacity(size);
        data.resize(size, value);
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
    /// If `T` implements the [`Default`] trait, [`Default::default`] can be passed in.
    ///
    /// # Examples
    ///
    /// ```
    /// use array2d::Array2D;
    ///
    /// let arr: Array2D<u8> = Array2D::from_simple_fn(8, 10, || 2);
    ///
    /// assert_eq!(arr[[5, 3]], 2);
    /// assert_eq!(arr[[1, 8]], 2);
    ///
    /// let arr: Array2D<u8> = Array2D::from_simple_fn(8, 10, Default::default);
    ///
    /// assert_eq!(arr[[0, 3]], 0);
    /// assert_eq!(arr[[6, 4]], 0);
    /// ```
    pub fn from_simple_fn<F>(width: usize, height: usize, f: F) -> Array2D<T>
    where
        F: FnMut() -> T,
    {
        let size = width.checked_mul(height).expect("dimensions too large");
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
    /// assert_eq!(arr[[5, 3]], 8);
    /// assert_eq!(arr[[7, 9]], 16);
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

    /// Gets a reference to the value at the given position of the array.
    /// If the position is within the dimensions of the array, returns `Some(&T)`.
    /// Otherwise, returns `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// use array2d::Array2D;
    ///
    /// let mut arr: Array2D<u8> = Array2D::new(8, 10, 3);
    ///
    /// arr[[1, 1]] = 4;
    ///
    /// assert_eq!(arr.get((5, 2)), Some(&3));
    /// assert_eq!(arr.get((1, 1)), Some(&4));
    /// assert_eq!(arr.get((8, 6)), None);
    /// assert_eq!(arr.get((4, 10)), None);
    /// ```
    pub fn get(&self, pos: (usize, usize)) -> Option<&T> {
        if pos.0 >= self.width || pos.1 >= self.height {
            return None;
        }
        Some(&self.data[pos.0 + pos.1 * self.width])
    }

    /// Gets a mutable reference to the value at the given position of the array.
    /// If the position is within the dimensions of the array, returns `Some(&mut T)`.
    /// Otherwise, returns `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// use array2d::Array2D;
    ///
    /// let mut arr: Array2D<u8> = Array2D::new(8, 10, 4);
    ///
    /// arr[[5, 3]] = 2;
    ///
    /// assert_eq!(arr.get_mut((5, 3)), Some(&mut 2));
    /// assert_eq!(arr.get_mut((0, 0)), Some(&mut 4));
    /// assert_eq!(arr.get_mut((1, 10)), None);
    /// assert_eq!(arr.get_mut((9, 7)), None);
    /// ```
    pub fn get_mut(&mut self, pos: (usize, usize)) -> Option<&mut T> {
        if pos.0 >= self.width || pos.1 >= self.height {
            return None;
        }
        Some(&mut self.data[pos.0 + pos.1 * self.width])
    }

    /// Sets the value at the given position of the array to `value`.
    /// Returns `true` on success, or `false` if the position is outside the dimensions of the array.
    ///
    /// # Examples
    ///
    /// ```
    /// use array2d::Array2D;
    ///
    /// let mut arr: Array2D<u8> = Array2D::new(8, 10, 5);
    ///
    /// assert_eq!(arr.set((2, 3), 7), true);
    /// assert_eq!(arr.set((9, 12), 1), false);
    ///
    /// assert_eq!(arr[[2, 3]], 7);
    /// ```
    pub fn set(&mut self, pos: (usize, usize), value: T) -> bool {
        if pos.0 >= self.width || pos.1 >= self.height {
            return false;
        }
        self.data[pos.0 + pos.1 * self.width] = value;
        true
    }

    /// Offsets the given position, returning the result.
    ///
    /// If the resulting position is inside the dimensions of the array, returns `Some((usize, usize))`.
    /// Otherwise, returns `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// use array2d::Array2D;
    ///
    /// let mut arr: Array2D<u8> = Array2D::new(8, 10, 7);
    ///
    /// assert_eq!(arr.offset((4, 3), (-3, 6)), Some((1, 9)));
    /// assert_eq!(arr.offset((6, 2), (-5, -3)), None);
    /// assert_eq!(arr.offset((5, 9), (1, 2)), None);
    ///
    /// arr[[3, 6]] = 2;
    ///
    /// if let Some(adj) = arr.offset((1, 9), (2, -3)) {
    ///     assert_eq!(arr[adj], 2);
    /// }
    /// ```
    pub fn offset(&self, pos: (usize, usize), offset: (isize, isize)) -> Option<(usize, usize)> {
        Some((
            offset_value(pos.0, offset.0, self.width)?,
            offset_value(pos.1, offset.1, self.height)?,
        ))
    }
}

impl<T> Index<[usize; 2]> for Array2D<T> {
    type Output = T;

    fn index(&self, index: [usize; 2]) -> &Self::Output {
        self.get((index[0], index[1])).expect("index out of bounds")
    }
}

impl<T> IndexMut<[usize; 2]> for Array2D<T> {
    fn index_mut(&mut self, index: [usize; 2]) -> &mut Self::Output {
        self.get_mut((index[0], index[1]))
            .expect("index out of bounds")
    }
}

impl<T> Index<(usize, usize)> for Array2D<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        self.get(index).expect("index out of bounds")
    }
}

impl<T> IndexMut<(usize, usize)> for Array2D<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        self.get_mut(index).expect("index out of bounds")
    }
}

impl<T> Debug for Array2D<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let mut longest = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                longest = longest.max(self[[x, y]].to_string().len());
            }
        }

        writeln!(f, "{}x{}", self.width, self.height)?;

        for y in 0..self.height {
            for x in 0..self.width {
                let str = self[[x, y]].to_string();
                write!(f, "{}{}", " ".repeat(longest - str.len()), str)?;
                if x != self.width - 1 {
                    write!(f, ", ")?;
                }
            }
            if y != self.height - 1 {
                writeln!(f)?;
            }
        }

        Ok(())
    }
}

fn offset_value(value: usize, offset: isize, limit: usize) -> Option<usize> {
    let new = if offset < 0 {
        let abs = offset.abs() as usize;
        if abs > value {
            return None;
        }
        value - abs
    } else {
        value.checked_add(offset as usize)?
    };
    if new >= limit {
        return None;
    }
    Some(new)
}
