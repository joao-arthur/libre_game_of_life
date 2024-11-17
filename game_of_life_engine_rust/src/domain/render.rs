use super::{
    cell::State,
    geometry::{
        coordinate::{cartesian_to_matrix, CartesianP},
        poligon::{
            rect::{get_center, get_length, Rect},
            square::Sq,
        },
    },
    universe::Universe,
};

#[derive(Debug, PartialEq, Clone)]
pub struct RenderSettings {
    pub cam: Rect,
    pub dim: u16,
    pub gap: u8,
}

pub fn get_values_to_render(u: &Universe, s: &RenderSettings) -> Vec<Sq> {
    let length = get_length(&s.cam);
    let center = get_center(&s.cam);
    let subdivision_size = s.dim as u64 / length;
    let center_absolute =
        CartesianP { x: center.x * subdivision_size as i64, y: center.y * subdivision_size as i64 };
    let mut values_to_render: Vec<Sq> = u
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
            let arr_index = cartesian_to_matrix(*value.0, length.into());
            let gap = s.gap;
            Sq {
                x: arr_index.col as i64 * subdivision_size as i64 + gap as i64 - center_absolute.x,
                y: arr_index.row as i64 * subdivision_size as i64 + gap as i64 + center_absolute.y,
                size: subdivision_size as u64 - gap as u64 * 2,
            }
        })
        .collect();
    values_to_render.sort_by(|a, b| a.y.cmp(&b.y));
    values_to_render.sort_by(|a, b| a.x.cmp(&b.x));

    values_to_render
}

#[cfg(test)]
mod test {
    use crate::domain::universe::from_string;

    use super::*;

    #[test]
    fn test_render_gap() {
        let model = from_string(vec![
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
        .unwrap();
        let render_settings_gap0 =
            RenderSettings { cam: Rect::from(-5, -4, 4, 5), dim: 1000, gap: 0 };
        let render_settings_gap1 =
            RenderSettings { cam: Rect::from(-5, -4, 4, 5), dim: 1000, gap: 1 };
        let render_settings_gap2 =
            RenderSettings { cam: Rect::from(-5, -4, 4, 5), dim: 1000, gap: 2 };
        assert_eq!(
            get_values_to_render(&model, &render_settings_gap0),
            vec![
                Sq { x: 0, y: 0, size: 100 },
                Sq { x: 0, y: 900, size: 100 },
                Sq { x: 100, y: 100, size: 100 },
                Sq { x: 100, y: 800, size: 100 },
                Sq { x: 800, y: 100, size: 100 },
                Sq { x: 800, y: 800, size: 100 },
                Sq { x: 900, y: 0, size: 100 },
                Sq { x: 900, y: 900, size: 100 },
            ]
        );
        assert_eq!(
            get_values_to_render(&model, &render_settings_gap1),
            vec![
                Sq { x: 1, y: 1, size: 98 },
                Sq { x: 1, y: 901, size: 98 },
                Sq { x: 101, y: 101, size: 98 },
                Sq { x: 101, y: 801, size: 98 },
                Sq { x: 801, y: 101, size: 98 },
                Sq { x: 801, y: 801, size: 98 },
                Sq { x: 901, y: 1, size: 98 },
                Sq { x: 901, y: 901, size: 98 },
            ]
        );
        assert_eq!(
            get_values_to_render(&model, &render_settings_gap2),
            vec![
                Sq { x: 2, y: 2, size: 96 },
                Sq { x: 2, y: 902, size: 96 },
                Sq { x: 102, y: 102, size: 96 },
                Sq { x: 102, y: 802, size: 96 },
                Sq { x: 802, y: 102, size: 96 },
                Sq { x: 802, y: 802, size: 96 },
                Sq { x: 902, y: 2, size: 96 },
                Sq { x: 902, y: 902, size: 96 },
            ]
        );
    }
}
