use std::ops::Add;

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub x: u16,
    pub y: u16
}

impl From<(u16, u16)> for Position {
    fn from(pos_data: (u16, u16)) -> Self {
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