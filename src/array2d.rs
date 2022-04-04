use std::ops::{Index, IndexMut};

/// A simple generic 2D array type
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
    /// Requires that `T` is `Clone`.
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
    pub fn closure_new<F>(width: usize, height: usize, f: F) -> Array2D<T> 
    where F : FnMut() -> T {
        let mut data = Vec::with_capacity(width * height);
        data.resize_with(width * height, f);
        Array2D {
            data,
            width,
            height,
        }
    }

    /// Sets the value at position (`x`, `y`) of the array to `value`.
    /// Returns `true` on success, or `false` if the position (`x`, `y`) is outside the dimensions of the array.
    pub fn set(&mut self, x: usize, y: usize, value: T) -> bool {
        if x >= self.width || y >= self.height {
            return false;
        }
        self.data[x + y * self.width] = value;
        true
    }

    /// Gets a reference to the value at position (`x`, `y`) of the array.
    /// If the position (`x`, `y`) is within the dimensions of the array, returns `Some(&T)`.
    /// Otherwise, returns `None`.
    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        if x >= self.width || y >= self.height {
            return None;
        }
        Some(&self.data[x + y * self.width])
    }

    /// Gets a mutable reference to the value at position (`x`, `y`) of the array.
    /// If the position (`x`, `y`) is within the dimensions of the array, returns `Some(&mut T)`.
    /// Otherwise, returns `None`.
    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        if x >= self.width || y >= self.height {
            return None;
        }
        Some(&mut self.data[x + y * self.width])
    }

    /// Returns the width of the array.
    pub fn width(&self) -> usize {
        self.width
    }

    /// Returns the height of the array.
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
        self.get_mut(index[0], index[1]).expect("index out of bounds")
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
