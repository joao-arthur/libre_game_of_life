use crate::domain::plane::cartesian::CartesianPoint;

/** Constraint: x2 > x1 && y2 > y1 */
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
        return d_x + 1;
    } else {
        return d_y + 1;
    }
}

/////////////////////
pub fn get_middle_cell(r: &Rect, total_size: u16) -> CartesianPoint {
    let cell_size: i64 = get_division_size(r, total_size).try_into().unwrap();
    let middle = get_middle_point(r);
    CartesianPoint::from(middle.x * cell_size, middle.y * cell_size)
}

pub fn get_division_size(r: &Rect, size: u16) -> u64 {
    let size: u64 = size.into();
    size / delta_x(r)
}

pub fn get_middle_point(r: &Rect) -> CartesianPoint {
    CartesianPoint::from((r.x1 + r.x2) / 2, (r.y1 + r.y2) / 2)
}
/////////////////////

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
        assert_eq!(
            Rect::from(-23, 38, 198, 7),
            Rect {
                x1: -23,
                y1: 38,
                x2: 198,
                y2: 7
            }
        );
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
    /*
        #[test]
        fn test_get_cell_size() {
            let u = Rect::from(1, 1, 10, 10));
            assert_eq!(get_cell_size(&u, 100), 10);
            assert_eq!(get_cell_size(&u, 90), 9);
            assert_eq!(get_cell_size(&u, 50), 5);
            assert_eq!(get_cell_size(&u, 10), 1);
        }

        #[test]
        fn test_get_middle_point() {
            assert_eq!(
                get_middle_point(&Rect::from(-10, -10, 10, 10)),
                CartesianPoint::from(0, 0)
            );
            assert_eq!(
                get_middle_point(&Rect::from(1, 1, 10, 10)),
                CartesianPoint::from(5, 5)
            );
            assert_eq!(
                get_middle_point(&Rect::from(4, 4, 5, 5)),
                CartesianPoint::from(4, 4)
            );
            assert_eq!(
                get_middle_point(&Rect::from(5, 5, 5, 5)),
                CartesianPoint::from(5, 5)
            );
        }

        #[test]
        fn test_get_middle_cell() {
            assert_eq!(
                get_middle_cell(&Rect::from(-10, -10, 10, 10), 100),
                CartesianPoint::from(0, 0)
            );
            assert_eq!(
                get_middle_cell(&Rect::from(1, 1, 10, 10), 50),
                CartesianPoint::from(27, 27)
            );
            assert_eq!(
                get_middle_cell(&Rect::from(4, 4, 5, 5), 10),
                CartesianPoint::from(22, 22)
            );
            assert_eq!(
                get_middle_cell(&Rect::from(5, 5, 5, 5), 1),
                CartesianPoint::from(5, 5)
            );
        }

        #[test]
        fn test_move_in_plane() {
            let mut u = Rect::from(-10, -10, 10, 10);
            move_in_plane(&mut u, CartesianPoint::from(1, 0));
            assert_eq!(u.pos, Rect::from(-9, -10, 11, 10));
            move_in_plane(&mut u, CartesianPoint::from(-2, 0));
            assert_eq!(u.pos, Rect::from(-11, -10, 9, 10));
            move_in_plane(&mut u, CartesianPoint::from(0, 1));
            assert_eq!(u.pos, Rect::from(-11, -9, 9, 11));
            move_in_plane(&mut u, CartesianPoint::from(0, -2));
            assert_eq!(u.pos, Rect::from(-11, -11, 9, 9));
            move_in_plane(&mut u, CartesianPoint::from(11, 0));
            assert_eq!(u.pos, Rect::from(0, -11, 20, 9));
            move_in_plane(&mut u, CartesianPoint::from(0, 11));
            assert_eq!(u.pos, Rect::from(0, 0, 20, 20));
            move_in_plane(&mut u, CartesianPoint::from(-20, 0));
            assert_eq!(u.pos, Rect::from(-20, 0, 0, 20));
            move_in_plane(&mut u, CartesianPoint::from(0, -20));
            assert_eq!(u.pos, Rect::from(-20, -20, 0, 0));
        }

    */
}
