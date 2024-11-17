use super::{
    coordinate::CartesianP,
    poligon::rect::{self, Rect},
};

pub fn get_subdivision_size(r: &Rect, size: u16) -> u64 {
    let size: u64 = size.into();
    size / rect::get_length(r)
}

pub fn subdivide(value: i64, unit_size: u64) -> i64 {
    let u: i64 = unit_size.try_into().unwrap();
    value / u
}

pub fn get_center(r: &Rect) -> CartesianP {
    CartesianP::from((r.x1 + r.x2) / 2, (r.y1 + r.y2) / 2)
}

pub fn get_center_absolute(r: &Rect, size: u16) -> CartesianP {
    let cell_size: i64 = get_subdivision_size(r, size).try_into().unwrap();
    let center = get_center(r);
    CartesianP::from(center.x * cell_size, center.y * cell_size)
}

#[cfg(test)]
mod test {
    use super::*;

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
            CartesianP::from(0, 0)
        );
        assert_eq!(
            get_center(&Rect::from(1, 1, 10, 10)),
            CartesianP::from(5, 5)
        );
        assert_eq!(get_center(&Rect::from(4, 4, 5, 5)), CartesianP::from(4, 4));
        assert_eq!(get_center(&Rect::from(5, 5, 5, 5)), CartesianP::from(5, 5));
    }

    #[test]
    fn test_get_center_absolute() {
        assert_eq!(
            get_center_absolute(&Rect::from(-10, -10, 10, 10), 100),
            CartesianP::from(0, 0)
        );
        assert_eq!(
            get_center_absolute(&Rect::from(-10, -10, 10, 10), 100),
            CartesianP::from(0, 0)
        );
        assert_eq!(
            get_center_absolute(&Rect::from(1, 1, 10, 10), 50),
            CartesianP::from(25, 25)
        );
        assert_eq!(
            get_center_absolute(&Rect::from(4, 4, 5, 5), 10),
            CartesianP::from(20, 20)
        );
        assert_eq!(
            get_center_absolute(&Rect::from(5, 5, 5, 5), 1),
            CartesianP::from(5, 5)
        );
        assert_eq!(
            get_center_absolute(&Rect::from(-5, -4, 4, 5), 1000),
            CartesianP::from(0, 0)
        );
        assert_eq!(
            get_center_absolute(&Rect::from(-4, -4, 5, 5), 1000),
            CartesianP::from(0, 0)
        );
        assert_eq!(
            get_center_absolute(&Rect::from(-5, -4, 3, 5), 1000),
            CartesianP::from(-100, 0)
        );
        assert_eq!(
            get_center_absolute(&Rect::from(-6, -4, 2, 5), 1000),
            CartesianP::from(-200, 0)
        );
    }
}
