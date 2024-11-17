use std::fmt;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub struct CartesianPoint {
    pub x: i64,
    pub y: i64,
}

impl CartesianPoint {
    pub fn from(x: i64, y: i64) -> Self {
        CartesianPoint { x, y }
    }
}

impl fmt::Display for CartesianPoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cartesian_point() {
        let p = CartesianPoint::from(-23, 38);
        assert_eq!(p, CartesianPoint { x: -23, y: 38 });
        assert_eq!(format!("{p}"), "(-23, 38)");
    }
}
