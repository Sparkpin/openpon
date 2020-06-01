use num::{Num, Signed, Zero};
use std::ops::{Add, AddAssign, Neg, Sub};

/// For internal use as a trait bound.
/// Must be public due to Rust's rules on private trait bounds.
/// To implement this, your type should implement the constituent traits.
/// This should cover all useful primitives.
// BLOCKED: [rust-lang/rust#41517](https://github.com/rust-lang/rust/issues/41517)
// Trait aliases are unstable
pub trait Vec2TBound: Num + AddAssign + Zero {}
impl<T: Num + AddAssign + Zero> Vec2TBound for T {}

#[derive(Clone, Eq, PartialEq)]
pub struct Vec2<T: Vec2TBound> {
    pub x: T,
    pub y: T
}

impl<T: Vec2TBound> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self {x, y}
    }

    /// Add dx and dy to this Vec2's x and y
    pub fn translate(&mut self, dx: T, dy: T) {
        self.x += dx;
        self.y += dy;
    }

    pub fn set(&mut self, x: T, y: T) {
        self.x = x;
        self.y = y;
    }
}

impl<T: Vec2TBound> Default for Vec2<T> {
    fn default() -> Self {
        Self {x: T::zero(), y: T::zero()}
    }
}

impl<T: Vec2TBound> From<(T, T)> for Vec2<T> {
    fn from(tuple: (T, T)) -> Self {
        Self {x: tuple.0, y: tuple.1}
    }
}

impl<T: Vec2TBound> Add for Vec2<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {x: self.x + other.x, y: self.y + other.y}
    }
}

impl<T: Vec2TBound + Signed> Neg for Vec2<T> {
    type Output = Self;

    fn neg(self) -> Self {
        Self {x: -self.x, y: -self.y}
    }
}

impl<T: Vec2TBound> Sub for Vec2<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {x: self.x - other.x, y: self.y - other.y}
    }
}
