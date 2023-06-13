use std::ops::{Add, AddAssign};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vec2 {
    pub x: i16,
    pub y: i16
}
impl Vec2 {
    pub const ZERO: Vec2 = Vec2 { x: 0, y: 0 };
}
impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}
impl From<(i16, i16)> for Vec2 {
    fn from(pos_data: (i16, i16)) -> Self {
        Vec2 {
            x: pos_data.0,
            y: pos_data.1
        }
    }
}
impl Add for Vec2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}