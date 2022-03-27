use std::ops::{Index, IndexMut};

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
    pub fn new(width: usize, height: usize, default: T) -> Array2D<T> {
        let mut data = Vec::with_capacity(width * height);
        data.resize(width * height, default);
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
    pub fn closure_new<F: FnMut() -> T>(width: usize, height: usize, func: F) -> Array2D<T> {
        let mut data = Vec::with_capacity(width * height);
        data.resize_with(width * height, func);
        Array2D {
            data,
            width,
            height,
        }
    }

    pub fn set(&mut self, x: usize, y: usize, value: T) -> bool {
        if x >= self.width || y >= self.height {
            return false;
        }
        self.data[x + y * self.width] = value;
        true
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        if x >= self.width || y >= self.height {
            return None;
        }
        Some(&self.data[x + y * self.width])
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        if x >= self.width || y >= self.height {
            return None;
        }
        Some(&mut self.data[x + y * self.width])
    }

    pub fn width(&self) -> usize {
        self.width
    }

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
