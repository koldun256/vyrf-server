use std::ops::Add;

#[derive(Debug, Clone, Copy)]
pub struct Position {
    x: i32,
    y: i32
}

impl From<(i32, i32)> for Position {
    fn from(pos_data: (i32, i32)) -> Self {
        Position {
            x: pos_data.0,
            y: pos_data.1
        }
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Position {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}