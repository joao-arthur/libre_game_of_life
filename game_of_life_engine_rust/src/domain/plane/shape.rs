use std::fmt;

pub struct Square {
    pub x: i64,
    pub y: i64,
    pub size: u64,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Rect {
    pub x1: i64,
    pub y1: i64,
    pub x2: i64,
    pub y2: i64,
}

impl Rect {
    pub fn from(x1: i64, y1: i64, x2: i64, y2: i64) -> Self {
        Rect { x1, y1, x2, y2 }
    }
}

impl fmt::Display for Rect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "(({}, {}), ({}, {}))",
            self.x1, self.y1, self.x2, self.y2
        )
    }
}
