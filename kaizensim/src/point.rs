use std::ops::*;

#[derive(Clone, Copy, Default, PartialEq, PartialOrd, Eq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub const ZERO: Self = Point { x: 0, y: 0 };
    pub fn signum(&self) -> Self {
        Point { x: self.x.signum(), y: self.y.signum() }
    }
    pub fn sum(&self) -> i32 {
        self.x + self.y
    }
    pub fn is_empty(&self) -> bool {
        self.x == 0 && self.y == 0
    }
    pub fn is_line(&self) -> bool {
        self.x == 0 || self.y == 0
    }
    pub fn rotated(&self) -> Self {
        Point { x: -self.y, y: self.x }
    }
}

impl Mul<i32> for Point {
    type Output = Self;
    fn mul(self, rhs: i32) -> Self::Output {
        Point { x: self.x * rhs, y: self.y * rhs }
    }
}

impl Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Point { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Point {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Point { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl SubAssign for Point {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}