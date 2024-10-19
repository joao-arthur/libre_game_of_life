use std::collections::{HashMap, HashSet};

use crate::domain::{
    cell::{self, State},
    neighbor::number_of_alive_from_model,
    plane::{
        cartesian::{from_matrix, CartesianPoint},
        matrix::MatrixPoint,
    },
};

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

#[derive(Debug, PartialEq, Clone)]
pub struct Model {
    pub value: HashMap<CartesianPoint, State>,
    pub iter: u64,
    pub pos: Rect,
}

impl Model {
    pub fn from_pos(pos: Rect) -> Self {
        Model {
            pos,
            ..Default::default()
        }
    }

    pub fn from_value(value: HashMap<CartesianPoint, State>) -> Self {
        Model {
            value,
            ..Default::default()
        }
    }
}

impl Default for Model {
    fn default() -> Self {
        Model {
            value: HashMap::new(),
            iter: 0,
            pos: Rect::from(0, 0, 0, 0),
        }
    }
}

pub fn from_string(model_as_str: Vec<String>) -> Model {
    let mut value = HashMap::<CartesianPoint, State>::new();
    let len = model_as_str.len();
    let row_iter = model_as_str.iter().enumerate();
    for (row, row_str) in row_iter {
        let col_iter = row_str.chars().enumerate();
        for (col, col_str) in col_iter {
            if col_str == '⬜' {
                value.insert(
                    from_matrix(
                        MatrixPoint {
                            col: col.try_into().unwrap(),
                            row: row.try_into().unwrap(),
                        },
                        len.try_into().unwrap(),
                    ),
                    State::Alive,
                );
            }
        }
    }
    Model {
        value,
        iter: 0,
        pos: Rect::from(-10, -10, 10, 10),
    }
}

pub fn get_length(m: &Model) -> u16 {
    (m.pos.x2 - m.pos.x1 + 1).try_into().unwrap()
}

pub fn get_cell_size(m: &Model, total_size: u16) -> u16 {
    total_size / get_length(m)
}

pub fn get_middle_point(m: &Model) -> CartesianPoint {
    CartesianPoint::from((m.pos.x1 + m.pos.x2) / 2, (m.pos.y1 + m.pos.y2) / 2)
}

pub fn get_middle_cell(m: &Model, total_size: u16) -> CartesianPoint {
    let cell_size: i64 = get_cell_size(m, total_size).into();
    let middle = get_middle_point(m);
    CartesianPoint::from(middle.x * cell_size, middle.y * cell_size)
}

pub fn get_value(m: &Model, point: CartesianPoint) -> State {
    if m.value.get(&point).unwrap_or(&State::Dead) == &State::Alive {
        State::Alive
    } else {
        State::Dead
    }
}

pub fn iterate(m: &mut Model) {
    let points: HashSet<CartesianPoint> = m
        .value
        .keys()
        .flat_map(|p| {
            [
                CartesianPoint::from(p.x - 1, p.y + 1),
                CartesianPoint::from(p.x, p.y + 1),
                CartesianPoint::from(p.x + 1, p.y + 1),
                CartesianPoint::from(p.x - 1, p.y),
                p.clone(),
                CartesianPoint::from(p.x + 1, p.y),
                CartesianPoint::from(p.x - 1, p.y - 1),
                CartesianPoint::from(p.x, p.y - 1),
                CartesianPoint::from(p.x + 1, p.y - 1),
            ]
        })
        .collect();
    let entries: HashMap<CartesianPoint, State> = points
        .iter()
        .filter_map(|point| {
            let s = m.value.get(point).unwrap_or(&State::Dead);
            let number_of_alive_neighbors = number_of_alive_from_model(m, point.clone());
            let new_cell = cell::iterate(s.clone(), number_of_alive_neighbors);
            match new_cell {
                State::Dead => None,
                State::Alive => Some((point.clone(), State::Alive)),
            }
        })
        .collect();
    m.iter += 1;
    m.value = entries;
}

pub fn move_in_plane(m: &mut Model, delta: CartesianPoint) {
    m.pos = Rect::from(
        m.pos.x1 + delta.x,
        m.pos.y1 + delta.y,
        m.pos.x2 + delta.x,
        m.pos.y2 + delta.y,
    )
}

pub fn toggle_cell(m: &mut Model, point: CartesianPoint) {
    let new_cell = cell::toggle(m.value.get(&point).unwrap_or(&State::Dead));
    match new_cell {
        State::Dead => {
            m.value.remove(&point);
        }
        State::Alive => {
            m.value.insert(point, new_cell);
        }
    }
}

pub fn zoom(m: &mut Model, new_size: u16) {
    let half_new_size = new_size as f64 / 2 as f64;
    let half_x = (m.pos.x1 + m.pos.x2) as f64 / 2 as f64;
    let half_y = (m.pos.y1 + m.pos.y2) as f64 / 2 as f64;
    let x1 = (half_x - half_new_size).ceil() as i64;
    let y1 = (half_y - half_new_size).ceil() as i64;
    let x2 = x1 + new_size as i64 - 1;
    let y2 = y1 + new_size as i64 - 1;
    m.pos = Rect::from(x1, y1, x2, y2);
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::domain::cell::State;

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
    fn test_model() {
        assert_eq!(
            Model::default(),
            Model {
                value: HashMap::new(),
                iter: 0,
                pos: Rect::from(0, 0, 0, 0)
            }
        );
        assert_eq!(
            Model::from_pos(Rect::from(-23, 38, 198, 7)),
            Model {
                value: HashMap::new(),
                iter: 0,
                pos: Rect::from(-23, 38, 198, 7)
            }
        );
        assert_eq!(
            Model::from_value(HashMap::from([
                (CartesianPoint::from(-1, -1), State::Alive),
                (CartesianPoint::from(-1, 1), State::Alive),
                (CartesianPoint::from(1, -1), State::Alive),
                (CartesianPoint::from(1, 1), State::Alive),
            ])),
            Model {
                value: HashMap::from([
                    (CartesianPoint::from(-1, -1), State::Alive),
                    (CartesianPoint::from(-1, 1), State::Alive),
                    (CartesianPoint::from(1, -1), State::Alive),
                    (CartesianPoint::from(1, 1), State::Alive),
                ]),
                iter: 0,
                pos: Rect::from(0, 0, 0, 0)
            }
        );
    }

    #[test]
    fn test_from_string() {
        assert_eq!(
            from_string(vec!["".to_string()]),
            Model::from_pos(Rect::from(-10, -10, 10, 10))
        );
        assert_eq!(
            from_string(vec!["⬛".to_string()]),
            Model {
                value: HashMap::new(),
                iter: 0,
                pos: Rect::from(-10, -10, 10, 10)
            }
        );
        assert_eq!(
            from_string(vec!["⬜".to_string()]),
            Model {
                value: HashMap::from([(CartesianPoint::from(0, 0), State::Alive)]),
                iter: 0,
                pos: Rect::from(-10, -10, 10, 10),
            }
        );
        assert_eq!(
            from_string(vec![
                "⬛⬛⬛⬜".to_string(),
                "⬜⬛⬛⬛".to_string(),
                "⬛⬛⬜⬛".to_string(),
                "⬛⬛⬛⬛".to_string(),
            ]),
            Model {
                value: HashMap::from([
                    (CartesianPoint::from(1, 2), State::Alive),
                    (CartesianPoint::from(-2, 1), State::Alive),
                    (CartesianPoint::from(0, 0), State::Alive),
                ]),
                iter: 0,
                pos: Rect::from(-10, -10, 10, 10),
            }
        );
    }

    #[test]
    fn test_get_length() {
        assert_eq!(
            get_length(&Model::from_pos(Rect::from(-10, -10, 10, 10))),
            21
        );
        assert_eq!(get_length(&Model::from_pos(Rect::from(1, 1, 10, 10))), 10);
        assert_eq!(get_length(&Model::from_pos(Rect::from(4, 4, 5, 5))), 2);
        assert_eq!(get_length(&Model::from_pos(Rect::from(5, 5, 5, 5))), 1);
    }

    #[test]
    fn test_get_cell_size() {
        let m = Model::from_pos(Rect::from(1, 1, 10, 10));
        assert_eq!(get_cell_size(&m, 100), 10);
        assert_eq!(get_cell_size(&m, 90), 9);
        assert_eq!(get_cell_size(&m, 50), 5);
        assert_eq!(get_cell_size(&m, 10), 1);
    }

    #[test]
    fn test_get_middle_point() {
        assert_eq!(
            get_middle_point(&Model::from_pos(Rect::from(-10, -10, 10, 10))),
            CartesianPoint::from(0, 0)
        );
        // assert_eq!(
        //     get_middle_point(&Model::from_pos(Rect::from(1, 1, 10, 10))),
        //     CartesianPoint::from(5.5, 5.5)
        // );
        // assert_eq!(
        //     get_middle_point(&Model::from_pos(Rect::from(4, 4, 5, 5))),
        //     CartesianPoint::from(4.5, 4.5)
        // );
        assert_eq!(
            get_middle_point(&Model::from_pos(Rect::from(5, 5, 5, 5))),
            CartesianPoint::from(5, 5)
        );
    }

    #[test]
    fn test_get_middle_cell() {
        assert_eq!(
            get_middle_cell(&Model::from_pos(Rect::from(-10, -10, 10, 10)), 100),
            CartesianPoint::from(0, 0)
        );
        // assert_eq!(
        //     get_middle_cell(&Model::from_pos(Rect::from(1, 1, 10, 10)), 50),
        //     CartesianPoint::from(27.5, 27.5)
        // );
        // assert_eq!(
        //     get_middle_cell(&Model::from_pos(Rect::from(4, 4, 5, 5)), 10),
        //     CartesianPoint::from(22.5, 22.5)
        // );
        assert_eq!(
            get_middle_cell(&Model::from_pos(Rect::from(5, 5, 5, 5)), 1),
            CartesianPoint::from(5, 5)
        );
    }

    #[test]
    fn test_move_in_plane() {
        let mut m = Model::from_pos(Rect::from(-10, -10, 10, 10));
        move_in_plane(&mut m, CartesianPoint::from(1, 0));
        assert_eq!(m.pos, Rect::from(-9, -10, 11, 10));
        move_in_plane(&mut m, CartesianPoint::from(-2, 0));
        assert_eq!(m.pos, Rect::from(-11, -10, 9, 10));
        move_in_plane(&mut m, CartesianPoint::from(0, 1));
        assert_eq!(m.pos, Rect::from(-11, -9, 9, 11));
        move_in_plane(&mut m, CartesianPoint::from(0, -2));
        assert_eq!(m.pos, Rect::from(-11, -11, 9, 9));
        move_in_plane(&mut m, CartesianPoint::from(11, 0));
        assert_eq!(m.pos, Rect::from(0, -11, 20, 9));
        move_in_plane(&mut m, CartesianPoint::from(0, 11));
        assert_eq!(m.pos, Rect::from(0, 0, 20, 20));
        move_in_plane(&mut m, CartesianPoint::from(-20, 0));
        assert_eq!(m.pos, Rect::from(-20, 0, 0, 20));
        move_in_plane(&mut m, CartesianPoint::from(0, -20));
        assert_eq!(m.pos, Rect::from(-20, -20, 0, 0));
    }

    #[test]
    fn test_zoom_odd_size() {
        let mut m = Model::from_pos(Rect::from(-10, -10, 10, 10));
        zoom(&mut m, 1);
        assert_eq!(m.pos, Rect::from(0, 0, 0, 0));
        zoom(&mut m, 2);
        assert_eq!(m.pos, Rect::from(-1, -1, 0, 0));
        zoom(&mut m, 3);
        assert_eq!(m.pos, Rect::from(-2, -2, 0, 0));
        zoom(&mut m, 19);
        assert_eq!(m.pos, Rect::from(-10, -10, 8, 8));
        zoom(&mut m, 21);
        assert_eq!(m.pos, Rect::from(-11, -11, 9, 9));
        zoom(&mut m, 23);
        assert_eq!(m.pos, Rect::from(-12, -12, 10, 10));
    }

    #[test]
    fn test_zoom_even_size() {
        let mut m = Model::from_pos(Rect::from(10, 10, 19, 19));
        zoom(&mut m, 1);
        assert_eq!(m.pos, Rect::from(14, 14, 14, 14));
        zoom(&mut m, 2);
        assert_eq!(m.pos, Rect::from(13, 13, 14, 14));
        zoom(&mut m, 3);
        assert_eq!(m.pos, Rect::from(12, 12, 14, 14));
        zoom(&mut m, 8);
        assert_eq!(m.pos, Rect::from(9, 9, 16, 16));
        zoom(&mut m, 10);
        assert_eq!(m.pos, Rect::from(8, 8, 17, 17));
        zoom(&mut m, 12);
        assert_eq!(m.pos, Rect::from(7, 7, 18, 18));
    }

    #[test]
    fn test_toggle_model() {
        let mut m = from_string(vec![
            "⬛⬛⬛⬛".to_string(),
            "⬛⬛⬛⬛".to_string(),
            "⬜⬜⬜⬜".to_string(),
            "⬜⬜⬜⬜".to_string(),
        ]);
        let state1 = from_string(vec![
            "⬜⬛⬛⬛".to_string(),
            "⬛⬛⬛⬛".to_string(),
            "⬜⬜⬜⬜".to_string(),
            "⬜⬜⬜⬜".to_string(),
        ]);
        toggle_cell(&mut m, CartesianPoint::from(-2, 2));
        assert_eq!(m, state1);
        let state2 = from_string(vec![
            "⬜⬛⬛⬛".to_string(),
            "⬛⬜⬛⬛".to_string(),
            "⬜⬜⬜⬜".to_string(),
            "⬜⬜⬜⬜".to_string(),
        ]);
        toggle_cell(&mut m, CartesianPoint::from(-1, 1));
        assert_eq!(m, state2);
        let state3 = from_string(vec![
            "⬜⬛⬛⬛".to_string(),
            "⬛⬜⬛⬛".to_string(),
            "⬜⬜⬛⬜".to_string(),
            "⬜⬜⬜⬜".to_string(),
        ]);
        toggle_cell(&mut m, CartesianPoint::from(0, 0));
        assert_eq!(m, state3);
        let state4 = from_string(vec![
            "⬜⬛⬛⬛".to_string(),
            "⬛⬜⬛⬛".to_string(),
            "⬜⬜⬛⬜".to_string(),
            "⬜⬜⬜⬛".to_string(),
        ]);
        toggle_cell(&mut m, CartesianPoint::from(1, -1));
        assert_eq!(m, state4);
        let state5 = from_string(vec![
            "⬜⬛⬛⬛".to_string(),
            "⬛⬜⬛⬛".to_string(),
            "⬜⬜⬛⬜".to_string(),
            "⬛⬜⬜⬛".to_string(),
        ]);
        toggle_cell(&mut m, CartesianPoint::from(-2, -1));
        assert_eq!(m, state5);
        let state6 = from_string(vec![
            "⬜⬛⬛⬛".to_string(),
            "⬛⬜⬛⬛".to_string(),
            "⬜⬛⬛⬜".to_string(),
            "⬛⬜⬜⬛".to_string(),
        ]);
        toggle_cell(&mut m, CartesianPoint::from(-1, 0));
        assert_eq!(m, state6);
        let state7 = from_string(vec![
            "⬜⬛⬛⬛".to_string(),
            "⬛⬜⬜⬛".to_string(),
            "⬜⬛⬛⬜".to_string(),
            "⬛⬜⬜⬛".to_string(),
        ]);
        toggle_cell(&mut m, CartesianPoint::from(0, 1));
        assert_eq!(m, state7);
        let state8 = from_string(vec![
            "⬜⬛⬛⬜".to_string(),
            "⬛⬜⬜⬛".to_string(),
            "⬜⬛⬛⬜".to_string(),
            "⬛⬜⬜⬛".to_string(),
        ]);
        toggle_cell(&mut m, CartesianPoint::from(1, 2));
        assert_eq!(m, state8);
    }

    #[test]
    fn test_iterate() {
        let mut model1x1iter0 = from_string(vec!["⬜".to_string()]);
        let mut model1x1iter1 = from_string(vec!["⬛".to_string()]);
        model1x1iter1.iter = 1;
        iterate(&mut model1x1iter0);
        assert_eq!(model1x1iter0, model1x1iter1);

        let mut model2x2iter0 = from_string(vec!["⬜⬜".to_string(), "⬜⬜".to_string()]);
        let mut model2x2iter1 = from_string(vec!["⬜⬜".to_string(), "⬜⬜".to_string()]);
        model2x2iter1.iter = 1;
        iterate(&mut model2x2iter0);
        assert_eq!(model2x2iter0, model2x2iter1);

        let mut model3x3_1_iter0 = from_string(vec![
            "⬛⬜⬛".to_string(),
            "⬛⬜⬛".to_string(),
            "⬛⬜⬛".to_string(),
        ]);
        let mut model3x3_1_iter1 = from_string(vec![
            "⬛⬛⬛".to_string(),
            "⬜⬜⬜".to_string(),
            "⬛⬛⬛".to_string(),
        ]);
        model3x3_1_iter1.iter = 1;
        iterate(&mut model3x3_1_iter0);
        assert_eq!(model3x3_1_iter0, model3x3_1_iter1);

        let mut model3x3_2_iter0 = from_string(vec![
            "⬛⬛⬛".to_string(),
            "⬜⬜⬜".to_string(),
            "⬛⬛⬛".to_string(),
        ]);
        let mut model3x3_2_iter1 = from_string(vec![
            "⬛⬜⬛".to_string(),
            "⬛⬜⬛".to_string(),
            "⬛⬜⬛".to_string(),
        ]);
        model3x3_2_iter1.iter = 1;
        iterate(&mut model3x3_2_iter0);
        assert_eq!(model3x3_2_iter0, model3x3_2_iter1);

        let mut model3x3_3_iter0 = from_string(vec![
            "⬛⬛⬜".to_string(),
            "⬜⬜⬜".to_string(),
            "⬛⬛⬛".to_string(),
        ]);
        let mut model3x3_3_iter1 = from_string(vec![
            "⬛⬛⬜".to_string(),
            "⬛⬜⬜".to_string(),
            "⬛⬜⬛".to_string(),
        ]);
        model3x3_3_iter1.iter = 1;
        iterate(&mut model3x3_3_iter0);
        assert_eq!(model3x3_3_iter0, model3x3_3_iter1);

        let mut model3x3_4_iter0 = from_string(vec![
            "⬛⬛⬜".to_string(),
            "⬛⬜⬜".to_string(),
            "⬛⬜⬛".to_string(),
        ]);
        let mut model3x3_4_iter1 = from_string(vec![
            "⬛⬜⬜".to_string(),
            "⬛⬜⬜".to_string(),
            "⬛⬜⬜".to_string(),
        ]);
        model3x3_4_iter1.iter = 1;
        iterate(&mut model3x3_4_iter0);
        assert_eq!(model3x3_4_iter0, model3x3_4_iter1);

        let mut model3x3_5_iter0 = from_string(vec![
            "⬜⬜⬛".to_string(),
            "⬜⬜⬜".to_string(),
            "⬛⬜⬛".to_string(),
        ]);
        let mut model3x3_5_iter1 = from_string(vec![
            "⬜⬛⬜".to_string(),
            "⬛⬛⬜".to_string(),
            "⬜⬜⬜".to_string(),
        ]);
        model3x3_5_iter1.iter = 1;
        iterate(&mut model3x3_5_iter0);
        assert_eq!(model3x3_5_iter0, model3x3_5_iter1);
    }
}
