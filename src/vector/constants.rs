//! Useful `Vector` constants for traversing 2D space.

use crate::vector::Vector;

pub const ZERO: Vector = Vector::new(0, 0);

pub const EAST: Vector = Vector::new(1, 0);
pub const NORTH: Vector = Vector::new(0, -1);
pub const WEST: Vector = Vector::new(-1, 0);
pub const SOUTH: Vector = Vector::new(0, 1);

pub const NE: Vector = Vector::new(1, -1);
pub const NW: Vector = Vector::new(-1, -1);
pub const SW: Vector = Vector::new(-1, 1);
pub const SE: Vector = Vector::new(1, 1);

pub const ORTHOGONAL: [Vector; 4] = [EAST, NORTH, WEST, SOUTH];
pub const DIAGONAL: [Vector; 4] = [NE, NW, SW, SE];
pub const ADJACENT: [Vector; 8] = [EAST, NE, NORTH, NW, WEST, SW, SOUTH, SE];

pub const ORTHOGONAL_ZERO: [Vector; 5] = [ZERO, EAST, NORTH, WEST, SOUTH];
pub const DIAGONAL_ZERO: [Vector; 5] = [ZERO, NE, NW, SW, SE];
pub const ADJACENT_ZERO: [Vector; 9] = [ZERO, EAST, NE, NORTH, NW, WEST, SW, SOUTH, SE];
