use super::{
    cell::State,
    geometry::{
        coordinate::cartesian_to_matrix,
        poligon::rect::{RectF64, RectI64, get_length},
    },
    universe::Universe,
};

#[derive(Debug, PartialEq, Clone)]
pub struct RenderSettings {
    pub cam: RectI64,
    pub dim: u16,
    pub gap: u8,
}

pub fn get_values_to_render(universe: &Universe, settings: &RenderSettings) -> Vec<RectF64> {
    let dim = f64::from(settings.dim);
    let len = get_length(&settings.cam) as f64;
    let cell_size = dim / len;
    let mut values_to_render: Vec<RectF64> = universe
        .value
        .iter()
        .filter(|value| {
            value.0.x >= settings.cam.x1
                && value.0.x <= settings.cam.x2
                && value.0.y >= settings.cam.y1
                && value.0.y <= settings.cam.y2
        })
        .filter(|value| value.1 == &State::Alive)
        .map(|value| {
            let arr_index = cartesian_to_matrix(value.0, &settings.cam);
            let gap = f64::from(settings.gap);
            let col = arr_index.col as f64;
            let row = arr_index.row as f64;
            RectF64 {
                x1: col * cell_size + gap,
                y1: row * cell_size + gap,
                x2: col * cell_size + cell_size - gap,
                y2: row * cell_size + cell_size - gap,
            }
        })
        .collect();
    values_to_render.sort_by(|a, b| a.y1.partial_cmp(&b.y1).unwrap_or(std::cmp::Ordering::Greater));
    values_to_render.sort_by(|a, b| a.x1.partial_cmp(&b.x1).unwrap_or(std::cmp::Ordering::Greater));
    values_to_render
}

#[cfg(test)]
mod tests {
    use crate::{
        geometry::poligon::rect::{RectF64, RectI64},
        render::RenderSettings,
        universe::{Universe, universe_try_from_string},
    };

    use super::get_values_to_render;

    fn get_universe() -> Universe {
        universe_try_from_string(vec![
            "⬜⬛⬛⬛⬛⬛⬛⬛⬛⬜".into(),
            "⬛⬜⬛⬛⬛⬛⬛⬛⬜⬛".into(),
            "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".into(),
            "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".into(),
            "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".into(),
            "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".into(),
            "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".into(),
            "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛".into(),
            "⬛⬜⬛⬛⬛⬛⬛⬛⬜⬛".into(),
            "⬜⬛⬛⬛⬛⬛⬛⬛⬛⬜".into(),
        ])
        .unwrap()
    }

    #[test]
    fn render() {
        let universe = get_universe();
        let s = RenderSettings { cam: RectI64::of(-5, -5, 4, 4), dim: 1000, gap: 0 };
        assert_eq!(
            get_values_to_render(&universe, &s),
            [
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
    fn render_gap() {
        let universe = get_universe();
        let cam = RectI64::of(-5, -5, 4, 4);
        let s_gap1 = RenderSettings { cam, dim: 1000, gap: 1 };
        let s_gap2 = RenderSettings { cam, dim: 1000, gap: 2 };
        assert_eq!(
            get_values_to_render(&universe, &s_gap1),
            [
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
            get_values_to_render(&universe, &s_gap2),
            [
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
    fn render_cam() {
        let universe = get_universe();
        let s_cam_minus1 = RenderSettings { cam: RectI64::of(-6, -5, 3, 4), dim: 1000, gap: 0 };
        let s_cam_plus1 = RenderSettings { cam: RectI64::of(-4, -5, 5, 4), dim: 1000, gap: 0 };
        assert_eq!(
            get_values_to_render(&universe, &s_cam_minus1),
            [
                RectF64 { x1: 100.0, y1: 0.0, x2: 200.0, y2: 100.0 },
                RectF64 { x1: 100.0, y1: 900.0, x2: 200.0, y2: 1000.0 },
                RectF64 { x1: 200.0, y1: 100.0, x2: 300.0, y2: 200.0 },
                RectF64 { x1: 200.0, y1: 800.0, x2: 300.0, y2: 900.0 },
                RectF64 { x1: 900.0, y1: 100.0, x2: 1000.0, y2: 200.0 },
                RectF64 { x1: 900.0, y1: 800.0, x2: 1000.0, y2: 900.0 },
            ]
        );
        assert_eq!(
            get_values_to_render(&universe, &s_cam_plus1),
            [
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
    fn render_float_cell_size() {
        let universe = get_universe();
        let s = RenderSettings { cam: RectI64::of(-5, -5, 4, 4), dim: 996, gap: 0 };
        assert_eq!(
            get_values_to_render(&universe, &s),
            [
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
