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

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub struct CartesianP {
    pub x: i64,
    pub y: i64,
}

impl CartesianP {
    pub fn from(x: i64, y: i64) -> Self {
        CartesianP { x, y }
    }
}

impl fmt::Display for CartesianP {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

pub fn matrix_to_cartesian(p: MatrixP, length: u64) -> CartesianP {
    let len: i64 = length.try_into().unwrap();
    let col: i64 = p.col.try_into().unwrap();
    let row: i64 = p.row.try_into().unwrap();
    let half = len / 2;
    CartesianP::from(-(half) + col, half - row)
}

pub fn cartesian_to_matrix(p: CartesianP, length: u64) -> MatrixP {
    let len: i64 = length.try_into().unwrap();
    let half = len / 2;
    let row = half - p.y;
    let col = half + p.x;
    MatrixP::from(row.try_into().unwrap(), col.try_into().unwrap())
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

    #[test]
    fn test_cartesian_point() {
        let p = CartesianP::from(-23, 38);
        assert_eq!(p, CartesianP { x: -23, y: 38 });
        assert_eq!(format!("{p}"), "(-23, 38)");
    }

    #[test]
    fn test_matrix_to_cartesian_1x1_grid() {
        assert_eq!(matrix_to_cartesian(MatrixP::from(0, 0), 1), CartesianP::from(0, 0));
    }

    #[test]
    fn test_matrix_to_cartesian_2x2_grid() {
        assert_eq!(matrix_to_cartesian(MatrixP::from(0, 0), 2), CartesianP::from(-1, 1));
        assert_eq!(matrix_to_cartesian(MatrixP::from(0, 1), 2), CartesianP::from(0, 1));
        assert_eq!(matrix_to_cartesian(MatrixP::from(1, 0), 2), CartesianP::from(-1, 0));
        assert_eq!(matrix_to_cartesian(MatrixP::from(1, 1), 2), CartesianP::from(0, 0));
    }

    #[test]
    fn test_matrix_to_cartesian_3x3_grid() {
        assert_eq!(matrix_to_cartesian(MatrixP::from(0, 0), 3), CartesianP::from(-1, 1));
        assert_eq!(matrix_to_cartesian(MatrixP::from(0, 1), 3), CartesianP::from(0, 1));
        assert_eq!(matrix_to_cartesian(MatrixP::from(0, 2), 3), CartesianP::from(1, 1));
        assert_eq!(matrix_to_cartesian(MatrixP::from(1, 0), 3), CartesianP::from(-1, 0));
        assert_eq!(matrix_to_cartesian(MatrixP::from(1, 1), 3), CartesianP::from(0, 0));
        assert_eq!(matrix_to_cartesian(MatrixP::from(1, 2), 3), CartesianP::from(1, 0));
        assert_eq!(matrix_to_cartesian(MatrixP::from(2, 0), 3), CartesianP::from(-1, -1));
        assert_eq!(matrix_to_cartesian(MatrixP::from(2, 1), 3), CartesianP::from(0, -1));
        assert_eq!(matrix_to_cartesian(MatrixP::from(2, 2), 3), CartesianP::from(1, -1));
    }

    #[test]
    fn test_matrix_to_cartesian_4x4_grid() {
        assert_eq!(matrix_to_cartesian(MatrixP::from(0, 0), 4), CartesianP::from(-2, 2));
        assert_eq!(matrix_to_cartesian(MatrixP::from(0, 1), 4), CartesianP::from(-1, 2));
        assert_eq!(matrix_to_cartesian(MatrixP::from(0, 2), 4), CartesianP::from(0, 2));
        assert_eq!(matrix_to_cartesian(MatrixP::from(0, 3), 4), CartesianP::from(1, 2));
        assert_eq!(matrix_to_cartesian(MatrixP::from(1, 0), 4), CartesianP::from(-2, 1));
        assert_eq!(matrix_to_cartesian(MatrixP::from(1, 1), 4), CartesianP::from(-1, 1));
        assert_eq!(matrix_to_cartesian(MatrixP::from(1, 2), 4), CartesianP::from(0, 1));
        assert_eq!(matrix_to_cartesian(MatrixP::from(1, 3), 4), CartesianP::from(1, 1));
        assert_eq!(matrix_to_cartesian(MatrixP::from(2, 0), 4), CartesianP::from(-2, 0));
        assert_eq!(matrix_to_cartesian(MatrixP::from(2, 1), 4), CartesianP::from(-1, 0));
        assert_eq!(matrix_to_cartesian(MatrixP::from(2, 2), 4), CartesianP::from(0, 0));
        assert_eq!(matrix_to_cartesian(MatrixP::from(2, 3), 4), CartesianP::from(1, 0));
        assert_eq!(matrix_to_cartesian(MatrixP::from(3, 0), 4), CartesianP::from(-2, -1));
        assert_eq!(matrix_to_cartesian(MatrixP::from(3, 1), 4), CartesianP::from(-1, -1));
        assert_eq!(matrix_to_cartesian(MatrixP::from(3, 2), 4), CartesianP::from(0, -1));
        assert_eq!(matrix_to_cartesian(MatrixP::from(3, 3), 4), CartesianP::from(1, -1));
    }

    #[test]
    fn test_cartesian_to_matrix_1x1_grid() {
        assert_eq!(cartesian_to_matrix(CartesianP::from(0, 0), 1), MatrixP::from(0, 0));
    }

    #[test]
    fn test_cartesian_to_matrix_2x2_grid() {
        assert_eq!(cartesian_to_matrix(CartesianP::from(-1, 1), 2), MatrixP::from(0, 0));
        assert_eq!(cartesian_to_matrix(CartesianP::from(0, 1), 2), MatrixP::from(0, 1));
        assert_eq!(cartesian_to_matrix(CartesianP::from(-1, 0), 2), MatrixP::from(1, 0));
        assert_eq!(cartesian_to_matrix(CartesianP::from(0, 0), 2), MatrixP::from(1, 1));
    }

    #[test]
    fn test_cartesian_to_matrix_3x3_grid() {
        assert_eq!(cartesian_to_matrix(CartesianP::from(-1, 1), 3), MatrixP::from(0, 0));
        assert_eq!(cartesian_to_matrix(CartesianP::from(0, 1), 3), MatrixP::from(0, 1));
        assert_eq!(cartesian_to_matrix(CartesianP::from(1, 1), 3), MatrixP::from(0, 2));
        assert_eq!(cartesian_to_matrix(CartesianP::from(-1, 0), 3), MatrixP::from(1, 0));
        assert_eq!(cartesian_to_matrix(CartesianP::from(0, 0), 3), MatrixP::from(1, 1));
        assert_eq!(cartesian_to_matrix(CartesianP::from(1, 0), 3), MatrixP::from(1, 2));
        assert_eq!(cartesian_to_matrix(CartesianP::from(-1, -1), 3), MatrixP::from(2, 0));
        assert_eq!(cartesian_to_matrix(CartesianP::from(0, -1), 3), MatrixP::from(2, 1));
        assert_eq!(cartesian_to_matrix(CartesianP::from(1, -1), 3), MatrixP::from(2, 2));
    }

    #[test]
    fn test_cartesian_to_matrix_4x4_grid() {
        assert_eq!(cartesian_to_matrix(CartesianP::from(-2, 2), 4), MatrixP::from(0, 0));
        assert_eq!(cartesian_to_matrix(CartesianP::from(-1, 2), 4), MatrixP::from(0, 1));
        assert_eq!(cartesian_to_matrix(CartesianP::from(0, 2), 4), MatrixP::from(0, 2));
        assert_eq!(cartesian_to_matrix(CartesianP::from(1, 2), 4), MatrixP::from(0, 3));
        assert_eq!(cartesian_to_matrix(CartesianP::from(-2, 1), 4), MatrixP::from(1, 0));
        assert_eq!(cartesian_to_matrix(CartesianP::from(-1, 1), 4), MatrixP::from(1, 1));
        assert_eq!(cartesian_to_matrix(CartesianP::from(0, 1), 4), MatrixP::from(1, 2));
        assert_eq!(cartesian_to_matrix(CartesianP::from(1, 1), 4), MatrixP::from(1, 3));
        assert_eq!(cartesian_to_matrix(CartesianP::from(-2, 0), 4), MatrixP::from(2, 0));
        assert_eq!(cartesian_to_matrix(CartesianP::from(-1, 0), 4), MatrixP::from(2, 1));
        assert_eq!(cartesian_to_matrix(CartesianP::from(0, 0), 4), MatrixP::from(2, 2));
        assert_eq!(cartesian_to_matrix(CartesianP::from(1, 0), 4), MatrixP::from(2, 3));
        assert_eq!(cartesian_to_matrix(CartesianP::from(-2, -1), 4), MatrixP::from(3, 0));
        assert_eq!(cartesian_to_matrix(CartesianP::from(-1, -1), 4), MatrixP::from(3, 1));
        assert_eq!(cartesian_to_matrix(CartesianP::from(0, -1), 4), MatrixP::from(3, 2));
        assert_eq!(cartesian_to_matrix(CartesianP::from(1, -1), 4), MatrixP::from(3, 3));
    }
}
