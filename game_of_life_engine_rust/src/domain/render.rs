use super::{
    cell::State,
    geometry::{
        coordinate::cartesian_to_matrix,
        poligon::rect::{get_length, Rect},
    },
    universe::Universe,
};

#[derive(Debug, PartialEq, Clone)]
pub struct RenderSettings {
    pub cam: Rect,
    pub dim: u16,
    pub gap: u8,
}

pub fn get_values_to_render(u: &Universe, s: &RenderSettings) -> Vec<Rect> {
    let subdivision_size = s.dim as u64 / get_length(&s.cam);
    let mut values_to_render: Vec<Rect> = u
        .value
        .iter()
        .filter(|value| {
            value.0.x >= s.cam.x1
                && value.0.x <= s.cam.x2
                && value.0.y >= s.cam.y1
                && value.0.y <= s.cam.y2
        })
        .filter(|value| value.1 == &State::Alive)
        .map(|value| {
            let arr_index = cartesian_to_matrix(value.0, &s.cam);
            let gap = s.gap;
            let x = arr_index.col as i64 * subdivision_size as i64 + gap as i64;
            let y = arr_index.row as i64 * subdivision_size as i64 + gap as i64;

            Rect {
                x1: x,
                y1: y,
                x2: x + subdivision_size as i64 - gap as i64 * 2,
                y2: y + subdivision_size as i64 - gap as i64 * 2,
            }
        })
        .collect();
    values_to_render.sort_by(|a, b| a.y1.cmp(&b.y1));
    values_to_render.sort_by(|a, b| a.x1.cmp(&b.x1));

    values_to_render
}

#[cfg(test)]
mod test {
    use crate::domain::universe::from_string;

    use super::*;

    fn get_universe() -> Universe {
        from_string(vec![
           String::from("⬜⬛⬛⬛⬛⬛⬛⬛⬛⬜"),
           String::from("⬛⬜⬛⬛⬛⬛⬛⬛⬜⬛"),
           String::from("⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛"),
           String::from("⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛"),
           String::from("⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛"),
           String::from("⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛"),
           String::from("⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛"),
           String::from("⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛"),
           String::from("⬛⬜⬛⬛⬛⬛⬛⬛⬜⬛"),
           String::from("⬜⬛⬛⬛⬛⬛⬛⬛⬛⬜"),
       ])
       .unwrap()
    }

    #[test]
    fn test_render() {
        let u = get_universe();
        let s = RenderSettings { cam: Rect::from(-5, -5, 4, 4), dim: 1000, gap: 0 };
        assert_eq!(
            get_values_to_render(&u, &s),
            vec![
                Rect { x1: 0, y1: 0, x2: 100, y2: 100 },
                Rect { x1: 0, y1: 900, x2: 100, y2: 1000 },
                Rect { x1: 100, y1: 100, x2: 200, y2: 200 },
                Rect { x1: 100, y1: 800, x2: 200, y2: 900 },
                Rect { x1: 800, y1: 100, x2: 900, y2: 200 },
                Rect { x1: 800, y1: 800, x2: 900, y2: 900 },
                Rect { x1: 900, y1: 0, x2: 1000, y2: 100 },
                Rect { x1: 900, y1: 900, x2: 1000, y2: 1000 },
            ]
        );
    }

    #[test]
    fn test_render_gap() {
        let u = get_universe();
        let cam = Rect::from(-5, -5, 4, 4);
        let s_gap1 = RenderSettings { cam, dim: 1000, gap: 1 };
        let s_gap2 = RenderSettings { cam, dim: 1000, gap: 2 };
        assert_eq!(
            get_values_to_render(&u, &s_gap1),
            vec![
                Rect { x1: 1, y1: 1, x2: 99, y2: 99 },
                Rect { x1: 1, y1: 901, x2: 99, y2: 999 },
                Rect { x1: 101, y1: 101, x2: 199, y2: 199 },
                Rect { x1: 101, y1: 801, x2: 199, y2: 899 },
                Rect { x1: 801, y1: 101, x2: 899, y2: 199 },
                Rect { x1: 801, y1: 801, x2: 899, y2: 899 },
                Rect { x1: 901, y1: 1, x2: 999, y2: 99 },
                Rect { x1: 901, y1: 901, x2: 999, y2: 999 },
            ]
        );
        assert_eq!(
            get_values_to_render(&u, &s_gap2),
            vec![
                Rect { x1: 2, y1: 2, x2: 98, y2: 98 },
                Rect { x1: 2, y1: 902, x2: 98, y2: 998 },
                Rect { x1: 102, y1: 102, x2: 198, y2: 198 },
                Rect { x1: 102, y1: 802, x2: 198, y2: 898 },
                Rect { x1: 802, y1: 102, x2: 898, y2: 198 },
                Rect { x1: 802, y1: 802, x2: 898, y2: 898 },
                Rect { x1: 902, y1: 2, x2: 998, y2: 98 },
                Rect { x1: 902, y1: 902, x2: 998, y2: 998 },
            ]
        );
    }

    #[test]
    fn test_render_cam() {
        let u = get_universe();
        let s_cam_minus1 = RenderSettings { cam: Rect::from(-6, -5, 3, 4), dim: 1000, gap: 0 };
        let s_cam_plus1 = RenderSettings { cam: Rect::from(-4, -5, 5, 4), dim: 1000, gap: 0 };
        assert_eq!(
            get_values_to_render(&u, &s_cam_minus1),
            vec![
                Rect { x1: 100, y1: 0, x2: 200, y2: 100 },
                Rect { x1: 100, y1: 900, x2: 200, y2: 1000 },
                Rect { x1: 200, y1: 100, x2: 300, y2: 200 },
                Rect { x1: 200, y1: 800, x2: 300, y2: 900 },
                Rect { x1: 900, y1: 100, x2: 1000, y2: 200 },
                Rect { x1: 900, y1: 800, x2: 1000, y2: 900 },
            ]
        );
        assert_eq!(
            get_values_to_render(&u, &s_cam_plus1),
            vec![
                Rect { x1: 0, y1: 100, x2: 100, y2: 200 },
                Rect { x1: 0, y1: 800, x2: 100, y2: 900 },
                Rect { x1: 700, y1: 100, x2: 800, y2: 200 },
                Rect { x1: 700, y1: 800, x2: 800, y2: 900 },
                Rect { x1: 800, y1: 0, x2: 900, y2: 100 },
                Rect { x1: 800, y1: 900, x2: 900, y2: 1000 },
            ]
        );
    }
}
