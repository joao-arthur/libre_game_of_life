use std::fmt;

use super::poligon::rect::Rect;

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

pub fn matrix_to_cartesian(p: &MatrixP, cam: &Rect) -> CartesianP {
    CartesianP { x: cam.x1 + p.col as i64, y: cam.y2 - p.row as i64 }
}

pub fn cartesian_to_matrix(&p: &CartesianP, cam: &Rect) -> MatrixP {
    MatrixP { row: (-p.y + cam.y2) as u64, col: (p.x - cam.x1) as u64 }
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
    fn test_matrix_to_cartesian_1x1() {
        assert_eq!(matrix_to_cartesian(&MatrixP::from(0, 0), &Rect::from(0, 0, 0, 0)), CartesianP::from(0, 0));
    }

    #[test]
    fn test_matrix_to_cartesian_2x2() {
        assert_eq!(matrix_to_cartesian(&MatrixP::from(0, 0), &Rect::from(-1, -1, 0, 0)), CartesianP::from(-1, 0));
        assert_eq!(matrix_to_cartesian(&MatrixP::from(0, 1), &Rect::from(-1, -1, 0, 0)), CartesianP::from(0, 0));
        assert_eq!(matrix_to_cartesian(&MatrixP::from(1, 0), &Rect::from(-1, -1, 0, 0)), CartesianP::from(-1, -1));
        assert_eq!(matrix_to_cartesian(&MatrixP::from(1, 1), &Rect::from(-1, -1, 0, 0)), CartesianP::from(0, -1));
    }

    #[test]
    fn test_matrix_to_cartesian_3x3() {
        assert_eq!(matrix_to_cartesian(&MatrixP::from(0, 0), &Rect::from(-1, -1, 1, 1)), CartesianP::from(-1, 1));
        assert_eq!(matrix_to_cartesian(&MatrixP::from(0, 1), &Rect::from(-1, -1, 1, 1)), CartesianP::from(0, 1));
        assert_eq!(matrix_to_cartesian(&MatrixP::from(0, 2), &Rect::from(-1, -1, 1, 1)), CartesianP::from(1, 1));
        assert_eq!(matrix_to_cartesian(&MatrixP::from(1, 0), &Rect::from(-1, -1, 1, 1)), CartesianP::from(-1, 0));
        assert_eq!(matrix_to_cartesian(&MatrixP::from(1, 1), &Rect::from(-1, -1, 1, 1)), CartesianP::from(0, 0));
        assert_eq!(matrix_to_cartesian(&MatrixP::from(1, 2), &Rect::from(-1, -1, 1, 1)), CartesianP::from(1, 0));
        assert_eq!(matrix_to_cartesian(&MatrixP::from(2, 0), &Rect::from(-1, -1, 1, 1)), CartesianP::from(-1, -1));
        assert_eq!(matrix_to_cartesian(&MatrixP::from(2, 1), &Rect::from(-1, -1, 1, 1)), CartesianP::from(0, -1));
        assert_eq!(matrix_to_cartesian(&MatrixP::from(2, 2), &Rect::from(-1, -1, 1, 1)), CartesianP::from(1, -1));
    }

    #[test]
    fn test_matrix_to_cartesian_4x4() {
        assert_eq!(matrix_to_cartesian(&MatrixP::from(0, 0), &Rect::from(-2, -2, 1, 1)), CartesianP::from(-2, 1));
        assert_eq!(matrix_to_cartesian(&MatrixP::from(0, 1), &Rect::from(-2, -2, 1, 1)), CartesianP::from(-1, 1));
        assert_eq!(matrix_to_cartesian(&MatrixP::from(0, 2), &Rect::from(-2, -2, 1, 1)), CartesianP::from(0, 1));
        assert_eq!(matrix_to_cartesian(&MatrixP::from(0, 3), &Rect::from(-2, -2, 1, 1)), CartesianP::from(1, 1));
        assert_eq!(matrix_to_cartesian(&MatrixP::from(1, 0), &Rect::from(-2, -2, 1, 1)), CartesianP::from(-2, 0));
        assert_eq!(matrix_to_cartesian(&MatrixP::from(1, 1), &Rect::from(-2, -2, 1, 1)), CartesianP::from(-1, 0));
        assert_eq!(matrix_to_cartesian(&MatrixP::from(1, 2), &Rect::from(-2, -2, 1, 1)), CartesianP::from(0, 0));
        assert_eq!(matrix_to_cartesian(&MatrixP::from(1, 3), &Rect::from(-2, -2, 1, 1)), CartesianP::from(1, 0));
        assert_eq!(matrix_to_cartesian(&MatrixP::from(2, 0), &Rect::from(-2, -2, 1, 1)), CartesianP::from(-2, -1));
        assert_eq!(matrix_to_cartesian(&MatrixP::from(2, 1), &Rect::from(-2, -2, 1, 1)), CartesianP::from(-1, -1));
        assert_eq!(matrix_to_cartesian(&MatrixP::from(2, 2), &Rect::from(-2, -2, 1, 1)), CartesianP::from(0, -1));
        assert_eq!(matrix_to_cartesian(&MatrixP::from(2, 3), &Rect::from(-2, -2, 1, 1)), CartesianP::from(1, -1));
        assert_eq!(matrix_to_cartesian(&MatrixP::from(3, 0), &Rect::from(-2, -2, 1, 1)), CartesianP::from(-2, -2));
        assert_eq!(matrix_to_cartesian(&MatrixP::from(3, 1), &Rect::from(-2, -2, 1, 1)), CartesianP::from(-1, -2));
        assert_eq!(matrix_to_cartesian(&MatrixP::from(3, 2), &Rect::from(-2, -2, 1, 1)), CartesianP::from(0, -2));
        assert_eq!(matrix_to_cartesian(&MatrixP::from(3, 3), &Rect::from(-2, -2, 1, 1)), CartesianP::from(1, -2));
    }

    #[test]
    fn test_cartesian_to_matrix_1x1() {
        assert_eq!(cartesian_to_matrix(&CartesianP::from(0, 0), &Rect::from(0, 0, 0, 0)), MatrixP::from(0, 0));
    }

    #[test]
    fn test_cartesian_to_matrix_2x2() {
        assert_eq!(cartesian_to_matrix(&CartesianP::from(-1, 0), &Rect::from(-1, -1, 0, 0)), MatrixP::from(0, 0));
        assert_eq!(cartesian_to_matrix(&CartesianP::from(0, 0), &Rect::from(-1, -1, 0, 0)), MatrixP::from(0, 1));
        assert_eq!(cartesian_to_matrix(&CartesianP::from(-1, -1), &Rect::from(-1, -1, 0, 0)), MatrixP::from(1, 0));
        assert_eq!(cartesian_to_matrix(&CartesianP::from(0, -1), &Rect::from(-1, -1, 0, 0)), MatrixP::from(1, 1));
    }

    #[test]
    fn test_cartesian_to_matrix_3x3() {
        assert_eq!(cartesian_to_matrix(&CartesianP::from(-1, 1), &Rect::from(-1, -1, 1, 1)), MatrixP::from(0, 0));
        assert_eq!(cartesian_to_matrix(&CartesianP::from(0, 1), &Rect::from(-1, -1, 1, 1)), MatrixP::from(0, 1));
        assert_eq!(cartesian_to_matrix(&CartesianP::from(1, 1), &Rect::from(-1, -1, 1, 1)), MatrixP::from(0, 2));
        assert_eq!(cartesian_to_matrix(&CartesianP::from(-1, 0), &Rect::from(-1, -1, 1, 1)), MatrixP::from(1, 0));
        assert_eq!(cartesian_to_matrix(&CartesianP::from(0, 0), &Rect::from(-1, -1, 1, 1)), MatrixP::from(1, 1));
        assert_eq!(cartesian_to_matrix(&CartesianP::from(1, 0), &Rect::from(-1, -1, 1, 1)), MatrixP::from(1, 2));
        assert_eq!(cartesian_to_matrix(&CartesianP::from(-1, -1), &Rect::from(-1, -1, 1, 1)), MatrixP::from(2, 0));
        assert_eq!(cartesian_to_matrix(&CartesianP::from(0, -1), &Rect::from(-1, -1, 1, 1)), MatrixP::from(2, 1));
        assert_eq!(cartesian_to_matrix(&CartesianP::from(1, -1), &Rect::from(-1, -1, 1, 1)), MatrixP::from(2, 2));
    }

    #[test]
    fn test_cartesian_to_matrix_4x4() {
        assert_eq!(cartesian_to_matrix(&CartesianP::from(-2, 1), &Rect::from(-2, -2, 1, 1)), MatrixP::from(0, 0));
        assert_eq!(cartesian_to_matrix(&CartesianP::from(-1, 1), &Rect::from(-2, -2, 1, 1)), MatrixP::from(0, 1));
        assert_eq!(cartesian_to_matrix(&CartesianP::from(0, 1), &Rect::from(-2, -2, 1, 1)), MatrixP::from(0, 2));
        assert_eq!(cartesian_to_matrix(&CartesianP::from(1, 1), &Rect::from(-2, -2, 1, 1)), MatrixP::from(0, 3));
        assert_eq!(cartesian_to_matrix(&CartesianP::from(-2, 0), &Rect::from(-2, -2, 1, 1)), MatrixP::from(1, 0));
        assert_eq!(cartesian_to_matrix(&CartesianP::from(-1, 0), &Rect::from(-2, -2, 1, 1)), MatrixP::from(1, 1));
        assert_eq!(cartesian_to_matrix(&CartesianP::from(0, 0), &Rect::from(-2, -2, 1, 1)), MatrixP::from(1, 2));
        assert_eq!(cartesian_to_matrix(&CartesianP::from(1, 0), &Rect::from(-2, -2, 1, 1)), MatrixP::from(1, 3));
        assert_eq!(cartesian_to_matrix(&CartesianP::from(-2, -1), &Rect::from(-2, -2, 1, 1)), MatrixP::from(2, 0));
        assert_eq!(cartesian_to_matrix(&CartesianP::from(-1, -1), &Rect::from(-2, -2, 1, 1)), MatrixP::from(2, 1));
        assert_eq!(cartesian_to_matrix(&CartesianP::from(0, -1), &Rect::from(-2, -2, 1, 1)), MatrixP::from(2, 2));
        assert_eq!(cartesian_to_matrix(&CartesianP::from(1, -1), &Rect::from(-2, -2, 1, 1)), MatrixP::from(2, 3));
        assert_eq!(cartesian_to_matrix(&CartesianP::from(-2, -2), &Rect::from(-2, -2, 1, 1)), MatrixP::from(3, 0));
        assert_eq!(cartesian_to_matrix(&CartesianP::from(-1, -2), &Rect::from(-2, -2, 1, 1)), MatrixP::from(3, 1));
        assert_eq!(cartesian_to_matrix(&CartesianP::from(0, -2), &Rect::from(-2, -2, 1, 1)), MatrixP::from(3, 2));
        assert_eq!(cartesian_to_matrix(&CartesianP::from(1, -2), &Rect::from(-2, -2, 1, 1)), MatrixP::from(3, 3));
    }

    // #[test]
    // fn test_cartesian_to_matrix_uncentered() {
    //     assert_eq!(cartesian_to_matrix(&CartesianP::from(-5, -4), &Rect::from(-4, -4, 3, 3)), MatrixP::from(3, 3));
    // }
}
