use std::{
    fmt::{self, Debug, Display},
    ops::{Index, IndexMut},
};

/// A simple generic 2D array struct.
///
/// Indexable mutably and immutably by both `[usize; 2]` and `(usize, usize)`.
///
/// # Examples
///
/// ```
/// use array2d::Array2D;
///
/// let mut arr: Array2D<u8> = Array2D::new(3, 4, 0);
///
/// arr[[1, 0]] = 1;
/// arr[(2, 3)] = 2;
///
/// assert_eq!(arr[[2, 3]], 2);
/// assert_eq!(arr[(1, 0)], 1);
/// ```
#[derive(PartialEq, Eq, Clone)]
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
    /// let arr: Array2D<u8> = Array2D::new(3, 5, 1);
    ///
    /// assert_eq!(arr[[2, 4]], 1);
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
    /// # Examples
    ///
    /// ```
    /// use array2d::Array2D;
    ///
    /// let arr: Array2D<u8> = Array2D::new_with(3, 3, || 2);
    ///
    /// assert_eq!(arr[[1, 2]], 2);
    /// ```
    pub fn new_with<F>(width: usize, height: usize, f: F) -> Array2D<T>
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

    /// Gets a reference to the value at position (`x`, `y`) of the array.
    /// If the position (`x`, `y`) is within the dimensions of the array, returns `Some(&T)`.
    /// Otherwise, returns `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// use array2d::Array2D;
    ///
    /// let mut arr: Array2D<u8> = Array2D::new(5, 2, 0);
    ///
    /// arr[[1, 1]] = 1;
    ///
    /// assert_eq!(arr.get(1, 1), Some(&1));
    /// assert_eq!(arr.get(3, 2), None);
    /// ```
    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        if x >= self.width || y >= self.height {
            return None;
        }
        Some(&self.data[x + y * self.width])
    }

    /// Gets a mutable reference to the value at position (`x`, `y`) of the array.
    /// If the position (`x`, `y`) is within the dimensions of the array, returns `Some(&mut T)`.
    /// Otherwise, returns `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// use array2d::Array2D;
    ///
    /// let mut arr: Array2D<u8> = Array2D::new(6, 7, 0);
    ///
    /// arr[[5, 3]] = 1;
    ///
    /// assert_eq!(arr.get_mut(5, 3), Some(&mut 1));
    /// assert_eq!(arr.get_mut(4, 7), None);
    /// ```
    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        if x >= self.width || y >= self.height {
            return None;
        }
        Some(&mut self.data[x + y * self.width])
    }

    /// Sets the value at position (`x`, `y`) of the array to `value`.
    /// Returns `true` on success, or `false` if the position (`x`, `y`) is outside the dimensions of the array.
    ///
    /// # Examples
    ///
    /// ```
    /// use array2d::Array2D;
    ///
    /// let mut arr: Array2D<u8> = Array2D::new(3, 4, 0);
    ///
    /// assert_eq!(arr.set(2, 3, 1), true);
    /// assert_eq!(arr.set(3, 1, 2), false);
    ///
    /// assert_eq!(arr[[2, 3]], 1);
    /// ```
    pub fn set(&mut self, x: usize, y: usize, value: T) -> bool {
        if x >= self.width || y >= self.height {
            return false;
        }
        self.data[x + y * self.width] = value;
        true
    }

    /// Returns the width of the array.
    ///
    /// # Examples
    ///
    /// ```
    /// use array2d::Array2D;
    ///
    /// let arr: Array2D<u8> = Array2D::new(3, 2, 0);
    ///
    /// assert_eq!(arr.width(), 3);
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
    /// let arr: Array2D<u8> = Array2D::new(2, 5, 0);
    ///
    /// assert_eq!(arr.height(), 5);
    /// ```
    pub fn height(&self) -> usize {
        self.height
    }
}

impl<T> Index<[usize; 2]> for Array2D<T> {
    type Output = T;

    fn index(&self, index: [usize; 2]) -> &Self::Output {
        self.get(index[0], index[1]).expect("index out of bounds")
    }
}

impl<T> IndexMut<[usize; 2]> for Array2D<T> {
    fn index_mut(&mut self, index: [usize; 2]) -> &mut Self::Output {
        self.get_mut(index[0], index[1])
            .expect("index out of bounds")
    }
}

impl<T> Index<(usize, usize)> for Array2D<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        self.get(index.0, index.1).expect("index out of bounds")
    }
}

impl<T> IndexMut<(usize, usize)> for Array2D<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        self.get_mut(index.0, index.1).expect("index out of bounds")
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

        for y in 0..self.height {
            write!(f, "[{}", self[[0, y]])?;
            for x in 1..self.width {
                let str = self[[x, y]].to_string();
                write!(f, ", {}{}", " ".repeat(longest - str.len()), str)?;
            }
            writeln!(f, "]")?;
        }

        writeln!(f)?;
        writeln!(f, "width: {}", self.width)?;
        writeln!(f, "height: {}", self.height)?;

        Ok(())
    }
}
