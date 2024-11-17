use super::{
    cell::State,
    coordinate::convert::cartesian_to_matrix,
    operations::{get_center_absolute, get_subdivision_size},
    poligon::{
        rect::{self, Rect},
        square::Sq,
    },
    universe::Universe,
};

fn render(universe: Universe, dim: u16, cam: Rect, gap: u16) -> Vec<Sq> {
    let length = rect::get_length(&cam);
    let subdivision_size = get_subdivision_size(&cam, dim);
    let center_absolute = get_center_absolute(&cam, dim);
    universe
        .value
        .iter()
        .filter(|value| {
            value.0.x >= cam.x1 && value.0.x <= cam.x2 && value.0.y >= cam.y1 && value.0.y <= cam.y2
        })
        .filter(|value| value.1 == &State::Alive)
        .map(|value| {
            let arr_index = cartesian_to_matrix(*value.0, length.into());
            Sq {
                x: arr_index.col as i64 * subdivision_size as i64 + gap as i64 - center_absolute.x,
                y: arr_index.row as i64 * subdivision_size as i64 + gap as i64 + center_absolute.y,
                size: subdivision_size as u64 - gap as u64 * 2,
            }
        })
        .collect()
}
