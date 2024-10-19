pub mod cartesian;
pub mod matrix;

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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rect() {
        assert_eq!(
            Rect::from(-23, 38, 198, 7),
            Rect {
                x1: -23,
                y1: 38,
                x2: 198,
                y2: 7
            }
        );
    }
}
