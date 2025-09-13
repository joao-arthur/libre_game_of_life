use crate::{cell::State, render_settings::RenderSettings, universe::Universe};
use manfredo::{
    cartesian::rect::{rect_f64, rect_i32},
    transform::cartesian_in_cam_to_matrix::point_i32::cartesian_in_cam_to_matrix,
};

pub type Renderable = manfredo::cartesian::rect::rect_f64::Rect;

pub fn get_values_to_render(universe: &Universe, settings: &RenderSettings) -> Vec<Renderable> {
    let dim = f64::from(settings.dim);
    let len = rect_i32::max_len(&settings.cam) as f64;
    let cell_size = dim / len;
    let mut values_to_render: Vec<rect_f64::Rect> = universe
        .value
        .iter()
        .filter(|(point, _)| rect_i32::contains_point(&settings.cam, point))
        .filter(|(_, state)| state == &&State::Alive)
        .map(|(point, _)| {
            let arr_index = cartesian_in_cam_to_matrix(point, &settings.cam);
            let gap = f64::from(settings.gap);
            let col = arr_index.col as f64;
            let row = arr_index.row as f64;
            rect_f64::Rect::of(
                col * cell_size + gap,
                row * cell_size + gap,
                col * cell_size + cell_size - gap,
                row * cell_size + cell_size - gap,
            )
        })
        .collect();
    values_to_render
        .sort_by(|a, b| a.min.y.partial_cmp(&b.min.y).unwrap_or(std::cmp::Ordering::Greater));
    values_to_render
        .sort_by(|a, b| a.min.x.partial_cmp(&b.min.x).unwrap_or(std::cmp::Ordering::Greater));
    values_to_render
}

#[cfg(test)]
mod tests {
    use super::{Renderable, get_values_to_render};
    use crate::{
        render_settings::{Cam, RenderSettings},
        universe::{Universe, universe_from_str},
    };

    fn get_universe() -> Universe {
        universe_from_str([
            "⬜⬛⬛⬛⬛⬛⬛⬛⬛⬜",
            "⬛⬜⬛⬛⬛⬛⬛⬛⬜⬛",
            "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
            "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
            "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
            "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
            "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
            "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
            "⬛⬜⬛⬛⬛⬛⬛⬛⬜⬛",
            "⬜⬛⬛⬛⬛⬛⬛⬛⬛⬜",
        ])
    }

    #[test]
    fn render() {
        let universe = get_universe();
        let s = RenderSettings { cam: Cam::of(-5, -5, 4, 4), dim: 1000, gap: 0 };
        assert_eq!(
            get_values_to_render(&universe, &s),
            [
                Renderable::of(0.0, 0.0, 100.0, 100.0),
                Renderable::of(0.0, 900.0, 100.0, 1000.0),
                Renderable::of(100.0, 100.0, 200.0, 200.0),
                Renderable::of(100.0, 800.0, 200.0, 900.0),
                Renderable::of(800.0, 100.0, 900.0, 200.0),
                Renderable::of(800.0, 800.0, 900.0, 900.0),
                Renderable::of(900.0, 0.0, 1000.0, 100.0),
                Renderable::of(900.0, 900.0, 1000.0, 1000.0),
            ]
        );
    }

    #[test]
    fn render_gap() {
        let universe = get_universe();
        let s_gap1 = RenderSettings { cam: Cam::of(-5, -5, 4, 4), dim: 1000, gap: 1 };
        let s_gap2 = RenderSettings { cam: Cam::of(-5, -5, 4, 4), dim: 1000, gap: 2 };
        assert_eq!(
            get_values_to_render(&universe, &s_gap1),
            [
                Renderable::of(1.0, 1.0, 99.0, 99.0),
                Renderable::of(1.0, 901.0, 99.0, 999.0),
                Renderable::of(101.0, 101.0, 199.0, 199.0),
                Renderable::of(101.0, 801.0, 199.0, 899.0),
                Renderable::of(801.0, 101.0, 899.0, 199.0),
                Renderable::of(801.0, 801.0, 899.0, 899.0),
                Renderable::of(901.0, 1.0, 999.0, 99.0),
                Renderable::of(901.0, 901.0, 999.0, 999.0),
            ]
        );
        assert_eq!(
            get_values_to_render(&universe, &s_gap2),
            [
                Renderable::of(2.0, 2.0, 98.0, 98.0),
                Renderable::of(2.0, 902.0, 98.0, 998.0),
                Renderable::of(102.0, 102.0, 198.0, 198.0),
                Renderable::of(102.0, 802.0, 198.0, 898.0),
                Renderable::of(802.0, 102.0, 898.0, 198.0),
                Renderable::of(802.0, 802.0, 898.0, 898.0),
                Renderable::of(902.0, 2.0, 998.0, 98.0),
                Renderable::of(902.0, 902.0, 998.0, 998.0),
            ]
        );
    }

    #[test]
    fn render_cam() {
        let universe = get_universe();
        let s_cam_minus1 = RenderSettings { cam: Cam::of(-6, -5, 3, 4), dim: 1000, gap: 0 };
        let s_cam_plus1 = RenderSettings { cam: Cam::of(-4, -5, 5, 4), dim: 1000, gap: 0 };
        assert_eq!(
            get_values_to_render(&universe, &s_cam_minus1),
            [
                Renderable::of(100.0, 0.0, 200.0, 100.0),
                Renderable::of(100.0, 900.0, 200.0, 1000.0),
                Renderable::of(200.0, 100.0, 300.0, 200.0),
                Renderable::of(200.0, 800.0, 300.0, 900.0),
                Renderable::of(900.0, 100.0, 1000.0, 200.0),
                Renderable::of(900.0, 800.0, 1000.0, 900.0),
            ]
        );
        assert_eq!(
            get_values_to_render(&universe, &s_cam_plus1),
            [
                Renderable::of(0.0, 100.0, 100.0, 200.0),
                Renderable::of(0.0, 800.0, 100.0, 900.0),
                Renderable::of(700.0, 100.0, 800.0, 200.0),
                Renderable::of(700.0, 800.0, 800.0, 900.0),
                Renderable::of(800.0, 0.0, 900.0, 100.0),
                Renderable::of(800.0, 900.0, 900.0, 1000.0),
            ]
        );
    }

    #[test]
    fn render_float_cell_size() {
        let universe = get_universe();
        let s = RenderSettings { cam: Cam::of(-5, -5, 4, 4), dim: 996, gap: 0 };
        assert_eq!(
            get_values_to_render(&universe, &s),
            [
                Renderable::of(0.0, 0.0, 99.6, 99.6),
                Renderable::of(0.0, 896.4, 99.6, 996.0),
                Renderable::of(99.6, 99.6, 199.2, 199.2),
                Renderable::of(99.6, 796.8, 199.2, 896.4),
                Renderable::of(796.8, 99.6, 896.4, 199.2),
                Renderable::of(796.8, 796.8, 896.4, 896.4),
                Renderable::of(896.4, 0.0, 996.0, 99.6),
                Renderable::of(896.4, 896.4, 996.0, 996.0)
            ]
        );
    }
}
