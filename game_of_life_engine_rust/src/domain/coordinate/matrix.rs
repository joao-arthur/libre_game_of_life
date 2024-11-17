use std::fmt;

#[derive(Debug, PartialEq)]
pub struct MatrixPoint {
    pub row: u64,
    pub col: u64,
}

impl MatrixPoint {
    pub fn from(row: u64, col: u64) -> Self {
        MatrixPoint { row, col }
    }
}

impl fmt::Display for MatrixPoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_matrix_point() {
        let p = MatrixPoint::from(23, 38);
        assert_eq!(p, MatrixPoint { row: 23, col: 38 });
        assert_eq!(format!("{p}"), "(23, 38)");
    }
}
