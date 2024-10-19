use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[wasm_bindgen]
pub struct ArrPos {
    pub row: u64,
    pub col: u64,
}

#[wasm_bindgen]
impl ArrPos {
    #[wasm_bindgen(constructor)]
    pub fn new(row: u64, col: u64) -> ArrPos {
        ArrPos { row, col }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy, Serialize, Deserialize)]
#[wasm_bindgen]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

#[wasm_bindgen]
impl Point {
    #[wasm_bindgen(constructor)]
    pub fn new(x: i64, y: i64) -> Point {
        Point { x, y }
    }
}

impl Point {
    pub fn from(x: i64, y: i64) -> Self {
        Point { x, y }
    }
}

pub fn absolute_to_relative(value: i64, unit_size: u64) -> i64 {
    value / unit_size as i64
}

pub fn index_to_point(position: ArrPos, length: u64) -> Point {
    let half = length / 2;
    let x: i64 = -(half as i64) + position.col as i64;
    let y: i64 = half as i64 - position.row as i64;
    Point::from(x, y)
}

pub fn point_to_index(point: Point, length: u64) -> ArrPos {
    let half = length / 2;
    ArrPos {
        col: half + point.x as u64,
        row: half - point.y as u64,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_point() {
        assert_eq!(Point::from(-23, 38), Point { x: -23, y: 38 });
    }

    #[test]
    fn test_absolute_to_relative() {
        assert_eq!(absolute_to_relative(0, 30), 0);
        assert_eq!(absolute_to_relative(10, 30), 0);
        assert_eq!(absolute_to_relative(20, 30), 0);
        assert_eq!(absolute_to_relative(29, 30), 0);
        assert_eq!(absolute_to_relative(30, 30), 1);
        assert_eq!(absolute_to_relative(300, 30), 10);
    }

    #[test]
    fn test_index_to_point_1x1_grid() {
        assert_eq!(
            index_to_point(ArrPos { row: 0, col: 0 }, 1),
            Point { x: 0, y: 0 }
        );
    }

    #[test]
    fn test_index_to_point_2x2_grid() {
        assert_eq!(
            index_to_point(ArrPos { row: 0, col: 0 }, 2),
            Point { x: -1, y: 1 }
        );
        assert_eq!(
            index_to_point(ArrPos { row: 0, col: 1 }, 2),
            Point { x: 0, y: 1 }
        );
        assert_eq!(
            index_to_point(ArrPos { row: 1, col: 0 }, 2),
            Point { x: -1, y: 0 }
        );
        assert_eq!(
            index_to_point(ArrPos { row: 1, col: 1 }, 2),
            Point { x: 0, y: 0 }
        );
    }

    #[test]
    fn test_index_to_point_3x3_grid() {
        assert_eq!(
            index_to_point(ArrPos { row: 0, col: 0 }, 3),
            Point { x: -1, y: 1 }
        );
        assert_eq!(
            index_to_point(ArrPos { row: 0, col: 1 }, 3),
            Point { x: 0, y: 1 }
        );
        assert_eq!(
            index_to_point(ArrPos { row: 0, col: 2 }, 3),
            Point { x: 1, y: 1 }
        );
        assert_eq!(
            index_to_point(ArrPos { row: 1, col: 0 }, 3),
            Point { x: -1, y: 0 }
        );
        assert_eq!(
            index_to_point(ArrPos { row: 1, col: 1 }, 3),
            Point { x: 0, y: 0 }
        );
        assert_eq!(
            index_to_point(ArrPos { row: 1, col: 2 }, 3),
            Point { x: 1, y: 0 }
        );
        assert_eq!(
            index_to_point(ArrPos { row: 2, col: 0 }, 3),
            Point { x: -1, y: -1 }
        );
        assert_eq!(
            index_to_point(ArrPos { row: 2, col: 1 }, 3),
            Point { x: 0, y: -1 }
        );
        assert_eq!(
            index_to_point(ArrPos { row: 2, col: 2 }, 3),
            Point { x: 1, y: -1 }
        );
    }

    #[test]
    fn test_index_to_point_4x4_grid() {
        assert_eq!(
            index_to_point(ArrPos { row: 0, col: 0 }, 4),
            Point { x: -2, y: 2 }
        );
        assert_eq!(
            index_to_point(ArrPos { row: 0, col: 1 }, 4),
            Point { x: -1, y: 2 }
        );
        assert_eq!(
            index_to_point(ArrPos { row: 0, col: 2 }, 4),
            Point { x: 0, y: 2 }
        );
        assert_eq!(
            index_to_point(ArrPos { row: 0, col: 3 }, 4),
            Point { x: 1, y: 2 }
        );
        assert_eq!(
            index_to_point(ArrPos { row: 1, col: 0 }, 4),
            Point { x: -2, y: 1 }
        );
        assert_eq!(
            index_to_point(ArrPos { row: 1, col: 1 }, 4),
            Point { x: -1, y: 1 }
        );
        assert_eq!(
            index_to_point(ArrPos { row: 1, col: 2 }, 4),
            Point { x: 0, y: 1 }
        );
        assert_eq!(
            index_to_point(ArrPos { row: 1, col: 3 }, 4),
            Point { x: 1, y: 1 }
        );
        assert_eq!(
            index_to_point(ArrPos { row: 2, col: 0 }, 4),
            Point { x: -2, y: 0 }
        );
        assert_eq!(
            index_to_point(ArrPos { row: 2, col: 1 }, 4),
            Point { x: -1, y: 0 }
        );
        assert_eq!(
            index_to_point(ArrPos { row: 2, col: 2 }, 4),
            Point { x: 0, y: 0 }
        );
        assert_eq!(
            index_to_point(ArrPos { row: 2, col: 3 }, 4),
            Point { x: 1, y: 0 }
        );
        assert_eq!(
            index_to_point(ArrPos { row: 3, col: 0 }, 4),
            Point { x: -2, y: -1 }
        );
        assert_eq!(
            index_to_point(ArrPos { row: 3, col: 1 }, 4),
            Point { x: -1, y: -1 }
        );
        assert_eq!(
            index_to_point(ArrPos { row: 3, col: 2 }, 4),
            Point { x: 0, y: -1 }
        );
        assert_eq!(
            index_to_point(ArrPos { row: 3, col: 3 }, 4),
            Point { x: 1, y: -1 }
        );
    }

    #[test]
    fn test_point_to_index_1x1_grid() {
        assert_eq!(
            point_to_index(Point { x: 0, y: 0 }, 1),
            ArrPos { row: 0, col: 0 }
        );
    }

    #[test]
    fn test_point_to_index_2x2_grid() {
        assert_eq!(
            point_to_index(Point { x: -1, y: 1 }, 2),
            ArrPos { row: 0, col: 0 }
        );
        assert_eq!(
            point_to_index(Point { x: 0, y: 1 }, 2),
            ArrPos { row: 0, col: 1 }
        );
        assert_eq!(
            point_to_index(Point { x: -1, y: 0 }, 2),
            ArrPos { row: 1, col: 0 }
        );
        assert_eq!(
            point_to_index(Point { x: 0, y: 0 }, 2),
            ArrPos { row: 1, col: 1 }
        );
    }

    #[test]
    fn test_point_to_index_3x3_grid() {
        assert_eq!(
            point_to_index(Point { x: -1, y: 1 }, 3),
            ArrPos { row: 0, col: 0 }
        );
        assert_eq!(
            point_to_index(Point { x: 0, y: 1 }, 3),
            ArrPos { row: 0, col: 1 }
        );
        assert_eq!(
            point_to_index(Point { x: 1, y: 1 }, 3),
            ArrPos { row: 0, col: 2 }
        );
        assert_eq!(
            point_to_index(Point { x: -1, y: 0 }, 3),
            ArrPos { row: 1, col: 0 }
        );
        assert_eq!(
            point_to_index(Point { x: 0, y: 0 }, 3),
            ArrPos { row: 1, col: 1 }
        );
        assert_eq!(
            point_to_index(Point { x: 1, y: 0 }, 3),
            ArrPos { row: 1, col: 2 }
        );
        assert_eq!(
            point_to_index(Point { x: -1, y: -1 }, 3),
            ArrPos { row: 2, col: 0 }
        );
        assert_eq!(
            point_to_index(Point { x: 0, y: -1 }, 3),
            ArrPos { row: 2, col: 1 }
        );
        assert_eq!(
            point_to_index(Point { x: 1, y: -1 }, 3),
            ArrPos { row: 2, col: 2 }
        );
    }

    #[test]
    fn test_point_to_index_4x4_grid() {
        assert_eq!(
            point_to_index(Point { x: -2, y: 2 }, 4),
            ArrPos { row: 0, col: 0 }
        );
        assert_eq!(
            point_to_index(Point { x: -1, y: 2 }, 4),
            ArrPos { row: 0, col: 1 }
        );
        assert_eq!(
            point_to_index(Point { x: 0, y: 2 }, 4),
            ArrPos { row: 0, col: 2 }
        );
        assert_eq!(
            point_to_index(Point { x: 1, y: 2 }, 4),
            ArrPos { row: 0, col: 3 }
        );
        assert_eq!(
            point_to_index(Point { x: -2, y: 1 }, 4),
            ArrPos { row: 1, col: 0 }
        );
        assert_eq!(
            point_to_index(Point { x: -1, y: 1 }, 4),
            ArrPos { row: 1, col: 1 }
        );
        assert_eq!(
            point_to_index(Point { x: 0, y: 1 }, 4),
            ArrPos { row: 1, col: 2 }
        );
        assert_eq!(
            point_to_index(Point { x: 1, y: 1 }, 4),
            ArrPos { row: 1, col: 3 }
        );
        assert_eq!(
            point_to_index(Point { x: -2, y: 0 }, 4),
            ArrPos { row: 2, col: 0 }
        );
        assert_eq!(
            point_to_index(Point { x: -1, y: 0 }, 4),
            ArrPos { row: 2, col: 1 }
        );
        assert_eq!(
            point_to_index(Point { x: 0, y: 0 }, 4),
            ArrPos { row: 2, col: 2 }
        );
        assert_eq!(
            point_to_index(Point { x: 1, y: 0 }, 4),
            ArrPos { row: 2, col: 3 }
        );
        assert_eq!(
            point_to_index(Point { x: -2, y: -1 }, 4),
            ArrPos { row: 3, col: 0 }
        );
        assert_eq!(
            point_to_index(Point { x: -1, y: -1 }, 4),
            ArrPos { row: 3, col: 1 }
        );
        assert_eq!(
            point_to_index(Point { x: 0, y: -1 }, 4),
            ArrPos { row: 3, col: 2 }
        );
        assert_eq!(
            point_to_index(Point { x: 1, y: -1 }, 4),
            ArrPos { row: 3, col: 3 }
        );
    }
}
