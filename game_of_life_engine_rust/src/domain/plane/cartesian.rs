use super::matrix::MatrixPoint;
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

pub fn subdivide(value: i64, unit_size: u64) -> i64 {
    let u: i64 = unit_size.try_into().unwrap();
    value / u
}

pub fn from_matrix(p: MatrixPoint, length: u64) -> CartesianPoint {
    let len: i64 = length.try_into().unwrap();
    let col: i64 = p.col.try_into().unwrap();
    let row: i64 = p.row.try_into().unwrap();
    let half = len / 2;
    CartesianPoint::from(-(half) + col, half - row)
}

pub fn to_matrix(p: CartesianPoint, length: u64) -> MatrixPoint {
    let len: i64 = length.try_into().unwrap();
    let half = len / 2;
    let row = half - p.y;
    let col = half + p.x;
    MatrixPoint::from(row.try_into().unwrap(), col.try_into().unwrap())
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

    #[test]
    fn test_absolute_to_relative() {
        assert_eq!(subdivide(0, 30), 0);
        assert_eq!(subdivide(10, 30), 0);
        assert_eq!(subdivide(20, 30), 0);
        assert_eq!(subdivide(29, 30), 0);
        assert_eq!(subdivide(30, 30), 1);
        assert_eq!(subdivide(300, 30), 10);
    }

    #[test]
    fn test_index_to_point_1x1_grid() {
        assert_eq!(
            from_matrix(MatrixPoint::from(0, 0), 1),
            CartesianPoint::from(0, 0)
        );
    }

    #[test]
    fn test_index_to_point_2x2_grid() {
        assert_eq!(
            from_matrix(MatrixPoint::from(0, 0), 2),
            CartesianPoint::from(-1, 1)
        );
        assert_eq!(
            from_matrix(MatrixPoint::from(0, 1), 2),
            CartesianPoint::from(0, 1)
        );
        assert_eq!(
            from_matrix(MatrixPoint::from(1, 0), 2),
            CartesianPoint::from(-1, 0)
        );
        assert_eq!(
            from_matrix(MatrixPoint::from(1, 1), 2),
            CartesianPoint::from(0, 0)
        );
    }

    #[test]
    fn test_index_to_point_3x3_grid() {
        assert_eq!(
            from_matrix(MatrixPoint::from(0, 0), 3),
            CartesianPoint::from(-1, 1)
        );
        assert_eq!(
            from_matrix(MatrixPoint::from(0, 1), 3),
            CartesianPoint::from(0, 1)
        );
        assert_eq!(
            from_matrix(MatrixPoint::from(0, 2), 3),
            CartesianPoint::from(1, 1)
        );
        assert_eq!(
            from_matrix(MatrixPoint::from(1, 0), 3),
            CartesianPoint::from(-1, 0)
        );
        assert_eq!(
            from_matrix(MatrixPoint::from(1, 1), 3),
            CartesianPoint::from(0, 0)
        );
        assert_eq!(
            from_matrix(MatrixPoint::from(1, 2), 3),
            CartesianPoint::from(1, 0)
        );
        assert_eq!(
            from_matrix(MatrixPoint::from(2, 0), 3),
            CartesianPoint::from(-1, -1)
        );
        assert_eq!(
            from_matrix(MatrixPoint::from(2, 1), 3),
            CartesianPoint::from(0, -1)
        );
        assert_eq!(
            from_matrix(MatrixPoint::from(2, 2), 3),
            CartesianPoint::from(1, -1)
        );
    }

    #[test]
    fn test_index_to_point_4x4_grid() {
        assert_eq!(
            from_matrix(MatrixPoint::from(0, 0), 4),
            CartesianPoint::from(-2, 2)
        );
        assert_eq!(
            from_matrix(MatrixPoint::from(0, 1), 4),
            CartesianPoint::from(-1, 2)
        );
        assert_eq!(
            from_matrix(MatrixPoint::from(0, 2), 4),
            CartesianPoint::from(0, 2)
        );
        assert_eq!(
            from_matrix(MatrixPoint::from(0, 3), 4),
            CartesianPoint::from(1, 2)
        );
        assert_eq!(
            from_matrix(MatrixPoint::from(1, 0), 4),
            CartesianPoint::from(-2, 1)
        );
        assert_eq!(
            from_matrix(MatrixPoint::from(1, 1), 4),
            CartesianPoint::from(-1, 1)
        );
        assert_eq!(
            from_matrix(MatrixPoint::from(1, 2), 4),
            CartesianPoint::from(0, 1)
        );
        assert_eq!(
            from_matrix(MatrixPoint::from(1, 3), 4),
            CartesianPoint::from(1, 1)
        );
        assert_eq!(
            from_matrix(MatrixPoint::from(2, 0), 4),
            CartesianPoint::from(-2, 0)
        );
        assert_eq!(
            from_matrix(MatrixPoint::from(2, 1), 4),
            CartesianPoint::from(-1, 0)
        );
        assert_eq!(
            from_matrix(MatrixPoint::from(2, 2), 4),
            CartesianPoint::from(0, 0)
        );
        assert_eq!(
            from_matrix(MatrixPoint::from(2, 3), 4),
            CartesianPoint::from(1, 0)
        );
        assert_eq!(
            from_matrix(MatrixPoint::from(3, 0), 4),
            CartesianPoint::from(-2, -1)
        );
        assert_eq!(
            from_matrix(MatrixPoint::from(3, 1), 4),
            CartesianPoint::from(-1, -1)
        );
        assert_eq!(
            from_matrix(MatrixPoint::from(3, 2), 4),
            CartesianPoint::from(0, -1)
        );
        assert_eq!(
            from_matrix(MatrixPoint::from(3, 3), 4),
            CartesianPoint::from(1, -1)
        );
    }

    #[test]
    fn test_point_to_index_1x1_grid() {
        assert_eq!(
            to_matrix(CartesianPoint::from(0, 0), 1),
            MatrixPoint::from(0, 0)
        );
    }

    #[test]
    fn test_point_to_index_2x2_grid() {
        assert_eq!(
            to_matrix(CartesianPoint::from(-1, 1), 2),
            MatrixPoint::from(0, 0)
        );
        assert_eq!(
            to_matrix(CartesianPoint::from(0, 1), 2),
            MatrixPoint::from(0, 1)
        );
        assert_eq!(
            to_matrix(CartesianPoint::from(-1, 0), 2),
            MatrixPoint::from(1, 0)
        );
        assert_eq!(
            to_matrix(CartesianPoint::from(0, 0), 2),
            MatrixPoint::from(1, 1)
        );
    }

    #[test]
    fn test_point_to_index_3x3_grid() {
        assert_eq!(
            to_matrix(CartesianPoint::from(-1, 1), 3),
            MatrixPoint::from(0, 0)
        );
        assert_eq!(
            to_matrix(CartesianPoint::from(0, 1), 3),
            MatrixPoint::from(0, 1)
        );
        assert_eq!(
            to_matrix(CartesianPoint::from(1, 1), 3),
            MatrixPoint::from(0, 2)
        );
        assert_eq!(
            to_matrix(CartesianPoint::from(-1, 0), 3),
            MatrixPoint::from(1, 0)
        );
        assert_eq!(
            to_matrix(CartesianPoint::from(0, 0), 3),
            MatrixPoint::from(1, 1)
        );
        assert_eq!(
            to_matrix(CartesianPoint::from(1, 0), 3),
            MatrixPoint::from(1, 2)
        );
        assert_eq!(
            to_matrix(CartesianPoint::from(-1, -1), 3),
            MatrixPoint::from(2, 0)
        );
        assert_eq!(
            to_matrix(CartesianPoint::from(0, -1), 3),
            MatrixPoint::from(2, 1)
        );
        assert_eq!(
            to_matrix(CartesianPoint::from(1, -1), 3),
            MatrixPoint::from(2, 2)
        );
    }

    #[test]
    fn test_point_to_index_4x4_grid() {
        assert_eq!(
            to_matrix(CartesianPoint::from(-2, 2), 4),
            MatrixPoint::from(0, 0)
        );
        assert_eq!(
            to_matrix(CartesianPoint::from(-1, 2), 4),
            MatrixPoint::from(0, 1)
        );
        assert_eq!(
            to_matrix(CartesianPoint::from(0, 2), 4),
            MatrixPoint::from(0, 2)
        );
        assert_eq!(
            to_matrix(CartesianPoint::from(1, 2), 4),
            MatrixPoint::from(0, 3)
        );
        assert_eq!(
            to_matrix(CartesianPoint::from(-2, 1), 4),
            MatrixPoint::from(1, 0)
        );
        assert_eq!(
            to_matrix(CartesianPoint::from(-1, 1), 4),
            MatrixPoint::from(1, 1)
        );
        assert_eq!(
            to_matrix(CartesianPoint::from(0, 1), 4),
            MatrixPoint::from(1, 2)
        );
        assert_eq!(
            to_matrix(CartesianPoint::from(1, 1), 4),
            MatrixPoint::from(1, 3)
        );
        assert_eq!(
            to_matrix(CartesianPoint::from(-2, 0), 4),
            MatrixPoint::from(2, 0)
        );
        assert_eq!(
            to_matrix(CartesianPoint::from(-1, 0), 4),
            MatrixPoint::from(2, 1)
        );
        assert_eq!(
            to_matrix(CartesianPoint::from(0, 0), 4),
            MatrixPoint::from(2, 2)
        );
        assert_eq!(
            to_matrix(CartesianPoint::from(1, 0), 4),
            MatrixPoint::from(2, 3)
        );
        assert_eq!(
            to_matrix(CartesianPoint::from(-2, -1), 4),
            MatrixPoint::from(3, 0)
        );
        assert_eq!(
            to_matrix(CartesianPoint::from(-1, -1), 4),
            MatrixPoint::from(3, 1)
        );
        assert_eq!(
            to_matrix(CartesianPoint::from(0, -1), 4),
            MatrixPoint::from(3, 2)
        );
        assert_eq!(
            to_matrix(CartesianPoint::from(1, -1), 4),
            MatrixPoint::from(3, 3)
        );
    }
}
