use super::matrix::MatrixPoint;
use super::cartesian::CartesianPoint;

pub fn matrix_to_cartesian(p: MatrixPoint, length: u64) -> CartesianPoint {
    let len: i64 = length.try_into().unwrap();
    let col: i64 = p.col.try_into().unwrap();
    let row: i64 = p.row.try_into().unwrap();
    let half = len / 2;
    CartesianPoint::from(-(half) + col, half - row)
}

pub fn cartesian_to_matrix(p: CartesianPoint, length: u64) -> MatrixPoint {
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
    fn test_index_to_point_1x1_grid() {
        assert_eq!(matrix_to_cartesian(MatrixPoint::from(0, 0), 1), CartesianPoint::from(0, 0));
    }

    #[test]
    fn test_index_to_point_2x2_grid() {
        assert_eq!(matrix_to_cartesian(MatrixPoint::from(0, 0), 2),CartesianPoint::from(-1, 1));
        assert_eq!(matrix_to_cartesian(MatrixPoint::from(0, 1), 2), CartesianPoint::from(0, 1));
        assert_eq!(matrix_to_cartesian(MatrixPoint::from(1, 0), 2), CartesianPoint::from(-1, 0));
        assert_eq!(matrix_to_cartesian(MatrixPoint::from(1, 1), 2), CartesianPoint::from(0, 0));
    }

    #[test]
    fn test_index_to_point_3x3_grid() {
        assert_eq!(matrix_to_cartesian(MatrixPoint::from(0, 0), 3), CartesianPoint::from(-1, 1));
        assert_eq!(matrix_to_cartesian(MatrixPoint::from(0, 1), 3), CartesianPoint::from(0, 1));
        assert_eq!(matrix_to_cartesian(MatrixPoint::from(0, 2), 3), CartesianPoint::from(1, 1));
        assert_eq!(matrix_to_cartesian(MatrixPoint::from(1, 0), 3), CartesianPoint::from(-1, 0));
        assert_eq!(matrix_to_cartesian(MatrixPoint::from(1, 1), 3), CartesianPoint::from(0, 0));
        assert_eq!(matrix_to_cartesian(MatrixPoint::from(1, 2), 3), CartesianPoint::from(1, 0));
        assert_eq!(matrix_to_cartesian(MatrixPoint::from(2, 0), 3), CartesianPoint::from(-1, -1));
        assert_eq!(matrix_to_cartesian(MatrixPoint::from(2, 1), 3), CartesianPoint::from(0, -1));
        assert_eq!(matrix_to_cartesian(MatrixPoint::from(2, 2), 3), CartesianPoint::from(1, -1));
    }

    #[test]
    fn test_index_to_point_4x4_grid() {
        assert_eq!(matrix_to_cartesian(MatrixPoint::from(0, 0), 4), CartesianPoint::from(-2, 2));
        assert_eq!(matrix_to_cartesian(MatrixPoint::from(0, 1), 4), CartesianPoint::from(-1, 2));
        assert_eq!(matrix_to_cartesian(MatrixPoint::from(0, 2), 4), CartesianPoint::from(0, 2));
        assert_eq!(matrix_to_cartesian(MatrixPoint::from(0, 3), 4), CartesianPoint::from(1, 2));
        assert_eq!(matrix_to_cartesian(MatrixPoint::from(1, 0), 4), CartesianPoint::from(-2, 1));
        assert_eq!(matrix_to_cartesian(MatrixPoint::from(1, 1), 4), CartesianPoint::from(-1, 1));
        assert_eq!(matrix_to_cartesian(MatrixPoint::from(1, 2), 4), CartesianPoint::from(0, 1));
        assert_eq!(matrix_to_cartesian(MatrixPoint::from(1, 3), 4), CartesianPoint::from(1, 1));
        assert_eq!(matrix_to_cartesian(MatrixPoint::from(2, 0), 4), CartesianPoint::from(-2, 0));
        assert_eq!(matrix_to_cartesian(MatrixPoint::from(2, 1), 4), CartesianPoint::from(-1, 0));
        assert_eq!(matrix_to_cartesian(MatrixPoint::from(2, 2), 4), CartesianPoint::from(0, 0));
        assert_eq!(matrix_to_cartesian(MatrixPoint::from(2, 3), 4), CartesianPoint::from(1, 0));
        assert_eq!(matrix_to_cartesian(MatrixPoint::from(3, 0), 4), CartesianPoint::from(-2, -1));
        assert_eq!(matrix_to_cartesian(MatrixPoint::from(3, 1), 4), CartesianPoint::from(-1, -1));
        assert_eq!(matrix_to_cartesian(MatrixPoint::from(3, 2), 4), CartesianPoint::from(0, -1));
        assert_eq!(matrix_to_cartesian(MatrixPoint::from(3, 3), 4), CartesianPoint::from(1, -1));
    }

    #[test]
    fn test_point_to_index_1x1_grid() {
        assert_eq!(cartesian_to_matrix(CartesianPoint::from(0, 0), 1), MatrixPoint::from(0, 0));
    }

    #[test]
    fn test_point_to_index_2x2_grid() {
        assert_eq!(cartesian_to_matrix(CartesianPoint::from(-1, 1), 2), MatrixPoint::from(0, 0));
        assert_eq!(cartesian_to_matrix(CartesianPoint::from(0, 1), 2), MatrixPoint::from(0, 1));
        assert_eq!(cartesian_to_matrix(CartesianPoint::from(-1, 0), 2), MatrixPoint::from(1, 0));
        assert_eq!(cartesian_to_matrix(CartesianPoint::from(0, 0), 2), MatrixPoint::from(1, 1));
    }

    #[test]
    fn test_point_to_index_3x3_grid() {
        assert_eq!(cartesian_to_matrix(CartesianPoint::from(-1, 1), 3), MatrixPoint::from(0, 0));
        assert_eq!(cartesian_to_matrix(CartesianPoint::from(0, 1), 3), MatrixPoint::from(0, 1));
        assert_eq!(cartesian_to_matrix(CartesianPoint::from(1, 1), 3), MatrixPoint::from(0, 2));
        assert_eq!(cartesian_to_matrix(CartesianPoint::from(-1, 0), 3), MatrixPoint::from(1, 0));
        assert_eq!(cartesian_to_matrix(CartesianPoint::from(0, 0), 3), MatrixPoint::from(1, 1));
        assert_eq!(cartesian_to_matrix(CartesianPoint::from(1, 0), 3), MatrixPoint::from(1, 2));
        assert_eq!(cartesian_to_matrix(CartesianPoint::from(-1, -1), 3), MatrixPoint::from(2, 0));
        assert_eq!(cartesian_to_matrix(CartesianPoint::from(0, -1), 3), MatrixPoint::from(2, 1));
        assert_eq!(cartesian_to_matrix(CartesianPoint::from(1, -1), 3), MatrixPoint::from(2, 2));
    }

    #[test]
    fn test_point_to_index_4x4_grid() {
        assert_eq!(cartesian_to_matrix(CartesianPoint::from(-2, 2), 4), MatrixPoint::from(0, 0));
        assert_eq!(cartesian_to_matrix(CartesianPoint::from(-1, 2), 4), MatrixPoint::from(0, 1));
        assert_eq!(cartesian_to_matrix(CartesianPoint::from(0, 2), 4), MatrixPoint::from(0, 2));
        assert_eq!(cartesian_to_matrix(CartesianPoint::from(1, 2), 4), MatrixPoint::from(0, 3));
        assert_eq!(cartesian_to_matrix(CartesianPoint::from(-2, 1), 4), MatrixPoint::from(1, 0));
        assert_eq!(cartesian_to_matrix(CartesianPoint::from(-1, 1), 4), MatrixPoint::from(1, 1));
        assert_eq!(cartesian_to_matrix(CartesianPoint::from(0, 1), 4), MatrixPoint::from(1, 2));
        assert_eq!(cartesian_to_matrix(CartesianPoint::from(1, 1), 4), MatrixPoint::from(1, 3));
        assert_eq!(cartesian_to_matrix(CartesianPoint::from(-2, 0), 4), MatrixPoint::from(2, 0));
        assert_eq!(cartesian_to_matrix(CartesianPoint::from(-1, 0), 4), MatrixPoint::from(2, 1));
        assert_eq!(cartesian_to_matrix(CartesianPoint::from(0, 0), 4), MatrixPoint::from(2, 2));
        assert_eq!(cartesian_to_matrix(CartesianPoint::from(1, 0), 4), MatrixPoint::from(2, 3));
        assert_eq!(cartesian_to_matrix(CartesianPoint::from(-2, -1), 4), MatrixPoint::from(3, 0));
        assert_eq!(cartesian_to_matrix(CartesianPoint::from(-1, -1), 4), MatrixPoint::from(3, 1));
        assert_eq!(cartesian_to_matrix(CartesianPoint::from(0, -1), 4), MatrixPoint::from(3, 2));
        assert_eq!(cartesian_to_matrix(CartesianPoint::from(1, -1), 4), MatrixPoint::from(3, 3));
    }
}
