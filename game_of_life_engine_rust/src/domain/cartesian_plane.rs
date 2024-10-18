use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[wasm_bindgen]
pub struct ArrPos {
    pub row: i64,
    pub col: i64,
}

#[wasm_bindgen]
impl ArrPos {
    #[wasm_bindgen(constructor)]
    pub fn new(row: i64, col: i64) -> ArrPos {
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

pub fn serialize_point(point: Point) -> String {
    format!("({}, {})", point.x, point.y).to_string()
}

pub fn deserialize_point(str: &String) -> Option<Point> {
    let len = str.len();
    if str.find("(")? == 0 && str.find(")")? == len - 1 {
        let comma = str.find(", ")?;
        let x: i64 = str.get(1..comma)?.parse().ok()?;
        let y: i64 = str.get(comma + 2..len - 1)?.parse().ok()?;
        return Some(Point { x, y });
    }
    None
}

pub fn absolute_to_relative(value: i64, unit_size: i64) -> i64 {
    value / unit_size
}

pub fn index_to_point(position: ArrPos, length: i64) -> Point {
    let half = length / 2;
    Point::from(-half + position.col, half - position.row)
}

pub fn point_to_index(point: Point, length: i64) -> ArrPos {
    let half = length / 2;
    ArrPos {
        col: half + point.x,
        row: half - point.y,
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
    fn test_serialize_point() {
        assert_eq!(serialize_point(Point { x: -3, y: 1954 }), "(-3, 1954)");
        assert_eq!(serialize_point(Point { x: 42, y: -23823 }), "(42, -23823)");
    }

    #[test]
    fn test_deserialize_point() {
        assert_eq!(
            deserialize_point(&"(-3, 1954)".to_string()),
            Some(Point { x: -3, y: 1954 })
        );
        assert_eq!(
            deserialize_point(&"(42, -23823)".to_string()),
            Some(Point { x: 42, y: -23823 })
        );
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
