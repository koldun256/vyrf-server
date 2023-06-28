use std::ops::{Add, AddAssign, Mul};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vector2 {
    pub x: i16,
    pub y: i16
}
impl Vector2 {
    pub const ZERO: Vector2 = Vector2 { x: 0, y: 0 };
}
impl Mul<i16> for Vector2 {
    type Output = Self;
    fn mul(self, rhs: i16) -> Self {
        Vector2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
impl AddAssign for Vector2 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}
impl From<(i16, i16)> for Vector2 {
    fn from(pos_data: (i16, i16)) -> Self {
        Vector2 {
            x: pos_data.0,
            y: pos_data.1
        }
    }
}
impl Add for Vector2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vector2 {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}
