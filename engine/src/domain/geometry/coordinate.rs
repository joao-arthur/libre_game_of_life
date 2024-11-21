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
        let cam = Rect::from(0, 0, 0, 0);
        assert_eq!(matrix_to_cartesian(&MatrixP::from(0, 0), &cam), CartesianP::from(0, 0));
    }

    #[test]
    fn test_cartesian_to_matrix_1x1() {
        let cam = Rect::from(0, 0, 0, 0);
        assert_eq!(cartesian_to_matrix(&CartesianP::from(0, 0), &cam), MatrixP::from(0, 0));
    }

    #[test]
    fn test_matrix_to_cartesian_2x2() {
        let cam = Rect::from(-1, -1, 0, 0);
        assert_eq!(matrix_to_cartesian(&MatrixP::from(0, 0), &cam), CartesianP::from(-1, 0));
        assert_eq!(matrix_to_cartesian(&MatrixP::from(0, 1), &cam), CartesianP::from(0, 0));
        assert_eq!(matrix_to_cartesian(&MatrixP::from(1, 0), &cam), CartesianP::from(-1, -1));
        assert_eq!(matrix_to_cartesian(&MatrixP::from(1, 1), &cam), CartesianP::from(0, -1));
    }

    #[test]
    fn test_cartesian_to_matrix_2x2() {
        let cam = Rect::from(-1, -1, 0, 0);
        assert_eq!(cartesian_to_matrix(&CartesianP::from(-1, 0), &cam), MatrixP::from(0, 0));
        assert_eq!(cartesian_to_matrix(&CartesianP::from(0, 0), &cam), MatrixP::from(0, 1));
        assert_eq!(cartesian_to_matrix(&CartesianP::from(-1, -1), &cam), MatrixP::from(1, 0));
        assert_eq!(cartesian_to_matrix(&CartesianP::from(0, -1), &cam), MatrixP::from(1, 1));
    }

    #[test]
    fn test_matrix_to_cartesian_3x3() {
        let cam = Rect::from(-1, -1, 1, 1);
        assert_eq!(matrix_to_cartesian(&MatrixP::from(0, 0), &cam), CartesianP::from(-1, 1));
        assert_eq!(matrix_to_cartesian(&MatrixP::from(0, 1), &cam), CartesianP::from(0, 1));
        assert_eq!(matrix_to_cartesian(&MatrixP::from(0, 2), &cam), CartesianP::from(1, 1));
        assert_eq!(matrix_to_cartesian(&MatrixP::from(1, 0), &cam), CartesianP::from(-1, 0));
        assert_eq!(matrix_to_cartesian(&MatrixP::from(1, 1), &cam), CartesianP::from(0, 0));
        assert_eq!(matrix_to_cartesian(&MatrixP::from(1, 2), &cam), CartesianP::from(1, 0));
        assert_eq!(matrix_to_cartesian(&MatrixP::from(2, 0), &cam), CartesianP::from(-1, -1));
        assert_eq!(matrix_to_cartesian(&MatrixP::from(2, 1), &cam), CartesianP::from(0, -1));
        assert_eq!(matrix_to_cartesian(&MatrixP::from(2, 2), &cam), CartesianP::from(1, -1));
    }

    #[test]
    fn test_cartesian_to_matrix_3x3() {
        let cam = Rect::from(-1, -1, 1, 1);
        assert_eq!(cartesian_to_matrix(&CartesianP::from(-1, 1), &cam), MatrixP::from(0, 0));
        assert_eq!(cartesian_to_matrix(&CartesianP::from(0, 1), &cam), MatrixP::from(0, 1));
        assert_eq!(cartesian_to_matrix(&CartesianP::from(1, 1), &cam), MatrixP::from(0, 2));
        assert_eq!(cartesian_to_matrix(&CartesianP::from(-1, 0), &cam), MatrixP::from(1, 0));
        assert_eq!(cartesian_to_matrix(&CartesianP::from(0, 0), &cam), MatrixP::from(1, 1));
        assert_eq!(cartesian_to_matrix(&CartesianP::from(1, 0), &cam), MatrixP::from(1, 2));
        assert_eq!(cartesian_to_matrix(&CartesianP::from(-1, -1), &cam), MatrixP::from(2, 0));
        assert_eq!(cartesian_to_matrix(&CartesianP::from(0, -1), &cam), MatrixP::from(2, 1));
        assert_eq!(cartesian_to_matrix(&CartesianP::from(1, -1), &cam), MatrixP::from(2, 2));
    }

    #[test]
    fn test_matrix_to_cartesian_4x4() {
        let cam = Rect::from(-2, -2, 1, 1);
        assert_eq!(matrix_to_cartesian(&MatrixP::from(0, 0), &cam), CartesianP::from(-2, 1));
        assert_eq!(matrix_to_cartesian(&MatrixP::from(0, 1), &cam), CartesianP::from(-1, 1));
        assert_eq!(matrix_to_cartesian(&MatrixP::from(0, 2), &cam), CartesianP::from(0, 1));
        assert_eq!(matrix_to_cartesian(&MatrixP::from(0, 3), &cam), CartesianP::from(1, 1));
        assert_eq!(matrix_to_cartesian(&MatrixP::from(1, 0), &cam), CartesianP::from(-2, 0));
        assert_eq!(matrix_to_cartesian(&MatrixP::from(1, 1), &cam), CartesianP::from(-1, 0));
        assert_eq!(matrix_to_cartesian(&MatrixP::from(1, 2), &cam), CartesianP::from(0, 0));
        assert_eq!(matrix_to_cartesian(&MatrixP::from(1, 3), &cam), CartesianP::from(1, 0));
        assert_eq!(matrix_to_cartesian(&MatrixP::from(2, 0), &cam), CartesianP::from(-2, -1));
        assert_eq!(matrix_to_cartesian(&MatrixP::from(2, 1), &cam), CartesianP::from(-1, -1));
        assert_eq!(matrix_to_cartesian(&MatrixP::from(2, 2), &cam), CartesianP::from(0, -1));
        assert_eq!(matrix_to_cartesian(&MatrixP::from(2, 3), &cam), CartesianP::from(1, -1));
        assert_eq!(matrix_to_cartesian(&MatrixP::from(3, 0), &cam), CartesianP::from(-2, -2));
        assert_eq!(matrix_to_cartesian(&MatrixP::from(3, 1), &cam), CartesianP::from(-1, -2));
        assert_eq!(matrix_to_cartesian(&MatrixP::from(3, 2), &cam), CartesianP::from(0, -2));
        assert_eq!(matrix_to_cartesian(&MatrixP::from(3, 3), &cam), CartesianP::from(1, -2));
    }

    #[test]
    fn test_cartesian_to_matrix_4x4() {
        let cam = Rect::from(-2, -2, 1, 1);
        assert_eq!(cartesian_to_matrix(&CartesianP::from(-2, 1), &cam), MatrixP::from(0, 0));
        assert_eq!(cartesian_to_matrix(&CartesianP::from(-1, 1), &cam), MatrixP::from(0, 1));
        assert_eq!(cartesian_to_matrix(&CartesianP::from(0, 1), &cam), MatrixP::from(0, 2));
        assert_eq!(cartesian_to_matrix(&CartesianP::from(1, 1), &cam), MatrixP::from(0, 3));
        assert_eq!(cartesian_to_matrix(&CartesianP::from(-2, 0), &cam), MatrixP::from(1, 0));
        assert_eq!(cartesian_to_matrix(&CartesianP::from(-1, 0), &cam), MatrixP::from(1, 1));
        assert_eq!(cartesian_to_matrix(&CartesianP::from(0, 0), &cam), MatrixP::from(1, 2));
        assert_eq!(cartesian_to_matrix(&CartesianP::from(1, 0), &cam), MatrixP::from(1, 3));
        assert_eq!(cartesian_to_matrix(&CartesianP::from(-2, -1), &cam), MatrixP::from(2, 0));
        assert_eq!(cartesian_to_matrix(&CartesianP::from(-1, -1), &cam), MatrixP::from(2, 1));
        assert_eq!(cartesian_to_matrix(&CartesianP::from(0, -1), &cam), MatrixP::from(2, 2));
        assert_eq!(cartesian_to_matrix(&CartesianP::from(1, -1), &cam), MatrixP::from(2, 3));
        assert_eq!(cartesian_to_matrix(&CartesianP::from(-2, -2), &cam), MatrixP::from(3, 0));
        assert_eq!(cartesian_to_matrix(&CartesianP::from(-1, -2), &cam), MatrixP::from(3, 1));
        assert_eq!(cartesian_to_matrix(&CartesianP::from(0, -2), &cam), MatrixP::from(3, 2));
        assert_eq!(cartesian_to_matrix(&CartesianP::from(1, -2), &cam), MatrixP::from(3, 3));
    }

    #[test]
    fn test_matrix_to_cartesian_cam_negative() {
        let cam = Rect::from(-10, -5, -8, -3);
        assert_eq!(matrix_to_cartesian(&MatrixP::from(0, 0), &cam), CartesianP::from(-10, -3));
        assert_eq!(matrix_to_cartesian(&MatrixP::from(0, 1), &cam), CartesianP::from(-9, -3));
        assert_eq!(matrix_to_cartesian(&MatrixP::from(0, 2), &cam), CartesianP::from(-8, -3));
        assert_eq!(matrix_to_cartesian(&MatrixP::from(1, 0), &cam), CartesianP::from(-10, -4));
        assert_eq!(matrix_to_cartesian(&MatrixP::from(1, 1), &cam), CartesianP::from(-9, -4));
        assert_eq!(matrix_to_cartesian(&MatrixP::from(1, 2), &cam), CartesianP::from(-8, -4));
        assert_eq!(matrix_to_cartesian(&MatrixP::from(2, 0), &cam), CartesianP::from(-10, -5));
        assert_eq!(matrix_to_cartesian(&MatrixP::from(2, 1), &cam), CartesianP::from(-9, -5));
        assert_eq!(matrix_to_cartesian(&MatrixP::from(2, 2), &cam), CartesianP::from(-8, -5));
    }

    #[test]
    fn test_cartesian_to_matrix_cam_negative() {
        let cam = Rect::from(-10, -5, -8, -3);
        assert_eq!(cartesian_to_matrix(&CartesianP::from(-10, -3), &cam), MatrixP::from(0, 0));
        assert_eq!(cartesian_to_matrix(&CartesianP::from(-9, -3), &cam), MatrixP::from(0, 1));
        assert_eq!(cartesian_to_matrix(&CartesianP::from(-8, -3), &cam), MatrixP::from(0, 2));
        assert_eq!(cartesian_to_matrix(&CartesianP::from(-10, -4), &cam), MatrixP::from(1, 0));
        assert_eq!(cartesian_to_matrix(&CartesianP::from(-9, -4), &cam), MatrixP::from(1, 1));
        assert_eq!(cartesian_to_matrix(&CartesianP::from(-8, -4), &cam), MatrixP::from(1, 2));
        assert_eq!(cartesian_to_matrix(&CartesianP::from(-10, -5), &cam), MatrixP::from(2, 0));
        assert_eq!(cartesian_to_matrix(&CartesianP::from(-9, -5), &cam), MatrixP::from(2, 1));
        assert_eq!(cartesian_to_matrix(&CartesianP::from(-8, -5), &cam), MatrixP::from(2, 2));
    }

    #[test]
    fn test_matrix_to_cartesian_cam_pos() {
        let cam = Rect::from(3, 5, 5, 7);
        assert_eq!(matrix_to_cartesian(&MatrixP::from(0, 0), &cam), CartesianP::from(3, 7));
        assert_eq!(matrix_to_cartesian(&MatrixP::from(0, 1), &cam), CartesianP::from(4, 7));
        assert_eq!(matrix_to_cartesian(&MatrixP::from(0, 2), &cam), CartesianP::from(5, 7));
        assert_eq!(matrix_to_cartesian(&MatrixP::from(1, 0), &cam), CartesianP::from(3, 6));
        assert_eq!(matrix_to_cartesian(&MatrixP::from(1, 1), &cam), CartesianP::from(4, 6));
        assert_eq!(matrix_to_cartesian(&MatrixP::from(1, 2), &cam), CartesianP::from(5, 6));
        assert_eq!(matrix_to_cartesian(&MatrixP::from(2, 0), &cam), CartesianP::from(3, 5));
        assert_eq!(matrix_to_cartesian(&MatrixP::from(2, 1), &cam), CartesianP::from(4, 5));
        assert_eq!(matrix_to_cartesian(&MatrixP::from(2, 2), &cam), CartesianP::from(5, 5));
    }

    #[test]
    fn test_cartesian_to_matrix_cam_pos() {
        let cam = Rect::from(3, 5, 5, 7);
        assert_eq!(cartesian_to_matrix(&CartesianP::from(3, 7), &cam), MatrixP::from(0, 0));
        assert_eq!(cartesian_to_matrix(&CartesianP::from(4, 7), &cam), MatrixP::from(0, 1));
        assert_eq!(cartesian_to_matrix(&CartesianP::from(5, 7), &cam), MatrixP::from(0, 2));
        assert_eq!(cartesian_to_matrix(&CartesianP::from(3, 6), &cam), MatrixP::from(1, 0));
        assert_eq!(cartesian_to_matrix(&CartesianP::from(4, 6), &cam), MatrixP::from(1, 1));
        assert_eq!(cartesian_to_matrix(&CartesianP::from(5, 6), &cam), MatrixP::from(1, 2));
        assert_eq!(cartesian_to_matrix(&CartesianP::from(3, 5), &cam), MatrixP::from(2, 0));
        assert_eq!(cartesian_to_matrix(&CartesianP::from(4, 5), &cam), MatrixP::from(2, 1));
        assert_eq!(cartesian_to_matrix(&CartesianP::from(5, 5), &cam), MatrixP::from(2, 2));
    }
}
