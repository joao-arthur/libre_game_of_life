use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Sq {
    pub x: i64,
    pub y: i64,
    pub size: u64,
}

impl fmt::Display for Sq {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(({}, {}), {})", self.x, self.y, self.size)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sq() {
        let sq = Sq { x: -23, y: 38, size: 63 };
        assert_eq!(format!("{sq}"), "((-23, 38), 63)");
    }
}
