use crate::geometry::coordinate::CartesianPoint;
use std::fmt;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct RectI64 {
    pub x1: i64,
    pub y1: i64,
    pub x2: i64,
    pub y2: i64,
}

#[derive(Debug, PartialEq)]
pub struct RectF64 {
    pub x1: f64,
    pub y1: f64,
    pub x2: f64,
    pub y2: f64,
}

impl RectI64 {
    pub fn of(x1: i64, y1: i64, x2: i64, y2: i64) -> Self {
        RectI64 { x1, y1, x2, y2 }
    }
}

impl fmt::Display for RectI64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(({}, {}), ({}, {}))", self.x1, self.y1, self.x2, self.y2)
    }
}

fn delta_x(r: &RectI64) -> u64 {
    (r.x2 - r.x1).unsigned_abs()
}

fn delta_y(r: &RectI64) -> u64 {
    (r.y2 - r.y1).unsigned_abs()
}

pub fn get_length(r: &RectI64) -> u64 {
    let d_x = delta_x(r);
    let d_y = delta_y(r);
    if d_x > d_y {
        d_x + 1
    } else {
        d_y + 1
    }
}

pub fn center(r: &mut RectI64, p: CartesianPoint) {
    let len_x = r.x2 - r.x1;
    let len_y = r.y2 - r.y1;
    let end_x = len_x / 2;
    let end_y = len_y / 2;
    let start_x = p.x - end_x;
    let start_y = p.y - end_y;
    r.x1 = start_x;
    r.y1 = start_y;
    r.x2 = p.x + end_x;
    r.y2 = p.y + end_y;
}

pub fn zoom_in(r: &mut RectI64) {
    r.x1 += 1;
    r.y1 += 1;
    r.x2 -= 1;
    r.y2 -= 1;
}

pub fn zoom_out(r: &mut RectI64) {
    r.x1 -= 1;
    r.y1 -= 1;
    r.x2 += 1;
    r.y2 += 1;
}

pub fn zoom_to(r: &mut RectI64, size: u16) {
    if size < 3 {
        return;
    }
    let size = i64::from(size);
    let d_x = delta_x(r) as i64;
    let d_y = delta_y(r) as i64;
    let len_x = d_x + 1;
    let len_y = d_y + 1;
    let diff_x = len_x - size;
    let diff_y = len_y - size;
    r.x1 += diff_x / 2;
    r.y1 += diff_y / 2;
    r.x2 -= diff_x / 2;
    r.y2 -= diff_y / 2;
}

pub fn move_by(r: &mut RectI64, delta: CartesianPoint) {
    r.x1 += delta.x;
    r.y1 += delta.y;
    r.x2 += delta.x;
    r.y2 += delta.y;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rect() {
        let r = RectI64::of(-23, 38, 198, 7);
        assert_eq!(r, RectI64 { x1: -23, y1: 38, x2: 198, y2: 7 });
        assert_eq!(format!("{r}"), "((-23, 38), (198, 7))");
    }

    #[test]
    fn test_delta_x() {
        assert_eq!(delta_x(&RectI64::of(0, 0, 0, 0)), 0);
        assert_eq!(delta_x(&RectI64::of(0, 0, 1, 1)), 1);
        assert_eq!(delta_x(&RectI64::of(-1, -1, 0, 0)), 1);
        assert_eq!(delta_x(&RectI64::of(-1, -1, 1, 1)), 2);
        assert_eq!(delta_x(&RectI64::of(-2, -2, 2, 2)), 4);
        assert_eq!(delta_x(&RectI64::of(-3, -3, 3, 3)), 6);
        assert_eq!(delta_x(&RectI64::of(-4, -4, 4, 4)), 8);
        assert_eq!(delta_x(&RectI64::of(-5, -5, 5, 5)), 10);
    }

    #[test]
    fn test_delta_y() {
        assert_eq!(delta_y(&RectI64::of(0, 0, 0, 0)), 0);
        assert_eq!(delta_y(&RectI64::of(0, 0, 1, 1)), 1);
        assert_eq!(delta_y(&RectI64::of(-1, -1, 0, 0)), 1);
        assert_eq!(delta_y(&RectI64::of(-1, -1, 1, 1)), 2);
        assert_eq!(delta_y(&RectI64::of(-2, -2, 2, 2)), 4);
        assert_eq!(delta_y(&RectI64::of(-3, -3, 3, 3)), 6);
        assert_eq!(delta_y(&RectI64::of(-4, -4, 4, 4)), 8);
        assert_eq!(delta_y(&RectI64::of(-5, -5, 5, 5)), 10);
    }

    #[test]
    fn test_get_length() {
        assert_eq!(get_length(&RectI64::of(-10, -10, 10, 10)), 21);
        assert_eq!(get_length(&RectI64::of(-10, -10, 9, 9)), 20);
        assert_eq!(get_length(&RectI64::of(0, 0, 10, 10)), 11);
        assert_eq!(get_length(&RectI64::of(0, 0, 9, 9)), 10);
        assert_eq!(get_length(&RectI64::of(1, 1, 10, 10)), 10);
        assert_eq!(get_length(&RectI64::of(4, 4, 5, 5)), 2);
        assert_eq!(get_length(&RectI64::of(5, 5, 5, 5)), 1);
    }

    #[test]
    fn get_length_rectangle() {
        assert_eq!(get_length(&RectI64::of(-10, -5, 10, 5)), 21);
        assert_eq!(get_length(&RectI64::of(-5, -10, 5, 10)), 21);
    }

    #[test]
    fn zoom_in_odd_size() {
        let mut r = RectI64::of(-5, -5, 5, 5);
        zoom_in(&mut r);
        assert_eq!(r, RectI64::of(-4, -4, 4, 4));
        zoom_in(&mut r);
        assert_eq!(r, RectI64::of(-3, -3, 3, 3));
        zoom_in(&mut r);
        assert_eq!(r, RectI64::of(-2, -2, 2, 2));
        zoom_in(&mut r);
        assert_eq!(r, RectI64::of(-1, -1, 1, 1));
        zoom_in(&mut r);
        assert_eq!(r, RectI64::of(0, 0, 0, 0));
    }

    #[test]
    fn zoom_in_even_size_1st_scenario() {
        let mut r = RectI64::of(-5, -5, 4, 4);
        zoom_in(&mut r);
        assert_eq!(r, RectI64::of(-4, -4, 3, 3));
        zoom_in(&mut r);
        assert_eq!(r, RectI64::of(-3, -3, 2, 2));
        zoom_in(&mut r);
        assert_eq!(r, RectI64::of(-2, -2, 1, 1));
        zoom_in(&mut r);
        assert_eq!(r, RectI64::of(-1, -1, 0, 0));
        zoom_in(&mut r);
        assert_eq!(r, RectI64::of(0, 0, -1, -1));
    }

    #[test]
    fn zoom_in_even_size_2nd_scenario() {
        let mut r = RectI64::of(-4, -4, 5, 5);
        zoom_in(&mut r);
        assert_eq!(r, RectI64::of(-3, -3, 4, 4));
        zoom_in(&mut r);
        assert_eq!(r, RectI64::of(-2, -2, 3, 3));
        zoom_in(&mut r);
        assert_eq!(r, RectI64::of(-1, -1, 2, 2));
        zoom_in(&mut r);
        assert_eq!(r, RectI64::of(0, 0, 1, 1));
        zoom_in(&mut r);
        assert_eq!(r, RectI64::of(1, 1, 0, 0));
    }

    #[test]
    fn zoom_out_odd_size() {
        let mut r = RectI64::of(-1, -1, 1, 1);
        zoom_out(&mut r);
        assert_eq!(r, RectI64::of(-2, -2, 2, 2));
        zoom_out(&mut r);
        assert_eq!(r, RectI64::of(-3, -3, 3, 3));
        zoom_out(&mut r);
        assert_eq!(r, RectI64::of(-4, -4, 4, 4));
        zoom_out(&mut r);
        assert_eq!(r, RectI64::of(-5, -5, 5, 5));
        zoom_out(&mut r);
        assert_eq!(r, RectI64::of(-6, -6, 6, 6));
    }

    #[test]
    fn zoom_out_even_size() {
        let mut r = RectI64::of(0, 0, 1, 1);
        zoom_out(&mut r);
        assert_eq!(r, RectI64::of(-1, -1, 2, 2));
        zoom_out(&mut r);
        assert_eq!(r, RectI64::of(-2, -2, 3, 3));
        zoom_out(&mut r);
        assert_eq!(r, RectI64::of(-3, -3, 4, 4));
        zoom_out(&mut r);
        assert_eq!(r, RectI64::of(-4, -4, 5, 5));
        zoom_out(&mut r);
        assert_eq!(r, RectI64::of(-5, -5, 6, 6));
    }

    #[test]
    fn zoom_to_odd_size() {
        let mut r = RectI64::of(-5, -5, 5, 5);
        zoom_to(&mut r, 11);
        assert_eq!(r, RectI64::of(-5, -5, 5, 5));
        zoom_to(&mut r, 9);
        assert_eq!(r, RectI64::of(-4, -4, 4, 4));
        zoom_to(&mut r, 7);
        assert_eq!(r, RectI64::of(-3, -3, 3, 3));
        zoom_to(&mut r, 5);
        assert_eq!(r, RectI64::of(-2, -2, 2, 2));
        zoom_to(&mut r, 3);
        assert_eq!(r, RectI64::of(-1, -1, 1, 1));
        zoom_to(&mut r, 1);
        assert_eq!(r, RectI64::of(-1, -1, 1, 1));
        zoom_to(&mut r, 1);
        assert_eq!(r, RectI64::of(-1, -1, 1, 1));
        zoom_to(&mut r, 3);
        assert_eq!(r, RectI64::of(-1, -1, 1, 1));
        zoom_to(&mut r, 5);
        assert_eq!(r, RectI64::of(-2, -2, 2, 2));
        zoom_to(&mut r, 7);
        assert_eq!(r, RectI64::of(-3, -3, 3, 3));
    }

    #[test]
    fn zoom_to_even_size_1st_scenario() {
        let mut r = RectI64::of(-5, -5, 4, 4);
        zoom_to(&mut r, 10);
        assert_eq!(r, RectI64::of(-5, -5, 4, 4));
        zoom_to(&mut r, 8);
        assert_eq!(r, RectI64::of(-4, -4, 3, 3));
        zoom_to(&mut r, 6);
        assert_eq!(r, RectI64::of(-3, -3, 2, 2));
        zoom_to(&mut r, 4);
        assert_eq!(r, RectI64::of(-2, -2, 1, 1));
        zoom_to(&mut r, 2);
        assert_eq!(r, RectI64::of(-2, -2, 1, 1));
        zoom_to(&mut r, 2);
        assert_eq!(r, RectI64::of(-2, -2, 1, 1));
        zoom_to(&mut r, 4);
        assert_eq!(r, RectI64::of(-2, -2, 1, 1));
        zoom_to(&mut r, 6);
        assert_eq!(r, RectI64::of(-3, -3, 2, 2));
    }

    #[test]
    fn test_zoom_to_even_size_2nd_scenario() {
        let mut r = RectI64::of(-4, -4, 5, 5);
        zoom_to(&mut r, 10);
        assert_eq!(r, RectI64::of(-4, -4, 5, 5));
        zoom_to(&mut r, 8);
        assert_eq!(r, RectI64::of(-3, -3, 4, 4));
        zoom_to(&mut r, 6);
        assert_eq!(r, RectI64::of(-2, -2, 3, 3));
        zoom_to(&mut r, 4);
        assert_eq!(r, RectI64::of(-1, -1, 2, 2));
        zoom_to(&mut r, 2);
        assert_eq!(r, RectI64::of(-1, -1, 2, 2));
        zoom_to(&mut r, 2);
        assert_eq!(r, RectI64::of(-1, -1, 2, 2));
        zoom_to(&mut r, 4);
        assert_eq!(r, RectI64::of(-1, -1, 2, 2));
        zoom_to(&mut r, 6);
        assert_eq!(r, RectI64::of(-2, -2, 3, 3));
    }

    #[test]
    fn test_move_by() {
        let mut r = RectI64::of(-10, -10, 10, 10);
        move_by(&mut r, CartesianPoint::of(10, 10));
        assert_eq!(r, RectI64::of(0, 0, 20, 20));
        move_by(&mut r, CartesianPoint::of(-5, -5));
        assert_eq!(r, RectI64::of(-5, -5, 15, 15));
        move_by(&mut r, CartesianPoint::of(-15, 5));
        assert_eq!(r, RectI64::of(-20, 0, 0, 20));
    }

    #[test]
    fn test_center() {
        let mut r = RectI64::of(-10, -10, 10, 10);
        center(&mut r, CartesianPoint::of(0, 0));
        assert_eq!(r, RectI64::of(-10, -10, 10, 10));
        center(&mut r, CartesianPoint::of(10, 10));
        assert_eq!(r, RectI64::of(0, 0, 20, 20));
    }
}
