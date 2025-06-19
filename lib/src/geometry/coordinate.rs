use std::fmt;

use super::poligon::rect::RectI64;

#[derive(Debug, PartialEq)]
pub struct MatrixPoint {
    pub row: u64,
    pub col: u64,
}

impl MatrixPoint {
    pub fn of(row: u64, col: u64) -> Self {
        MatrixPoint { row, col }
    }
}

impl fmt::Display for MatrixPoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub struct CartesianPoint {
    pub x: i64,
    pub y: i64,
}

impl CartesianPoint {
    pub fn of(x: i64, y: i64) -> Self {
        CartesianPoint { x, y }
    }
}

impl fmt::Display for CartesianPoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

pub fn matrix_to_cartesian(p: &MatrixPoint, cam: &RectI64) -> CartesianPoint {
    CartesianPoint { x: cam.x1 + p.col as i64, y: cam.y2 - p.row as i64 }
}

pub fn cartesian_to_matrix(&p: &CartesianPoint, cam: &RectI64) -> MatrixPoint {
    MatrixPoint { row: (-p.y + cam.y2) as u64, col: (p.x - cam.x1) as u64 }
}

#[cfg(test)]
mod tests {
    use crate::geometry::{
        coordinate::{CartesianPoint, MatrixPoint},
        poligon::rect::RectI64,
    };

    use super::{cartesian_to_matrix, matrix_to_cartesian};

    #[test]
    fn matrix_point() {
        let point = MatrixPoint::of(23, 38);
        assert_eq!(point, MatrixPoint { row: 23, col: 38 });
        assert_eq!(point.to_string(), "(23, 38)");
    }

    #[test]
    fn cartesian_point() {
        let point = CartesianPoint::of(-23, 38);
        assert_eq!(point, CartesianPoint { x: -23, y: 38 });
        assert_eq!(point.to_string(), "(-23, 38)");
    }

    #[test]
    fn matrix_to_cartesian_1x1() {
        let cam = RectI64::of(0, 0, 0, 0);
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(0, 0), &cam), CartesianPoint::of(0, 0));
    }

    #[test]
    fn cartesian_to_matrix_1x1() {
        let cam = RectI64::of(0, 0, 0, 0);
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(0, 0), &cam), MatrixPoint::of(0, 0));
    }

    #[test]
    fn matrix_to_cartesian_2x2() {
        let cam = RectI64::of(-1, -1, 0, 0);
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(0, 0), &cam), CartesianPoint::of(-1, 0));
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(0, 1), &cam), CartesianPoint::of(0, 0));
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(1, 0), &cam), CartesianPoint::of(-1, -1));
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(1, 1), &cam), CartesianPoint::of(0, -1));
    }

    #[test]
    fn cartesian_to_matrix_2x2() {
        let cam = RectI64::of(-1, -1, 0, 0);
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(-1, 0), &cam), MatrixPoint::of(0, 0));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(0, 0), &cam), MatrixPoint::of(0, 1));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(-1, -1), &cam), MatrixPoint::of(1, 0));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(0, -1), &cam), MatrixPoint::of(1, 1));
    }

    #[test]
    fn matrix_to_cartesian_3x3() {
        let cam = RectI64::of(-1, -1, 1, 1);
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(0, 0), &cam), CartesianPoint::of(-1, 1));
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(0, 1), &cam), CartesianPoint::of(0, 1));
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(0, 2), &cam), CartesianPoint::of(1, 1));
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(1, 0), &cam), CartesianPoint::of(-1, 0));
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(1, 1), &cam), CartesianPoint::of(0, 0));
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(1, 2), &cam), CartesianPoint::of(1, 0));
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(2, 0), &cam), CartesianPoint::of(-1, -1));
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(2, 1), &cam), CartesianPoint::of(0, -1));
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(2, 2), &cam), CartesianPoint::of(1, -1));
    }

    #[test]
    fn cartesian_to_matrix_3x3() {
        let cam = RectI64::of(-1, -1, 1, 1);
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(-1, 1), &cam), MatrixPoint::of(0, 0));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(0, 1), &cam), MatrixPoint::of(0, 1));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(1, 1), &cam), MatrixPoint::of(0, 2));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(-1, 0), &cam), MatrixPoint::of(1, 0));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(0, 0), &cam), MatrixPoint::of(1, 1));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(1, 0), &cam), MatrixPoint::of(1, 2));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(-1, -1), &cam), MatrixPoint::of(2, 0));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(0, -1), &cam), MatrixPoint::of(2, 1));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(1, -1), &cam), MatrixPoint::of(2, 2));
    }

    #[test]
    fn matrix_to_cartesian_4x4() {
        let cam = RectI64::of(-2, -2, 1, 1);
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(0, 0), &cam), CartesianPoint::of(-2, 1));
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(0, 1), &cam), CartesianPoint::of(-1, 1));
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(0, 2), &cam), CartesianPoint::of(0, 1));
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(0, 3), &cam), CartesianPoint::of(1, 1));
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(1, 0), &cam), CartesianPoint::of(-2, 0));
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(1, 1), &cam), CartesianPoint::of(-1, 0));
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(1, 2), &cam), CartesianPoint::of(0, 0));
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(1, 3), &cam), CartesianPoint::of(1, 0));
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(2, 0), &cam), CartesianPoint::of(-2, -1));
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(2, 1), &cam), CartesianPoint::of(-1, -1));
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(2, 2), &cam), CartesianPoint::of(0, -1));
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(2, 3), &cam), CartesianPoint::of(1, -1));
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(3, 0), &cam), CartesianPoint::of(-2, -2));
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(3, 1), &cam), CartesianPoint::of(-1, -2));
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(3, 2), &cam), CartesianPoint::of(0, -2));
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(3, 3), &cam), CartesianPoint::of(1, -2));
    }

    #[test]
    fn cartesian_to_matrix_4x4() {
        let cam = RectI64::of(-2, -2, 1, 1);
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(-2, 1), &cam), MatrixPoint::of(0, 0));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(-1, 1), &cam), MatrixPoint::of(0, 1));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(0, 1), &cam), MatrixPoint::of(0, 2));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(1, 1), &cam), MatrixPoint::of(0, 3));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(-2, 0), &cam), MatrixPoint::of(1, 0));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(-1, 0), &cam), MatrixPoint::of(1, 1));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(0, 0), &cam), MatrixPoint::of(1, 2));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(1, 0), &cam), MatrixPoint::of(1, 3));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(-2, -1), &cam), MatrixPoint::of(2, 0));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(-1, -1), &cam), MatrixPoint::of(2, 1));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(0, -1), &cam), MatrixPoint::of(2, 2));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(1, -1), &cam), MatrixPoint::of(2, 3));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(-2, -2), &cam), MatrixPoint::of(3, 0));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(-1, -2), &cam), MatrixPoint::of(3, 1));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(0, -2), &cam), MatrixPoint::of(3, 2));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(1, -2), &cam), MatrixPoint::of(3, 3));
    }

    #[test]
    fn test_matrix_to_cartesian_cam_negative() {
        let cam = RectI64::of(-10, -5, -8, -3);
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(0, 0), &cam), CartesianPoint::of(-10, -3));
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(0, 1), &cam), CartesianPoint::of(-9, -3));
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(0, 2), &cam), CartesianPoint::of(-8, -3));
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(1, 0), &cam), CartesianPoint::of(-10, -4));
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(1, 1), &cam), CartesianPoint::of(-9, -4));
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(1, 2), &cam), CartesianPoint::of(-8, -4));
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(2, 0), &cam), CartesianPoint::of(-10, -5));
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(2, 1), &cam), CartesianPoint::of(-9, -5));
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(2, 2), &cam), CartesianPoint::of(-8, -5));
    }

    #[test]
    fn cartesian_to_matrix_cam_negative() {
        let cam = RectI64::of(-10, -5, -8, -3);
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(-10, -3), &cam), MatrixPoint::of(0, 0));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(-9, -3), &cam), MatrixPoint::of(0, 1));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(-8, -3), &cam), MatrixPoint::of(0, 2));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(-10, -4), &cam), MatrixPoint::of(1, 0));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(-9, -4), &cam), MatrixPoint::of(1, 1));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(-8, -4), &cam), MatrixPoint::of(1, 2));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(-10, -5), &cam), MatrixPoint::of(2, 0));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(-9, -5), &cam), MatrixPoint::of(2, 1));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(-8, -5), &cam), MatrixPoint::of(2, 2));
    }

    #[test]
    fn matrix_to_cartesian_cam_positive() {
        let cam = RectI64::of(3, 5, 5, 7);
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(0, 0), &cam), CartesianPoint::of(3, 7));
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(0, 1), &cam), CartesianPoint::of(4, 7));
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(0, 2), &cam), CartesianPoint::of(5, 7));
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(1, 0), &cam), CartesianPoint::of(3, 6));
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(1, 1), &cam), CartesianPoint::of(4, 6));
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(1, 2), &cam), CartesianPoint::of(5, 6));
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(2, 0), &cam), CartesianPoint::of(3, 5));
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(2, 1), &cam), CartesianPoint::of(4, 5));
        assert_eq!(matrix_to_cartesian(&MatrixPoint::of(2, 2), &cam), CartesianPoint::of(5, 5));
    }

    #[test]
    fn cartesian_to_matrix_cam_positive() {
        let cam = RectI64::of(3, 5, 5, 7);
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(3, 7), &cam), MatrixPoint::of(0, 0));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(4, 7), &cam), MatrixPoint::of(0, 1));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(5, 7), &cam), MatrixPoint::of(0, 2));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(3, 6), &cam), MatrixPoint::of(1, 0));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(4, 6), &cam), MatrixPoint::of(1, 1));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(5, 6), &cam), MatrixPoint::of(1, 2));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(3, 5), &cam), MatrixPoint::of(2, 0));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(4, 5), &cam), MatrixPoint::of(2, 1));
        assert_eq!(cartesian_to_matrix(&CartesianPoint::of(5, 5), &cam), MatrixPoint::of(2, 2));
    }
}
