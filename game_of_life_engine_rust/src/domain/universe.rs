use std::{
    collections::{HashMap, HashSet},
    fmt,
};

use crate::domain::{
    cell::{self, State},
    neighbor::number_of_alive_from_model,
    plane::{
        cartesian::{from_matrix, CartesianPoint},
        matrix::MatrixPoint,
        Rect,
    },
};

#[derive(Debug, PartialEq, Clone)]
pub struct Universe {
    pub value: HashMap<CartesianPoint, State>,
    pub iter: u64,
    pub pos: Rect,
}

impl Universe {
    pub fn from_pos(pos: Rect) -> Self {
        Universe {
            pos,
            ..Default::default()
        }
    }

    pub fn from_value(value: HashMap<CartesianPoint, State>) -> Self {
        Universe {
            value,
            ..Default::default()
        }
    }
}

impl Default for Universe {
    fn default() -> Self {
        Universe {
            value: HashMap::new(),
            iter: 0,
            pos: Rect::from(0, 0, 0, 0),
        }
    }
}

#[derive(Debug)]
pub struct InvalidCharacterError;

impl fmt::Display for InvalidCharacterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Only \"⬜\" and \"⬛\" characters are allowed!")
    }
}

#[derive(Debug)]
pub struct InvalidLengthError;

impl fmt::Display for InvalidLengthError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "The length of every line and the number of lines must be equal!"
        )
    }
}

#[derive(Debug)]
pub enum FromStringError {
    InvalidCharacter(InvalidCharacterError),
    InvalidLength(InvalidLengthError),
}

pub fn from_string(as_str: Vec<String>) -> Result<Universe, FromStringError> {
    let mut value = HashMap::<CartesianPoint, State>::new();
    let len = as_str.len();
    // TODO errors
    let row_iter = as_str.iter().enumerate();
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
    Ok(Universe {
        value,
        iter: 0,
        pos: Rect::from(-10, -10, 10, 10),
    })
}

pub fn get_length(u: &Universe) -> u16 {
    (u.pos.x2 - u.pos.x1 + 1).try_into().unwrap()
}

pub fn get_cell_size(u: &Universe, total_size: u16) -> u16 {
    total_size / get_length(u)
}

pub fn get_middle_point(u: &Universe) -> CartesianPoint {
    CartesianPoint::from((u.pos.x1 + u.pos.x2) / 2, (u.pos.y1 + u.pos.y2) / 2)
}

pub fn get_middle_cell(u: &Universe, total_size: u16) -> CartesianPoint {
    let cell_size: i64 = get_cell_size(u, total_size).into();
    let middle = get_middle_point(u);
    CartesianPoint::from(middle.x * cell_size, middle.y * cell_size)
}

pub fn get_value(u: &Universe, point: CartesianPoint) -> State {
    if u.value.get(&point).unwrap_or(&State::Dead) == &State::Alive {
        State::Alive
    } else {
        State::Dead
    }
}

pub fn iterate(u: &mut Universe) {
    let points: HashSet<CartesianPoint> = u
        .value
        .keys()
        .flat_map(|point| {
            [
                CartesianPoint::from(point.x - 1, point.y + 1),
                CartesianPoint::from(point.x, point.y + 1),
                CartesianPoint::from(point.x + 1, point.y + 1),
                CartesianPoint::from(point.x - 1, point.y),
                point.clone(),
                CartesianPoint::from(point.x + 1, point.y),
                CartesianPoint::from(point.x - 1, point.y - 1),
                CartesianPoint::from(point.x, point.y - 1),
                CartesianPoint::from(point.x + 1, point.y - 1),
            ]
        })
        .collect();
    let entries: HashMap<CartesianPoint, State> = points
        .iter()
        .filter_map(|point| {
            let s = u.value.get(point).unwrap_or(&State::Dead);
            let number_of_alive_neighbors = number_of_alive_from_model(u, point.clone());
            let new_cell = cell::iterate(s.clone(), number_of_alive_neighbors);
            match new_cell {
                State::Dead => None,
                State::Alive => Some((point.clone(), State::Alive)),
            }
        })
        .collect();
    u.iter += 1;
    u.value = entries;
}

pub fn move_in_plane(u: &mut Universe, delta: CartesianPoint) {
    u.pos = Rect::from(
        u.pos.x1 + delta.x,
        u.pos.y1 + delta.y,
        u.pos.x2 + delta.x,
        u.pos.y2 + delta.y,
    )
}

pub fn toggle_cell(u: &mut Universe, point: CartesianPoint) {
    let new_cell = cell::toggle(u.value.get(&point).unwrap_or(&State::Dead));
    match new_cell {
        State::Dead => {
            u.value.remove(&point);
        }
        State::Alive => {
            u.value.insert(point, new_cell);
        }
    }
}

pub fn zoom(u: &mut Universe, new_size: u16) {
    let half_new_size = new_size as f64 / 2 as f64;
    let half_x = (u.pos.x1 + u.pos.x2) as f64 / 2 as f64;
    let half_y = (u.pos.y1 + u.pos.y2) as f64 / 2 as f64;
    let x1 = (half_x - half_new_size).ceil() as i64;
    let y1 = (half_y - half_new_size).ceil() as i64;
    let x2 = x1 + new_size as i64 - 1;
    let y2 = y1 + new_size as i64 - 1;
    u.pos = Rect::from(x1, y1, x2, y2);
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::domain::cell::State;

    #[test]
    fn test_model() {
        assert_eq!(
            Universe::default(),
            Universe {
                value: HashMap::new(),
                iter: 0,
                pos: Rect::from(0, 0, 0, 0)
            }
        );
        assert_eq!(
            Universe::from_pos(Rect::from(-23, 38, 198, 7)),
            Universe {
                value: HashMap::new(),
                iter: 0,
                pos: Rect::from(-23, 38, 198, 7)
            }
        );
        assert_eq!(
            Universe::from_value(HashMap::from([
                (CartesianPoint::from(-1, -1), State::Alive),
                (CartesianPoint::from(-1, 1), State::Alive),
                (CartesianPoint::from(1, -1), State::Alive),
                (CartesianPoint::from(1, 1), State::Alive),
            ])),
            Universe {
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
            from_string(vec!["".to_string()]).unwrap(),
            Universe::from_pos(Rect::from(-10, -10, 10, 10))
        );
        assert_eq!(
            from_string(vec!["⬛".to_string()]).unwrap(),
            Universe {
                value: HashMap::new(),
                iter: 0,
                pos: Rect::from(-10, -10, 10, 10)
            }
        );
        assert_eq!(
            from_string(vec!["⬜".to_string()]).unwrap(),
            Universe {
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
            ])
            .unwrap(),
            Universe {
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
            get_length(&Universe::from_pos(Rect::from(-10, -10, 10, 10))),
            21
        );
        assert_eq!(
            get_length(&Universe::from_pos(Rect::from(1, 1, 10, 10))),
            10
        );
        assert_eq!(get_length(&Universe::from_pos(Rect::from(4, 4, 5, 5))), 2);
        assert_eq!(get_length(&Universe::from_pos(Rect::from(5, 5, 5, 5))), 1);
    }

    #[test]
    fn test_get_cell_size() {
        let u = Universe::from_pos(Rect::from(1, 1, 10, 10));
        assert_eq!(get_cell_size(&u, 100), 10);
        assert_eq!(get_cell_size(&u, 90), 9);
        assert_eq!(get_cell_size(&u, 50), 5);
        assert_eq!(get_cell_size(&u, 10), 1);
    }

    #[test]
    fn test_get_middle_point() {
        assert_eq!(
            get_middle_point(&Universe::from_pos(Rect::from(-10, -10, 10, 10))),
            CartesianPoint::from(0, 0)
        );
        // assert_eq!(
        //     get_middle_point(&Universe::from_pos(Rect::from(1, 1, 10, 10))),
        //     CartesianPoint::from(5.5, 5.5)
        // );
        // assert_eq!(
        //     get_middle_point(&Universe::from_pos(Rect::from(4, 4, 5, 5))),
        //     CartesianPoint::from(4.5, 4.5)
        // );
        assert_eq!(
            get_middle_point(&Universe::from_pos(Rect::from(5, 5, 5, 5))),
            CartesianPoint::from(5, 5)
        );
    }

    #[test]
    fn test_get_middle_cell() {
        assert_eq!(
            get_middle_cell(&Universe::from_pos(Rect::from(-10, -10, 10, 10)), 100),
            CartesianPoint::from(0, 0)
        );
        // assert_eq!(
        //     get_middle_cell(&Universe::from_pos(Rect::from(1, 1, 10, 10)), 50),
        //     CartesianPoint::from(27.5, 27.5)
        // );
        // assert_eq!(
        //     get_middle_cell(&Universe::from_pos(Rect::from(4, 4, 5, 5)), 10),
        //     CartesianPoint::from(22.5, 22.5)
        // );
        assert_eq!(
            get_middle_cell(&Universe::from_pos(Rect::from(5, 5, 5, 5)), 1),
            CartesianPoint::from(5, 5)
        );
    }

    #[test]
    fn test_move_in_plane() {
        let mut u = Universe::from_pos(Rect::from(-10, -10, 10, 10));
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

    #[test]
    fn test_zoom_odd_size() {
        let mut u = Universe::from_pos(Rect::from(-10, -10, 10, 10));
        zoom(&mut u, 1);
        assert_eq!(u.pos, Rect::from(0, 0, 0, 0));
        zoom(&mut u, 2);
        assert_eq!(u.pos, Rect::from(-1, -1, 0, 0));
        zoom(&mut u, 3);
        assert_eq!(u.pos, Rect::from(-2, -2, 0, 0));
        zoom(&mut u, 19);
        assert_eq!(u.pos, Rect::from(-10, -10, 8, 8));
        zoom(&mut u, 21);
        assert_eq!(u.pos, Rect::from(-11, -11, 9, 9));
        zoom(&mut u, 23);
        assert_eq!(u.pos, Rect::from(-12, -12, 10, 10));
    }

    #[test]
    fn test_zoom_even_size() {
        let mut u = Universe::from_pos(Rect::from(10, 10, 19, 19));
        zoom(&mut u, 1);
        assert_eq!(u.pos, Rect::from(14, 14, 14, 14));
        zoom(&mut u, 2);
        assert_eq!(u.pos, Rect::from(13, 13, 14, 14));
        zoom(&mut u, 3);
        assert_eq!(u.pos, Rect::from(12, 12, 14, 14));
        zoom(&mut u, 8);
        assert_eq!(u.pos, Rect::from(9, 9, 16, 16));
        zoom(&mut u, 10);
        assert_eq!(u.pos, Rect::from(8, 8, 17, 17));
        zoom(&mut u, 12);
        assert_eq!(u.pos, Rect::from(7, 7, 18, 18));
    }

    #[test]
    fn test_toggle_model() {
        let mut u = from_string(vec![
            "⬛⬛⬛⬛".to_string(),
            "⬛⬛⬛⬛".to_string(),
            "⬜⬜⬜⬜".to_string(),
            "⬜⬜⬜⬜".to_string(),
        ])
        .unwrap();
        let state1 = from_string(vec![
            "⬜⬛⬛⬛".to_string(),
            "⬛⬛⬛⬛".to_string(),
            "⬜⬜⬜⬜".to_string(),
            "⬜⬜⬜⬜".to_string(),
        ])
        .unwrap();
        toggle_cell(&mut u, CartesianPoint::from(-2, 2));
        assert_eq!(u, state1);
        let state2 = from_string(vec![
            "⬜⬛⬛⬛".to_string(),
            "⬛⬜⬛⬛".to_string(),
            "⬜⬜⬜⬜".to_string(),
            "⬜⬜⬜⬜".to_string(),
        ])
        .unwrap();
        toggle_cell(&mut u, CartesianPoint::from(-1, 1));
        assert_eq!(u, state2);
        let state3 = from_string(vec![
            "⬜⬛⬛⬛".to_string(),
            "⬛⬜⬛⬛".to_string(),
            "⬜⬜⬛⬜".to_string(),
            "⬜⬜⬜⬜".to_string(),
        ])
        .unwrap();
        toggle_cell(&mut u, CartesianPoint::from(0, 0));
        assert_eq!(u, state3);
        let state4 = from_string(vec![
            "⬜⬛⬛⬛".to_string(),
            "⬛⬜⬛⬛".to_string(),
            "⬜⬜⬛⬜".to_string(),
            "⬜⬜⬜⬛".to_string(),
        ])
        .unwrap();
        toggle_cell(&mut u, CartesianPoint::from(1, -1));
        assert_eq!(u, state4);
        let state5 = from_string(vec![
            "⬜⬛⬛⬛".to_string(),
            "⬛⬜⬛⬛".to_string(),
            "⬜⬜⬛⬜".to_string(),
            "⬛⬜⬜⬛".to_string(),
        ])
        .unwrap();
        toggle_cell(&mut u, CartesianPoint::from(-2, -1));
        assert_eq!(u, state5);
        let state6 = from_string(vec![
            "⬜⬛⬛⬛".to_string(),
            "⬛⬜⬛⬛".to_string(),
            "⬜⬛⬛⬜".to_string(),
            "⬛⬜⬜⬛".to_string(),
        ])
        .unwrap();
        toggle_cell(&mut u, CartesianPoint::from(-1, 0));
        assert_eq!(u, state6);
        let state7 = from_string(vec![
            "⬜⬛⬛⬛".to_string(),
            "⬛⬜⬜⬛".to_string(),
            "⬜⬛⬛⬜".to_string(),
            "⬛⬜⬜⬛".to_string(),
        ])
        .unwrap();
        toggle_cell(&mut u, CartesianPoint::from(0, 1));
        assert_eq!(u, state7);
        let state8 = from_string(vec![
            "⬜⬛⬛⬜".to_string(),
            "⬛⬜⬜⬛".to_string(),
            "⬜⬛⬛⬜".to_string(),
            "⬛⬜⬜⬛".to_string(),
        ])
        .unwrap();
        toggle_cell(&mut u, CartesianPoint::from(1, 2));
        assert_eq!(u, state8);
    }

    #[test]
    fn test_iterate() {
        let mut model1x1iter0 = from_string(vec!["⬜".to_string()]).unwrap();
        let mut model1x1iter1 = from_string(vec!["⬛".to_string()]).unwrap();
        model1x1iter1.iter = 1;
        iterate(&mut model1x1iter0);
        assert_eq!(model1x1iter0, model1x1iter1);

        let mut model2x2iter0 = from_string(vec!["⬜⬜".to_string(), "⬜⬜".to_string()]).unwrap();
        let mut model2x2iter1 = from_string(vec!["⬜⬜".to_string(), "⬜⬜".to_string()]).unwrap();
        model2x2iter1.iter = 1;
        iterate(&mut model2x2iter0);
        assert_eq!(model2x2iter0, model2x2iter1);

        let mut model3x3_1_iter0 = from_string(vec![
            "⬛⬜⬛".to_string(),
            "⬛⬜⬛".to_string(),
            "⬛⬜⬛".to_string(),
        ])
        .unwrap();
        let mut model3x3_1_iter1 = from_string(vec![
            "⬛⬛⬛".to_string(),
            "⬜⬜⬜".to_string(),
            "⬛⬛⬛".to_string(),
        ])
        .unwrap();
        model3x3_1_iter1.iter = 1;
        iterate(&mut model3x3_1_iter0);
        assert_eq!(model3x3_1_iter0, model3x3_1_iter1);

        let mut model3x3_2_iter0 = from_string(vec![
            "⬛⬛⬛".to_string(),
            "⬜⬜⬜".to_string(),
            "⬛⬛⬛".to_string(),
        ])
        .unwrap();
        let mut model3x3_2_iter1 = from_string(vec![
            "⬛⬜⬛".to_string(),
            "⬛⬜⬛".to_string(),
            "⬛⬜⬛".to_string(),
        ])
        .unwrap();
        model3x3_2_iter1.iter = 1;
        iterate(&mut model3x3_2_iter0);
        assert_eq!(model3x3_2_iter0, model3x3_2_iter1);

        let mut model3x3_3_iter0 = from_string(vec![
            "⬛⬛⬜".to_string(),
            "⬜⬜⬜".to_string(),
            "⬛⬛⬛".to_string(),
        ])
        .unwrap();
        let mut model3x3_3_iter1 = from_string(vec![
            "⬛⬛⬜".to_string(),
            "⬛⬜⬜".to_string(),
            "⬛⬜⬛".to_string(),
        ])
        .unwrap();
        model3x3_3_iter1.iter = 1;
        iterate(&mut model3x3_3_iter0);
        assert_eq!(model3x3_3_iter0, model3x3_3_iter1);

        let mut model3x3_4_iter0 = from_string(vec![
            "⬛⬛⬜".to_string(),
            "⬛⬜⬜".to_string(),
            "⬛⬜⬛".to_string(),
        ])
        .unwrap();
        let mut model3x3_4_iter1 = from_string(vec![
            "⬛⬜⬜".to_string(),
            "⬛⬜⬜".to_string(),
            "⬛⬜⬜".to_string(),
        ])
        .unwrap();
        model3x3_4_iter1.iter = 1;
        iterate(&mut model3x3_4_iter0);
        assert_eq!(model3x3_4_iter0, model3x3_4_iter1);

        let mut model3x3_5_iter0 = from_string(vec![
            "⬜⬜⬛".to_string(),
            "⬜⬜⬜".to_string(),
            "⬛⬜⬛".to_string(),
        ])
        .unwrap();
        let mut model3x3_5_iter1 = from_string(vec![
            "⬜⬛⬜".to_string(),
            "⬛⬛⬜".to_string(),
            "⬜⬜⬜".to_string(),
        ])
        .unwrap();
        model3x3_5_iter1.iter = 1;
        iterate(&mut model3x3_5_iter0);
        assert_eq!(model3x3_5_iter0, model3x3_5_iter1);
    }
}
