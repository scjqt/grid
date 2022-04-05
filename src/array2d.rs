use std::ops::{Index, IndexMut};

/// A simple generic 2D array struct.
///
/// Indexable mutably and immutably by both `[isize; 2]` and `(isize, isize)`.
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
#[derive(PartialEq, Eq, Clone, Debug)]
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
    /// Requires that `T` is `Clone`.
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
        let mut data = Vec::with_capacity(width * height);
        data.resize(width * height, value);
        Array2D {
            data,
            width,
            height,
        }
    }
}

impl<T> Array2D<T>
where
    T: Default,
{
    /// Constructs a new `Array2D<T>` with the given dimensions, initialising all values to `T::default()`.
    /// Requires that `T` is `Default`.
    ///
    /// # Examples
    ///
    /// ```
    /// use array2d::Array2D;
    ///
    /// let arr: Array2D<u8> = Array2D::default_new(4, 2);
    ///
    /// assert_eq!(arr[[3, 1]], 0);
    /// ```
    pub fn default_new(width: usize, height: usize) -> Array2D<T> {
        let mut data = Vec::with_capacity(width * height);
        data.resize_with(width * height, || T::default());
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
    /// let arr: Array2D<u8> = Array2D::closure_new(3, 3, || 2);
    ///
    /// assert_eq!(arr[[0, 2]], 2);
    /// ```
    pub fn closure_new<F>(width: usize, height: usize, f: F) -> Array2D<T>
    where
        F: FnMut() -> T,
    {
        let mut data = Vec::with_capacity(width * height);
        data.resize_with(width * height, f);
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
    /// arr[[0, 0]] = 1;
    ///
    /// assert_eq!(arr.get(0, 0), Some(&1));
    /// assert_eq!(arr.get(3, 2), None);
    /// ```
    pub fn get(&self, x: isize, y: isize) -> Option<&T> {
        if x < 0 || x as usize >= self.width || y < 0 || y as usize >= self.height {
            return None;
        }
        Some(&self.data[x as usize + y as usize * self.width])
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
    /// assert_eq!(arr.get_mut(4, -1), None);
    /// ```
    pub fn get_mut(&mut self, x: isize, y: isize) -> Option<&mut T> {
        if x < 0 || x as usize >= self.width || y < 0 || y as usize >= self.height {
            return None;
        }
        Some(&mut self.data[x as usize + y as usize * self.width])
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
    /// assert_eq!(arr.set(-2, 1, 2), false);
    ///
    /// assert_eq!(arr[[2, 3]], 1);
    /// ```
    pub fn set(&mut self, x: isize, y: isize, value: T) -> bool {
        if x < 0 || x as usize >= self.width || y < 0 || y as usize >= self.height {
            return false;
        }
        self.data[x as usize + y as usize * self.width] = value;
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

impl<T> Index<[isize; 2]> for Array2D<T> {
    type Output = T;

    fn index(&self, index: [isize; 2]) -> &Self::Output {
        self.get(index[0], index[1]).expect("index out of bounds")
    }
}

impl<T> IndexMut<[isize; 2]> for Array2D<T> {
    fn index_mut(&mut self, index: [isize; 2]) -> &mut Self::Output {
        self.get_mut(index[0], index[1])
            .expect("index out of bounds")
    }
}

impl<T> Index<(isize, isize)> for Array2D<T> {
    type Output = T;

    fn index(&self, index: (isize, isize)) -> &Self::Output {
        self.get(index.0, index.1).expect("index out of bounds")
    }
}

impl<T> IndexMut<(isize, isize)> for Array2D<T> {
    fn index_mut(&mut self, index: (isize, isize)) -> &mut Self::Output {
        self.get_mut(index.0, index.1).expect("index out of bounds")
    }
}
