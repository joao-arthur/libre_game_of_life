use crate::domain::plane::cartesian::CartesianPoint;
use std::fmt;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Rect {
    pub x1: i64,
    pub y1: i64,
    pub x2: i64,
    pub y2: i64,
}

impl Rect {
    pub fn from(x1: i64, y1: i64, x2: i64, y2: i64) -> Self {
        Rect { x1, y1, x2, y2 }
    }
}

impl fmt::Display for Rect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "(({}, {}), ({}, {}))",
            self.x1, self.y1, self.x2, self.y2
        )
    }
}

fn delta_x(r: &Rect) -> u64 {
    (r.x2 - r.x1).try_into().unwrap()
}

fn delta_y(r: &Rect) -> u64 {
    (r.y2 - r.y1).try_into().unwrap()
}

pub fn get_length(r: &Rect) -> u64 {
    let d_x = delta_x(r);
    let d_y = delta_y(r);
    if d_x > d_y {
        d_x + 1
    } else {
        d_y + 1
    }
}

pub fn get_subdivision_size(r: &Rect, size: u16) -> u64 {
    let size: u64 = size.into();
    size / get_length(r)
}

pub fn get_center(r: &Rect) -> CartesianPoint {
    CartesianPoint::from((r.x1 + r.x2) / 2, (r.y1 + r.y2) / 2)
}

pub fn get_center_absolute(r: &Rect, size: u16) -> CartesianPoint {
    let cell_size: i64 = get_subdivision_size(r, size).try_into().unwrap();
    let center = get_center(r);
    CartesianPoint::from(center.x * cell_size, center.y * cell_size)
}

pub fn center(r: &mut Rect, p: CartesianPoint) {
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

pub fn zoom_in(r: &mut Rect) {
    if delta_x(&r) > 2 && delta_y(r) > 2 {
        r.x1 += 1;
        r.y1 += 1;
        r.x2 -= 1;
        r.y2 -= 1;
    }
}

pub fn zoom_out(r: &mut Rect) {
    r.x1 -= 1;
    r.y1 -= 1;
    r.x2 += 1;
    r.y2 += 1;
}

pub fn zoom_to(r: &mut Rect, size: u16) {
    if size < 3 {
        return;
    }
    let size: i64 = size.into();
    let d_x: i64 = delta_x(r).try_into().unwrap();
    let d_y: i64 = delta_y(r).try_into().unwrap();
    let len_x: i64 = d_x + 1;
    let len_y: i64 = d_y + 1;
    let diff_x: i64 = len_x - size;
    let diff_y: i64 = len_y - size;
    r.x1 += diff_x / 2;
    r.y1 += diff_y / 2;
    r.x2 -= diff_x / 2;
    r.y2 -= diff_y / 2;
}

pub fn move_by(r: &mut Rect, delta: CartesianPoint) {
    r.x1 += delta.x;
    r.y1 += delta.y;
    r.x2 += delta.x;
    r.y2 += delta.y;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rect() {
        let r = Rect::from(-23, 38, 198, 7);
        assert_eq!(
            r,
            Rect {
                x1: -23,
                y1: 38,
                x2: 198,
                y2: 7
            }
        );
        assert_eq!(format!("{r}"), "((-23, 38), (198, 7))");
    }

    #[test]
    fn test_delta_x() {
        assert_eq!(delta_x(&Rect::from(0, 0, 0, 0)), 0);
        assert_eq!(delta_x(&Rect::from(0, 0, 1, 1)), 1);
        assert_eq!(delta_x(&Rect::from(-1, -1, 0, 0)), 1);
        assert_eq!(delta_x(&Rect::from(-1, -1, 1, 1)), 2);
        assert_eq!(delta_x(&Rect::from(-2, -2, 2, 2)), 4);
        assert_eq!(delta_x(&Rect::from(-3, -3, 3, 3)), 6);
        assert_eq!(delta_x(&Rect::from(-4, -4, 4, 4)), 8);
        assert_eq!(delta_x(&Rect::from(-5, -5, 5, 5)), 10);
    }

    #[test]
    fn test_delta_y() {
        assert_eq!(delta_x(&Rect::from(0, 0, 0, 0)), 0);
        assert_eq!(delta_x(&Rect::from(0, 0, 1, 1)), 1);
        assert_eq!(delta_x(&Rect::from(-1, -1, 0, 0)), 1);
        assert_eq!(delta_x(&Rect::from(-1, -1, 1, 1)), 2);
        assert_eq!(delta_x(&Rect::from(-2, -2, 2, 2)), 4);
        assert_eq!(delta_x(&Rect::from(-3, -3, 3, 3)), 6);
        assert_eq!(delta_x(&Rect::from(-4, -4, 4, 4)), 8);
        assert_eq!(delta_x(&Rect::from(-5, -5, 5, 5)), 10);
    }

    #[test]
    fn test_zoom_in_odd_size() {
        let mut r = Rect::from(-5, -5, 5, 5);
        zoom_in(&mut r);
        assert_eq!(r, Rect::from(-4, -4, 4, 4));
        zoom_in(&mut r);
        assert_eq!(r, Rect::from(-3, -3, 3, 3));
        zoom_in(&mut r);
        assert_eq!(r, Rect::from(-2, -2, 2, 2));
        zoom_in(&mut r);
        assert_eq!(r, Rect::from(-1, -1, 1, 1));
        zoom_in(&mut r);
        assert_eq!(r, Rect::from(-1, -1, 1, 1));
    }

    #[test]
    fn test_zoom_in_even_size_1() {
        let mut r = Rect::from(-5, -5, 4, 4);
        zoom_in(&mut r);
        assert_eq!(r, Rect::from(-4, -4, 3, 3));
        zoom_in(&mut r);
        assert_eq!(r, Rect::from(-3, -3, 2, 2));
        zoom_in(&mut r);
        assert_eq!(r, Rect::from(-2, -2, 1, 1));
        zoom_in(&mut r);
        assert_eq!(r, Rect::from(-1, -1, 0, 0));
        zoom_in(&mut r);
        assert_eq!(r, Rect::from(-1, -1, 0, 0));
    }

    #[test]
    fn test_zoom_in_even_size_2() {
        let mut r = Rect::from(-4, -4, 5, 5);
        zoom_in(&mut r);
        assert_eq!(r, Rect::from(-3, -3, 4, 4));
        zoom_in(&mut r);
        assert_eq!(r, Rect::from(-2, -2, 3, 3));
        zoom_in(&mut r);
        assert_eq!(r, Rect::from(-1, -1, 2, 2));
        zoom_in(&mut r);
        assert_eq!(r, Rect::from(0, 0, 1, 1));
        zoom_in(&mut r);
        assert_eq!(r, Rect::from(0, 0, 1, 1));
    }

    #[test]
    fn test_zoom_out_odd_size() {
        let mut r = Rect::from(-1, -1, 1, 1);
        zoom_out(&mut r);
        assert_eq!(r, Rect::from(-2, -2, 2, 2));
        zoom_out(&mut r);
        assert_eq!(r, Rect::from(-3, -3, 3, 3));
        zoom_out(&mut r);
        assert_eq!(r, Rect::from(-4, -4, 4, 4));
        zoom_out(&mut r);
        assert_eq!(r, Rect::from(-5, -5, 5, 5));
        zoom_out(&mut r);
        assert_eq!(r, Rect::from(-6, -6, 6, 6));
    }

    #[test]
    fn test_zoom_out_even_size() {
        let mut r = Rect::from(0, 0, 1, 1);
        zoom_out(&mut r);
        assert_eq!(r, Rect::from(-1, -1, 2, 2));
        zoom_out(&mut r);
        assert_eq!(r, Rect::from(-2, -2, 3, 3));
        zoom_out(&mut r);
        assert_eq!(r, Rect::from(-3, -3, 4, 4));
        zoom_out(&mut r);
        assert_eq!(r, Rect::from(-4, -4, 5, 5));
        zoom_out(&mut r);
        assert_eq!(r, Rect::from(-5, -5, 6, 6));
    }

    #[test]
    fn test_zoom_to_odd_size() {
        let mut r = Rect::from(-5, -5, 5, 5);
        zoom_to(&mut r, 11);
        assert_eq!(r, Rect::from(-5, -5, 5, 5));
        zoom_to(&mut r, 9);
        assert_eq!(r, Rect::from(-4, -4, 4, 4));
        zoom_to(&mut r, 7);
        assert_eq!(r, Rect::from(-3, -3, 3, 3));
        zoom_to(&mut r, 5);
        assert_eq!(r, Rect::from(-2, -2, 2, 2));
        zoom_to(&mut r, 3);
        assert_eq!(r, Rect::from(-1, -1, 1, 1));
        zoom_to(&mut r, 1);
        assert_eq!(r, Rect::from(-1, -1, 1, 1));
        zoom_to(&mut r, 1);
        assert_eq!(r, Rect::from(-1, -1, 1, 1));
        zoom_to(&mut r, 3);
        assert_eq!(r, Rect::from(-1, -1, 1, 1));
        zoom_to(&mut r, 5);
        assert_eq!(r, Rect::from(-2, -2, 2, 2));
        zoom_to(&mut r, 7);
        assert_eq!(r, Rect::from(-3, -3, 3, 3));
    }

    #[test]
    fn test_zoom_to_even_size_1() {
        let mut r = Rect::from(-5, -5, 4, 4);
        zoom_to(&mut r, 10);
        assert_eq!(r, Rect::from(-5, -5, 4, 4));
        zoom_to(&mut r, 8);
        assert_eq!(r, Rect::from(-4, -4, 3, 3));
        zoom_to(&mut r, 6);
        assert_eq!(r, Rect::from(-3, -3, 2, 2));
        zoom_to(&mut r, 4);
        assert_eq!(r, Rect::from(-2, -2, 1, 1));
        zoom_to(&mut r, 2);
        assert_eq!(r, Rect::from(-2, -2, 1, 1));
        zoom_to(&mut r, 2);
        assert_eq!(r, Rect::from(-2, -2, 1, 1));
        zoom_to(&mut r, 4);
        assert_eq!(r, Rect::from(-2, -2, 1, 1));
        zoom_to(&mut r, 6);
        assert_eq!(r, Rect::from(-3, -3, 2, 2));
    }

    #[test]
    fn test_zoom_to_even_size_2() {
        let mut r = Rect::from(-4, -4, 5, 5);
        zoom_to(&mut r, 10);
        assert_eq!(r, Rect::from(-4, -4, 5, 5));
        zoom_to(&mut r, 8);
        assert_eq!(r, Rect::from(-3, -3, 4, 4));
        zoom_to(&mut r, 6);
        assert_eq!(r, Rect::from(-2, -2, 3, 3));
        zoom_to(&mut r, 4);
        assert_eq!(r, Rect::from(-1, -1, 2, 2));
        zoom_to(&mut r, 2);
        assert_eq!(r, Rect::from(-1, -1, 2, 2));
        zoom_to(&mut r, 2);
        assert_eq!(r, Rect::from(-1, -1, 2, 2));
        zoom_to(&mut r, 4);
        assert_eq!(r, Rect::from(-1, -1, 2, 2));
        zoom_to(&mut r, 6);
        assert_eq!(r, Rect::from(-2, -2, 3, 3));
    }

    #[test]
    fn test_move_by() {
        let mut r = Rect::from(-10, -10, 10, 10);
        move_by(&mut r, CartesianPoint::from(10, 10));
        assert_eq!(r, Rect::from(0, 0, 20, 20));
        move_by(&mut r, CartesianPoint::from(-5, -5));
        assert_eq!(r, Rect::from(-5, -5, 15, 15));
        move_by(&mut r, CartesianPoint::from(-15, 5));
        assert_eq!(r, Rect::from(-20, 0, 0, 20));
    }

    #[test]
    fn test_get_length() {
        assert_eq!(get_length(&Rect::from(-10, -10, 10, 10)), 21);
        assert_eq!(get_length(&Rect::from(-10, -10, 9, 9)), 20);
        assert_eq!(get_length(&Rect::from(0, 0, 10, 10)), 11);
        assert_eq!(get_length(&Rect::from(0, 0, 9, 9)), 10);
        assert_eq!(get_length(&Rect::from(1, 1, 10, 10)), 10);
        assert_eq!(get_length(&Rect::from(4, 4, 5, 5)), 2);
        assert_eq!(get_length(&Rect::from(5, 5, 5, 5)), 1);
    }

    #[test]
    fn test_get_subdivision_size() {
        let r = Rect::from(1, 1, 10, 10);
        assert_eq!(get_subdivision_size(&r, 100), 10);
        assert_eq!(get_subdivision_size(&r, 90), 9);
        assert_eq!(get_subdivision_size(&r, 50), 5);
        assert_eq!(get_subdivision_size(&r, 10), 1);
    }

    #[test]
    fn test_get_center() {
        assert_eq!(
            get_center(&Rect::from(-10, -10, 10, 10)),
            CartesianPoint::from(0, 0)
        );
        assert_eq!(
            get_center(&Rect::from(1, 1, 10, 10)),
            CartesianPoint::from(5, 5)
        );
        assert_eq!(
            get_center(&Rect::from(4, 4, 5, 5)),
            CartesianPoint::from(4, 4)
        );
        assert_eq!(
            get_center(&Rect::from(5, 5, 5, 5)),
            CartesianPoint::from(5, 5)
        );
    }

    #[test]
    fn test_get_center_absolute() {
        assert_eq!(
            get_center_absolute(&Rect::from(-10, -10, 10, 10), 100),
            CartesianPoint::from(0, 0)
        );
        assert_eq!(
            get_center_absolute(&Rect::from(-10, -10, 10, 10), 100),
            CartesianPoint::from(0, 0)
        );
        assert_eq!(
            get_center_absolute(&Rect::from(1, 1, 10, 10), 50),
            CartesianPoint::from(25, 25)
        );
        assert_eq!(
            get_center_absolute(&Rect::from(4, 4, 5, 5), 10),
            CartesianPoint::from(20, 20)
        );
        assert_eq!(
            get_center_absolute(&Rect::from(5, 5, 5, 5), 1),
            CartesianPoint::from(5, 5)
        );
        assert_eq!(
            get_center_absolute(&Rect::from(-5, -4, 4, 5), 1000),
            CartesianPoint::from(0, 0)
        );
        assert_eq!(
            get_center_absolute(&Rect::from(-4, -4, 5, 5), 1000),
            CartesianPoint::from(0, 0)
        );
        assert_eq!(
            get_center_absolute(&Rect::from(-5, -4, 3, 5), 1000),
            CartesianPoint::from(-100, 0)
        );
        assert_eq!(
            get_center_absolute(&Rect::from(-6, -4, 2, 5), 1000),
            CartesianPoint::from(-200, 0)
        );
    }

    #[test]
    fn test_center() {
        let mut r = Rect::from(-10, -10, 10, 10);
        center(&mut r, CartesianPoint::from(0, 0));
        assert_eq!(r, Rect::from(-10, -10, 10, 10));
        center(&mut r, CartesianPoint::from(10, 10));
        assert_eq!(r, Rect::from(0, 0, 20, 20));
    }
}
