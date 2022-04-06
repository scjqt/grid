use std::{
    fmt::{self, Debug, Display},
    ops::{Index, IndexMut},
    slice::{Iter, IterMut},
    vec::IntoIter,
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

    /// Gets a reference to the value at the given position of the array, after offsetting it.
    /// If the new position is within the dimensions of the array, returns `Some(&T)`.
    /// Otherwise, returns `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// use array2d::Array2D;
    ///
    /// let mut arr: Array2D<u8> = Array2D::new(8, 10, 6);
    ///
    /// arr[[7, 8]] = 1;
    ///
    /// assert_eq!(arr.get_offset((1, 3), (6, 5)), Some(&1));
    /// assert_eq!(arr.get_offset((10, 12), (-5, -3)), Some(&6));
    /// assert_eq!(arr.get_offset((0, 0), (-1, 0)), None);
    /// ```
    pub fn get_offset(&self, pos: (usize, usize), offset: (isize, isize)) -> Option<&T> {
        self.get(offset_checked(pos, offset)?)
    }

    /// Gets a mutable reference to the value at the given position of the array, after offsetting it.
    /// If the new position is within the dimensions of the array, returns `Some(&mut T)`.
    /// Otherwise, returns `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// use array2d::Array2D;
    ///
    /// let mut arr: Array2D<u8> = Array2D::new(8, 10, 7);
    ///
    /// arr[[6, 3]] = 4;
    ///
    /// assert_eq!(arr.get_offset((4, 11), (2, -8)), Some(&4));
    /// assert_eq!(arr.get_offset((3, 2), (1, -1)), Some(&7));
    /// assert_eq!(arr.get_offset((1, 5), (7, 0)), None);
    /// ```
    pub fn get_mut_offset(
        &mut self,
        pos: (usize, usize),
        offset: (isize, isize),
    ) -> Option<&mut T> {
        self.get_mut(offset_checked(pos, offset)?)
    }

    /// Sets the value at the given position of the array, after offsetting it, to `value`.
    /// Returns `true` on success, or `false` if the new position is outside the dimensions of the array.
    ///
    /// # Examples
    ///
    /// ```
    /// use array2d::Array2D;
    ///
    /// let mut arr: Array2D<u8> = Array2D::new(8, 10, 8);
    ///
    /// assert_eq!(arr.set_offset((5, 9), (1, -4), 0), true);
    /// assert_eq!(arr.set_offset((0, 10), (5, 0), 6), false);
    ///
    /// assert_eq!(arr[[6, 5]], 0);
    /// ```
    pub fn set_offset(&mut self, pos: (usize, usize), offset: (isize, isize), value: T) -> bool {
        if let Some(pos) = offset_checked(pos, offset) {
            return self.set(pos, value);
        }
        false
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
    pub fn positions(&self) -> PositionIter {
        PositionIter {
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
    pub fn iter_positions(&self) -> impl IntoIterator<Item = ((usize, usize), &T)> {
        self.positions().zip(self.iter())
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
    pub fn iter_mut_positions(&mut self) -> impl IntoIterator<Item = ((usize, usize), &mut T)> {
        self.positions().zip(self.iter_mut())
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
    pub fn into_iter_positions(self) -> impl IntoIterator<Item = ((usize, usize), T)> {
        self.positions().zip(self.into_iter())
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

        for y in 0..self.height {
            write!(f, "[{}", self[[0, y]])?;
            for x in 1..self.width {
                let str = self[[x, y]].to_string();
                write!(f, ", {}{}", " ".repeat(longest - str.len()), str)?;
            }
            writeln!(f, "]")?;
        }

        writeln!(f)?;
        writeln!(f, "width:  {}", self.width)?;
        write!(f, "height: {}", self.height)?;

        Ok(())
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
pub struct PositionIter {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

impl Iterator for PositionIter {
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

fn offset_checked(pos: (usize, usize), offset: (isize, isize)) -> Option<(usize, usize)> {
    Some((
        offset_value_checked(pos.0, offset.0)?,
        offset_value_checked(pos.1, offset.1)?,
    ))
}

fn offset_value_checked(value: usize, offset: isize) -> Option<usize> {
    if offset < 0 {
        let abs = offset.abs() as usize;
        if abs > value {
            return None;
        }
        return Some(value - abs);
    }
    value.checked_add(offset as usize)
}
