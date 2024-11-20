use super::{
    cell::State,
    geometry::{
        coordinate::cartesian_to_matrix,
        poligon::rect::{get_length, Rect, RectF64},
    },
    universe::Universe,
};

#[derive(Debug, PartialEq, Clone)]
pub struct RenderSettings {
    pub cam: Rect,
    pub dim: u16,
    pub gap: u8,
}

pub fn get_values_to_render(u: &Universe, s: &RenderSettings) -> Vec<RectF64> {
    let dimf: f64 = s.dim.into();
    let lenf: f64 = get_length(&s.cam) as f64;
    let subdivision_size = dimf / lenf;
    let mut values_to_render: Vec<RectF64> = u
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
            let gap_start_f: f64 = s.gap.into();
            let gap_end_f: f64 = gap_start_f * 2.0;

            let col_f: f64 = arr_index.col as f64;
            let row_f: f64 = arr_index.row as f64;

            let x = col_f * subdivision_size + gap_start_f;
            let y = row_f * subdivision_size + gap_start_f;

            RectF64 {
                x1: x,
                y1: y,
                x2: x + subdivision_size - gap_end_f,
                y2: y + subdivision_size - gap_end_f,
            }
        })
        .collect();
    values_to_render.sort_by(|a, b| a.y1.partial_cmp(&b.y1).unwrap());
    values_to_render.sort_by(|a, b| a.x1.partial_cmp(&b.x1).unwrap());

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
                RectF64 { x1: 0.0, y1: 0.0, x2: 100.0, y2: 100.0 },
                RectF64 { x1: 0.0, y1: 900.0, x2: 100.0, y2: 1000.0 },
                RectF64 { x1: 100.0, y1: 100.0, x2: 200.0, y2: 200.0 },
                RectF64 { x1: 100.0, y1: 800.0, x2: 200.0, y2: 900.0 },
                RectF64 { x1: 800.0, y1: 100.0, x2: 900.0, y2: 200.0 },
                RectF64 { x1: 800.0, y1: 800.0, x2: 900.0, y2: 900.0 },
                RectF64 { x1: 900.0, y1: 0.0, x2: 1000.0, y2: 100.0 },
                RectF64 { x1: 900.0, y1: 900.0, x2: 1000.0, y2: 1000.0 },
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
                RectF64 { x1: 1.0, y1: 1.0, x2: 99.0, y2: 99.0 },
                RectF64 { x1: 1.0, y1: 901.0, x2: 99.0, y2: 999.0 },
                RectF64 { x1: 101.0, y1: 101.0, x2: 199.0, y2: 199.0 },
                RectF64 { x1: 101.0, y1: 801.0, x2: 199.0, y2: 899.0 },
                RectF64 { x1: 801.0, y1: 101.0, x2: 899.0, y2: 199.0 },
                RectF64 { x1: 801.0, y1: 801.0, x2: 899.0, y2: 899.0 },
                RectF64 { x1: 901.0, y1: 1.0, x2: 999.0, y2: 99.0 },
                RectF64 { x1: 901.0, y1: 901.0, x2: 999.0, y2: 999.0 },
            ]
        );
        assert_eq!(
            get_values_to_render(&u, &s_gap2),
            vec![
                RectF64 { x1: 2.0, y1: 2.0, x2: 98.0, y2: 98.0 },
                RectF64 { x1: 2.0, y1: 902.0, x2: 98.0, y2: 998.0 },
                RectF64 { x1: 102.0, y1: 102.0, x2: 198.0, y2: 198.0 },
                RectF64 { x1: 102.0, y1: 802.0, x2: 198.0, y2: 898.0 },
                RectF64 { x1: 802.0, y1: 102.0, x2: 898.0, y2: 198.0 },
                RectF64 { x1: 802.0, y1: 802.0, x2: 898.0, y2: 898.0 },
                RectF64 { x1: 902.0, y1: 2.0, x2: 998.0, y2: 98.0 },
                RectF64 { x1: 902.0, y1: 902.0, x2: 998.0, y2: 998.0 },
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
                RectF64 { x1: 100.0, y1: 0.0, x2: 200.0, y2: 100.0 },
                RectF64 { x1: 100.0, y1: 900.0, x2: 200.0, y2: 1000.0 },
                RectF64 { x1: 200.0, y1: 100.0, x2: 300.0, y2: 200.0 },
                RectF64 { x1: 200.0, y1: 800.0, x2: 300.0, y2: 900.0 },
                RectF64 { x1: 900.0, y1: 100.0, x2: 1000.0, y2: 200.0 },
                RectF64 { x1: 900.0, y1: 800.0, x2: 1000.0, y2: 900.0 },
            ]
        );
        assert_eq!(
            get_values_to_render(&u, &s_cam_plus1),
            vec![
                RectF64 { x1: 0.0, y1: 100.0, x2: 100.0, y2: 200.0 },
                RectF64 { x1: 0.0, y1: 800.0, x2: 100.0, y2: 900.0 },
                RectF64 { x1: 700.0, y1: 100.0, x2: 800.0, y2: 200.0 },
                RectF64 { x1: 700.0, y1: 800.0, x2: 800.0, y2: 900.0 },
                RectF64 { x1: 800.0, y1: 0.0, x2: 900.0, y2: 100.0 },
                RectF64 { x1: 800.0, y1: 900.0, x2: 900.0, y2: 1000.0 },
            ]
        );
    }

    #[test]
    fn test_render_float_size_cell() {
        let u = get_universe();
        let s = RenderSettings { cam: Rect::from(-5, -5, 4, 4), dim: 996, gap: 0 };
        assert_eq!(
            get_values_to_render(&u, &s),
            vec![
                RectF64 { x1: 0.0, y1: 0.0, x2: 99.6, y2: 99.6 },
                RectF64 { x1: 0.0, y1: 896.4, x2: 99.6, y2: 996.0 },
                RectF64 { x1: 99.6, y1: 99.6, x2: 199.2, y2: 199.2 },
                RectF64 { x1: 99.6, y1: 796.8, x2: 199.2, y2: 896.4 },
                RectF64 { x1: 796.8, y1: 99.6, x2: 896.4, y2: 199.2 },
                RectF64 { x1: 796.8, y1: 796.8, x2: 896.4, y2: 896.4 },
                RectF64 { x1: 896.4, y1: 0.0, x2: 996.0, y2: 99.6 },
                RectF64 { x1: 896.4, y1: 896.4, x2: 996.0, y2: 996.0 }
            ]
        );
    }
}
