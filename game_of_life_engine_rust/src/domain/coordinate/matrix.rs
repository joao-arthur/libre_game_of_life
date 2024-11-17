use std::fmt;

#[derive(Debug, PartialEq)]
pub struct MatrixP {
    pub row: u64,
    pub col: u64,
}

impl MatrixP {
    pub fn from(row: u64, col: u64) -> Self {
        MatrixP { row, col }
    }
}

impl fmt::Display for MatrixP {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_matrix_point() {
        let p = MatrixP::from(23, 38);
        assert_eq!(p, MatrixP { row: 23, col: 38 });
        assert_eq!(format!("{p}"), "(23, 38)");
    }
}
