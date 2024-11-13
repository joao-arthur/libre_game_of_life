use super::{
    camera::{get_center_absolute, get_length, get_subdivision_size},
    cell::State,
    plane::{
        cartesian::{self, CartesianPoint},
        shape::{Rect, Square},
    },
    universe::Universe,
};

const DEAD_COLOR: &str = "#dbdbdb";
const ALIVE_COLOR: &str = "#2e2e2e";

fn render(universe: Universe, dim: u16, cam: Rect, gap: u16) {
    let length = get_length(&cam);
    let subdivision_size = get_subdivision_size(&cam, dim);
    let center_absolute = get_center_absolute(&cam, dim);

    let cam = cam;
    let background = Square {
        x: 0,
        y: 0,
        size: dim.into(),
    };

    let values_in_camera: Vec<(&CartesianPoint, &State)> = universe
        .value
        .iter()
        .filter(|value| {
            value.0.x >= cam.x1 && value.0.x <= cam.x2 && value.0.y >= cam.y1 && value.0.y <= cam.y2
        })
        .collect();
    for p in values_in_camera {
        let arr_index = cartesian::to_matrix(*p.0, length.into());
        match p.1 {
            State::Alive => {
                let s = Square {
                    x: arr_index.col as i64 * subdivision_size as i64 + gap as i64
                        - center_absolute.x,
                    y: arr_index.row as i64 * subdivision_size as i64
                        + gap as i64
                        + center_absolute.y,
                    size: subdivision_size as u64 - gap as u64 * 2,
                };
            }
            _ => {}
        }
    }
}
