//! A 2D vector struct with `i64` components.

pub mod constants;

use std::{
    fmt,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

/// A 2D vector struct with `i64` coordinates.
///
/// # Examples
///
/// ```
/// use grid::Vector;
///
/// let mut v = Vector::new(1, 2);
///
/// while v.x < 5 {
///     v += Vector::new(1, 3);
/// }
///
/// v *= 2;
///
/// assert_eq!(v, Vector::new(10, 28));
/// ```
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Debug, Default)]
pub struct Vector {
    pub x: i64,
    pub y: i64,
}

impl Vector {
    /// Creates a new `Vector` with the given `x` and `y` coordinates.
    #[inline(always)]
    pub const fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    /// Returns a vector containing the absolute values of each coordinate of `self`.
    #[inline]
    pub fn abs(self) -> Self {
        Self::new(self.x.abs(), self.y.abs())
    }

    /// Computes the manhattan distance from `self` to `rhs`.
    pub fn manhattan(self, rhs: Self) -> i64 {
        let diff = self - rhs;
        diff.x.abs() + diff.y.abs()
    }

    /// Returns a vector containing the signs of each coordinate of `self`.
    #[inline]
    pub fn signum(self) -> Self {
        Self::new(self.x.signum(), self.y.signum())
    }

    /// Returns a vector containing the minimum values of each coordinate of `self` and `rhs`.
    #[inline]
    pub fn min(self, rhs: Self) -> Self {
        Self::new(self.x.min(rhs.x), self.y.min(rhs.y))
    }

    /// Returns a vector containing the maximum values of each coordinate of `self` and `rhs`.
    #[inline]
    pub fn max(self, rhs: Self) -> Self {
        Self::new(self.x.max(rhs.x), self.y.max(rhs.y))
    }

    /// Returns a vector that is `self` clamped between `min` and `max`.
    ///
    /// Panics if `min.x > max.x` or `min.y > max.y`.
    #[inline]
    pub fn clamp(self, min: Self, max: Self) -> Self {
        Self::new(self.x.clamp(min.x, max.x), self.y.clamp(min.y, max.y))
    }

    /// Computes the dot product of `self` and `rhs`.
    #[inline]
    pub fn dot(self, rhs: Self) -> i64 {
        (self.x * rhs.x) + (self.y * rhs.y)
    }

    /// Returns a vector that is equal to `self` rotated by 90 degrees: (x, y) -> (-y, x)
    #[inline]
    pub fn perp(self) -> Self {
        Self::new(-self.y, self.x)
    }

    /// Computes the perpendicular dot product of `self` and `rhs`.
    #[inline]
    pub fn perp_dot(self, rhs: Self) -> i64 {
        (self.x * rhs.y) - (self.y * rhs.x)
    }

    /// Returns a vector equal to `self` with a `y` value of `0`.
    #[inline]
    pub fn horizontal(self) -> Self {
        Self::new(self.x, 0)
    }

    /// Returns a vector equal to `self` with an `x` value of `0`.
    #[inline]
    pub fn vertical(self) -> Self {
        Self::new(0, self.y)
    }
}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

/// A convenience macro for creating a `Vector` with `Vector::new()`.
///
/// # Examples
///
/// ```
/// use grid::{Vector, v};
///
/// let v = v!(5, 3);
///
/// assert_eq!(v, Vector::new(5, 3));
/// ```
#[macro_export]
macro_rules! v {
    ($x:expr, $y:expr) => {
        grid::Vector::new($x, $y)
    };
}

pub use v;

impl Add for Vector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Vector {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl SubAssign for Vector {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Mul<i64> for Vector {
    type Output = Self;

    fn mul(self, rhs: i64) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl Mul<Vector> for i64 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        Vector::new(self * rhs.x, self * rhs.y)
    }
}

impl MulAssign<i64> for Vector {
    fn mul_assign(&mut self, rhs: i64) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl Div<i64> for Vector {
    type Output = Self;

    fn div(self, rhs: i64) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs)
    }
}

impl DivAssign<i64> for Vector {
    fn div_assign(&mut self, rhs: i64) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y)
    }
}
