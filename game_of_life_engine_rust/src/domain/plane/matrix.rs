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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_arr() {
        assert_eq!(MatrixPoint::from(23, 38), MatrixPoint { row: 23, col: 38 });
    }
}
