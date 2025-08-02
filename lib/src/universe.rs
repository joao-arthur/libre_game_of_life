use std::{
    collections::{HashMap, HashSet},
    fmt,
};

use manfredo::{
    cartesian::rect::rect_i32::{RectI32, max_len},
    transform::matrix_to_cartesian_in_cam::point_i32::matrix_to_cartesian_in_cam,
};

use crate::{
    cell::{State, cell_iterate, cell_toggle, cell_try_of},
    neighbor::number_of_alive_from_model,
};

use super::render::RenderSettings;

pub type CartesianPoint = manfredo::cartesian::point::point_i32::PointI32;
pub type MatrixPoint = manfredo::matrix::point::point_u32::PointU32;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Universe {
    pub value: HashMap<CartesianPoint, State>,
    pub age: u64,
}

impl From<HashMap<CartesianPoint, State>> for Universe {
    fn from(value: HashMap<CartesianPoint, State>) -> Self {
        Universe { value, ..Default::default() }
    }
}

impl From<Vec<CartesianPoint>> for Universe {
    fn from(value: Vec<CartesianPoint>) -> Self {
        Universe {
            value: value.into_iter().map(|point| (point, State::Alive)).collect(),
            ..Default::default()
        }
    }
}

impl<const N: usize> From<[CartesianPoint; N]> for Universe {
    fn from(value: [CartesianPoint; N]) -> Self {
        Universe {
            value: value.into_iter().map(|point| (point, State::Alive)).collect(),
            ..Default::default()
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct InvalidCharacterErr;

impl fmt::Display for InvalidCharacterErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Must match the pattern [⬜⬛]")
    }
}

#[derive(Debug, PartialEq)]
pub struct InvalidLengthErr;

impl fmt::Display for InvalidLengthErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The length of every line and the number of lines must be equal")
    }
}

#[derive(Debug, PartialEq)]
pub enum FromStringErr {
    InvalidCharacter(InvalidCharacterErr),
    InvalidLength(InvalidLengthErr),
}

pub fn universe_try_from_string(as_str: Vec<String>) -> Result<Universe, FromStringErr> {
    if as_str.join("").find(|c| c != ' ' && cell_try_of(c).is_none()).is_some() {
        return Err(FromStringErr::InvalidCharacter(InvalidCharacterErr));
    }
    let len = as_str.len();
    let lines_len: HashSet<usize> = as_str.iter().map(|row| row.chars().count()).collect();
    if lines_len.len() > 1 {
        return Err(FromStringErr::InvalidLength(InvalidLengthErr));
    }
    let lines_len = as_str.first().unwrap().chars().count();
    if lines_len != len {
        return Err(FromStringErr::InvalidLength(InvalidLengthErr));
    }
    let rect_len = len as i32;
    let half = rect_len / 2;
    let cam = RectI32::of(-half, -half, -half + rect_len - 1, -half + rect_len - 1);
    let mut value = HashMap::<CartesianPoint, State>::new();
    for (row, row_str) in as_str.iter().enumerate() {
        for (col, col_str) in row_str.chars().enumerate() {
            if col_str == '⬜' {
                value.insert(
                    matrix_to_cartesian_in_cam(
                        &MatrixPoint { row: row as u32, col: col as u32 },
                        &cam,
                    ),
                    State::Alive,
                );
            }
        }
    }
    Ok(Universe::from(value))
}

pub fn universe_from_string(as_str: Vec<String>) -> Universe {
    universe_try_from_string(as_str).unwrap()
}

pub fn universe_try_from_str<const N: usize>(as_str: [&str; N]) -> Result<Universe, FromStringErr> {
    universe_try_from_string(as_str.into_iter().map(|s| s.into()).collect())
}

pub fn universe_from_str<const N: usize>(as_str: [&str; N]) -> Universe {
    universe_try_from_str(as_str).unwrap()
}

pub fn universe_get_value(universe: &Universe, point: &CartesianPoint) -> State {
    if universe.value.get(point).unwrap_or(&State::Dead) == &State::Alive {
        State::Alive
    } else {
        State::Dead
    }
}

pub fn universe_iterate(universe: &mut Universe) {
    let points: HashSet<CartesianPoint> = universe
        .value
        .keys()
        .flat_map(|point| {
            [
                CartesianPoint::of(point.x - 1, point.y + 1),
                CartesianPoint::of(point.x, point.y + 1),
                CartesianPoint::of(point.x + 1, point.y + 1),
                CartesianPoint::of(point.x - 1, point.y),
                point.clone(),
                CartesianPoint::of(point.x + 1, point.y),
                CartesianPoint::of(point.x - 1, point.y - 1),
                CartesianPoint::of(point.x, point.y - 1),
                CartesianPoint::of(point.x + 1, point.y - 1),
            ]
        })
        .collect();
    let entries: HashMap<CartesianPoint, State> = points
        .iter()
        .filter_map(|point| {
            let s = universe_get_value(universe, point);
            let number_of_alive_neighbors = number_of_alive_from_model(universe, point);
            let new_cell = cell_iterate(s, number_of_alive_neighbors);
            match new_cell {
                State::Dead => None,
                State::Alive => Some((point.clone(), State::Alive)),
            }
        })
        .collect();
    universe.age += 1;
    universe.value = entries;
}

pub fn universe_toggle(universe: &mut Universe, point: CartesianPoint) {
    let new_cell = cell_toggle(&universe_get_value(universe, &point));
    match new_cell {
        State::Dead => {
            universe.value.remove(&point);
        }
        State::Alive => {
            universe.value.insert(point, new_cell);
        }
    }
}

pub fn universe_toggle_by_matrix_point(
    universe: &mut Universe,
    settings: &RenderSettings,
    point: MatrixPoint,
) {
    let dim = f64::from(settings.dim);
    let len = max_len(&settings.cam) as f64;
    let cell_size = dim / len;
    let row = point.row as f64 / cell_size;
    let col = point.col as f64 / cell_size;
    let matrix_point = MatrixPoint { row: row as u32, col: col as u32 };
    let cartesian_point = matrix_to_cartesian_in_cam(&matrix_point, &settings.cam);
    universe_toggle(universe, cartesian_point);
}

pub fn universe_get_camera(universe: &Universe) -> RectI32 {
    let all_x: Vec<i32> = universe.value.keys().map(|point| point.x).collect();
    let all_y: Vec<i32> = universe.value.keys().map(|point| point.y).collect();
    let mut min_x = all_x.iter().min().unwrap().to_owned();
    let mut min_y = all_y.iter().min().unwrap().to_owned();
    let mut max_x = all_x.iter().max().unwrap().to_owned();
    let mut max_y = all_y.iter().max().unwrap().to_owned();
    let len_x = max_x - min_x + 1;
    let len_y = max_y - min_y + 1;
    if len_x > len_y {
        let diff = len_x - len_y;
        let diff_start = diff / 2;
        let diff_end = diff - diff_start;
        min_y -= diff_start;
        max_y += diff_end;
    }
    if len_y > len_x {
        let diff = len_y - len_x;
        let diff_start = diff / 2;
        let diff_end = diff - diff_start;
        min_x -= diff_start;
        max_x += diff_end;
    }
    RectI32::of(min_x - 4, min_y - 4, max_x + 4, max_y + 4)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use manfredo::cartesian::rect::rect_i32::RectI32;

    use crate::{cell::State, render::RenderSettings};

    use super::{
        CartesianPoint, FromStringErr, InvalidCharacterErr, InvalidLengthErr, MatrixPoint,
        Universe, universe_from_str, universe_from_string, universe_get_camera, universe_iterate,
        universe_toggle, universe_toggle_by_matrix_point, universe_try_from_str,
        universe_try_from_string,
    };

    #[test]
    fn invalid_character_err() {
        assert_eq!(InvalidCharacterErr.to_string(), "Must match the pattern [⬜⬛]");
    }

    #[test]
    fn invalid_length_err() {
        assert_eq!(
            InvalidLengthErr.to_string(),
            "The length of every line and the number of lines must be equal"
        );
    }

    #[test]
    fn test_model() {
        assert_eq!(Universe::default(), Universe { value: HashMap::new(), age: 0 });
        assert_eq!(
            Universe::from(HashMap::from([
                (CartesianPoint::of(-1, -1), State::Alive),
                (CartesianPoint::of(-1, 1), State::Alive),
                (CartesianPoint::of(1, -1), State::Dead),
                (CartesianPoint::of(1, 1), State::Alive),
            ])),
            Universe {
                value: HashMap::from([
                    (CartesianPoint::of(-1, -1), State::Alive),
                    (CartesianPoint::of(-1, 1), State::Alive),
                    (CartesianPoint::of(1, -1), State::Dead),
                    (CartesianPoint::of(1, 1), State::Alive),
                ]),
                age: 0,
            }
        );
        assert_eq!(
            Universe::from(vec![
                CartesianPoint::of(-1, -1),
                CartesianPoint::of(-1, 1),
                CartesianPoint::of(1, -1),
                CartesianPoint::of(1, 1),
            ]),
            Universe {
                value: HashMap::from([
                    (CartesianPoint::of(-1, -1), State::Alive),
                    (CartesianPoint::of(-1, 1), State::Alive),
                    (CartesianPoint::of(1, -1), State::Alive),
                    (CartesianPoint::of(1, 1), State::Alive),
                ]),
                age: 0,
            }
        );
        assert_eq!(
            Universe::from([
                CartesianPoint::of(-1, -1),
                CartesianPoint::of(-1, 1),
                CartesianPoint::of(1, -1),
                CartesianPoint::of(1, 1),
            ]),
            Universe {
                value: HashMap::from([
                    (CartesianPoint::of(-1, -1), State::Alive),
                    (CartesianPoint::of(-1, 1), State::Alive),
                    (CartesianPoint::of(1, -1), State::Alive),
                    (CartesianPoint::of(1, 1), State::Alive),
                ]),
                age: 0,
            }
        );
    }

    #[test]
    fn test_universe_from_string_err() {
        assert_eq!(
            universe_try_from_string(vec!["".into()]),
            Err(FromStringErr::InvalidLength(InvalidLengthErr)),
        );
        assert_eq!(
            universe_try_from_string(vec!["abcdefg".into()]),
            Err(FromStringErr::InvalidCharacter(InvalidCharacterErr)),
        );
        assert_eq!(
            universe_try_from_string(vec!["⬛⬛⬛⬛".into(), "⬛⬛⬛⬛⬛".into(), "⬛⬛⬛".into()]),
            Err(FromStringErr::InvalidLength(InvalidLengthErr)),
        );
        assert_eq!(
            universe_try_from_string(vec![
                "⬛⬛⬛⬛⬛".into(),
                "⬛⬛⬛⬛⬛".into(),
                "⬛⬛⬛⬛⬛".into(),
                "⬛⬛⬛⬛⬛".into(),
            ]),
            Err(FromStringErr::InvalidLength(InvalidLengthErr)),
        );
    }

    #[test]
    fn test_universe_try_from_string() {
        assert_eq!(universe_try_from_string(vec!["⬛".into()]), Ok(Universe::default()));
        assert_eq!(
            universe_try_from_string(vec!["⬜".into()]),
            Ok(Universe::from([CartesianPoint::of(0, 0)])),
        );
        assert_eq!(
            universe_try_from_string(vec![
                "⬛⬛⬛⬜".into(),
                "⬜⬛⬛⬛".into(),
                "⬛⬛⬜⬛".into(),
                "⬛⬛⬛⬛".into(),
            ]),
            Ok(Universe::from([
                CartesianPoint::of(-2, 0),
                CartesianPoint::of(0, -1),
                CartesianPoint::of(1, 1),
            ]))
        );
        assert_eq!(
            universe_try_from_string(vec![
                "⬛⬛⬛⬛⬛⬛".into(),
                "⬛⬛⬛⬛⬛⬛".into(),
                "⬛⬛⬜⬜⬛⬛".into(),
                "⬛⬛⬜⬜⬛⬛".into(),
                "⬛⬛⬛⬛⬛⬛".into(),
                "⬛⬛⬛⬛⬛⬛".into(),
            ]),
            Ok(Universe::from([
                CartesianPoint::of(-1, -1),
                CartesianPoint::of(-1, 0),
                CartesianPoint::of(0, -1),
                CartesianPoint::of(0, 0),
            ]))
        );
        assert_eq!(
            universe_try_from_string(vec![
                "⬛⬛⬛⬛⬛⬛⬛".into(),
                "⬛⬛⬛⬛⬛⬛⬛".into(),
                "⬛⬛⬛⬜⬛⬛⬛".into(),
                "⬛⬛⬛⬜⬛⬛⬛".into(),
                "⬛⬛⬛⬜⬛⬛⬛".into(),
                "⬛⬛⬛⬛⬛⬛⬛".into(),
                "⬛⬛⬛⬛⬛⬛⬛".into(),
            ]),
            Ok(Universe::from([
                CartesianPoint::of(0, -1),
                CartesianPoint::of(0, 0),
                CartesianPoint::of(0, 1),
            ]))
        );
    }

    #[test]
    fn test_universe_try_from_str() {
        assert_eq!(
            universe_try_from_str([
                "⬛⬛⬛⬛⬛⬛⬛",
                "⬛⬛⬛⬛⬛⬛⬛",
                "⬛⬛⬛⬜⬛⬛⬛",
                "⬛⬛⬛⬜⬛⬛⬛",
                "⬛⬛⬛⬜⬛⬛⬛",
                "⬛⬛⬛⬛⬛⬛⬛",
                "⬛⬛⬛⬛⬛⬛⬛",
            ]),
            Ok(Universe::from([
                CartesianPoint::of(0, -1),
                CartesianPoint::of(0, 0),
                CartesianPoint::of(0, 1),
            ]))
        );
    }

    #[test]
    fn test_universe_from_string() {
        assert_eq!(
            universe_from_string(vec![
                "⬛⬛⬛⬛⬛⬛⬛".into(),
                "⬛⬛⬛⬛⬛⬛⬛".into(),
                "⬛⬛⬛⬜⬛⬛⬛".into(),
                "⬛⬛⬛⬜⬛⬛⬛".into(),
                "⬛⬛⬛⬜⬛⬛⬛".into(),
                "⬛⬛⬛⬛⬛⬛⬛".into(),
                "⬛⬛⬛⬛⬛⬛⬛".into(),
            ]),
            Universe::from([
                CartesianPoint::of(0, -1),
                CartesianPoint::of(0, 0),
                CartesianPoint::of(0, 1),
            ])
        );
    }

    #[test]
    fn test_universe_from_str() {
        assert_eq!(
            universe_from_str([
                "⬛⬛⬛⬛⬛⬛⬛",
                "⬛⬛⬛⬛⬛⬛⬛",
                "⬛⬛⬛⬜⬛⬛⬛",
                "⬛⬛⬛⬜⬛⬛⬛",
                "⬛⬛⬛⬜⬛⬛⬛",
                "⬛⬛⬛⬛⬛⬛⬛",
                "⬛⬛⬛⬛⬛⬛⬛",
            ]),
            Universe::from([
                CartesianPoint::of(0, -1),
                CartesianPoint::of(0, 0),
                CartesianPoint::of(0, 1),
            ])
        );
    }

    #[test]
    fn test_universe_toggle() {
        let mut universe = universe_from_str(["⬛⬛⬛⬛", "⬛⬛⬛⬛", "⬜⬜⬜⬜", "⬜⬜⬜⬜"]);
        universe_toggle(&mut universe, CartesianPoint::of(-2, 1));
        assert_eq!(universe, universe_from_str(["⬜⬛⬛⬛", "⬛⬛⬛⬛", "⬜⬜⬜⬜", "⬜⬜⬜⬜"]));
        universe_toggle(&mut universe, CartesianPoint::of(-1, 0));
        assert_eq!(universe, universe_from_str(["⬜⬛⬛⬛", "⬛⬜⬛⬛", "⬜⬜⬜⬜", "⬜⬜⬜⬜"]));
        universe_toggle(&mut universe, CartesianPoint::of(0, -1));
        assert_eq!(universe, universe_from_str(["⬜⬛⬛⬛", "⬛⬜⬛⬛", "⬜⬜⬛⬜", "⬜⬜⬜⬜"]));
        universe_toggle(&mut universe, CartesianPoint::of(1, -2));
        assert_eq!(universe, universe_from_str(["⬜⬛⬛⬛", "⬛⬜⬛⬛", "⬜⬜⬛⬜", "⬜⬜⬜⬛"]));
        universe_toggle(&mut universe, CartesianPoint::of(-2, -2));
        assert_eq!(universe, universe_from_str(["⬜⬛⬛⬛", "⬛⬜⬛⬛", "⬜⬜⬛⬜", "⬛⬜⬜⬛"]));
        universe_toggle(&mut universe, CartesianPoint::of(-1, -1));
        assert_eq!(universe, universe_from_str(["⬜⬛⬛⬛", "⬛⬜⬛⬛", "⬜⬛⬛⬜", "⬛⬜⬜⬛"]));
        universe_toggle(&mut universe, CartesianPoint::of(0, 0));
        assert_eq!(universe, universe_from_str(["⬜⬛⬛⬛", "⬛⬜⬜⬛", "⬜⬛⬛⬜", "⬛⬜⬜⬛"]));
        universe_toggle(&mut universe, CartesianPoint::of(1, 1));
        assert_eq!(universe, universe_from_str(["⬜⬛⬛⬜", "⬛⬜⬜⬛", "⬜⬛⬛⬜", "⬛⬜⬜⬛"]));
    }

    #[test]
    fn test_universe_toggle_by_matrix_point() {
        let mut universe = Universe::default();
        let state_1 = universe_from_str([
            "⬜⬛⬛⬛⬛⬛⬛⬛⬛⬛",
            "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
            "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
            "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
            "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
            "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
            "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
            "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
            "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
            "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
        ]);
        let state_2 = universe_from_str([
            "⬜⬛⬛⬛⬛⬛⬛⬛⬛⬛",
            "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
            "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
            "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
            "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
            "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
            "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
            "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
            "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
            "⬜⬛⬛⬛⬛⬛⬛⬛⬛⬛",
        ]);
        let state_3 = universe_from_str([
            "⬜⬛⬛⬛⬛⬛⬛⬛⬛⬜",
            "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
            "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
            "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
            "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
            "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
            "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
            "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
            "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
            "⬜⬛⬛⬛⬛⬛⬛⬛⬛⬛",
        ]);
        let state_4 = universe_from_str([
            "⬜⬛⬛⬛⬛⬛⬛⬛⬛⬜",
            "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
            "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
            "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
            "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
            "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
            "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
            "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
            "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
            "⬜⬛⬛⬛⬛⬛⬛⬛⬛⬜",
        ]);
        let state_5 = universe_from_str([
            "⬜⬛⬛⬛⬛⬛⬛⬛⬛⬜",
            "⬛⬜⬛⬛⬛⬛⬛⬛⬛⬛",
            "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
            "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
            "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
            "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
            "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
            "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
            "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
            "⬜⬛⬛⬛⬛⬛⬛⬛⬛⬜",
        ]);
        let state_6 = universe_from_str([
            "⬜⬛⬛⬛⬛⬛⬛⬛⬛⬜",
            "⬛⬜⬛⬛⬛⬛⬛⬛⬛⬛",
            "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
            "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
            "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
            "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
            "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
            "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
            "⬛⬜⬛⬛⬛⬛⬛⬛⬛⬛",
            "⬜⬛⬛⬛⬛⬛⬛⬛⬛⬜",
        ]);
        let state_7 = universe_from_str([
            "⬜⬛⬛⬛⬛⬛⬛⬛⬛⬜",
            "⬛⬜⬛⬛⬛⬛⬛⬛⬜⬛",
            "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
            "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
            "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
            "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
            "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
            "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
            "⬛⬜⬛⬛⬛⬛⬛⬛⬛⬛",
            "⬜⬛⬛⬛⬛⬛⬛⬛⬛⬜",
        ]);
        let state_8 = universe_from_str([
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
        ]);
        let state_9 = universe_from_str([
            "⬜⬛⬛⬛⬛⬛⬛⬛⬛⬜",
            "⬛⬜⬛⬛⬛⬛⬛⬛⬜⬛",
            "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
            "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
            "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
            "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
            "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
            "⬛⬛⬛⬜⬛⬛⬛⬛⬛⬛",
            "⬛⬜⬛⬛⬛⬛⬛⬛⬜⬛",
            "⬜⬛⬛⬛⬛⬛⬛⬛⬛⬜",
        ]);
        let state_10 = universe_from_str([
            "⬜⬛⬛⬛⬛⬛⬛⬛⬛⬛",
            "⬛⬜⬛⬛⬛⬛⬛⬛⬜⬛",
            "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
            "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
            "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
            "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
            "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
            "⬛⬛⬛⬜⬛⬛⬛⬛⬛⬛",
            "⬛⬜⬛⬛⬛⬛⬛⬛⬜⬛",
            "⬜⬛⬛⬛⬛⬛⬛⬛⬛⬜",
        ]);
        let state_11 = universe_from_str([
            "⬜⬛⬛⬛⬛⬛⬛⬛⬜⬛",
            "⬛⬜⬛⬛⬛⬛⬛⬛⬜⬛",
            "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
            "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
            "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
            "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
            "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
            "⬛⬛⬛⬜⬛⬛⬛⬛⬛⬛",
            "⬛⬜⬛⬛⬛⬛⬛⬛⬜⬛",
            "⬜⬛⬛⬛⬛⬛⬛⬛⬛⬜",
        ]);
        let s1 = RenderSettings { cam: RectI32::of(-5, -5, 4, 4), dim: 1000, gap: 0 };
        let s2 = RenderSettings { cam: RectI32::of(-4, -4, 5, 5), dim: 1000, gap: 0 };
        let s3 = RenderSettings { cam: RectI32::of(-5, -4, 4, 5), dim: 1000, gap: 0 };
        universe_toggle_by_matrix_point(&mut universe, &s1, MatrixPoint::of(10, 10));
        assert_eq!(universe, state_1);
        universe_toggle_by_matrix_point(&mut universe, &s1, MatrixPoint::of(990, 10));
        assert_eq!(universe, state_2);
        universe_toggle_by_matrix_point(&mut universe, &s1, MatrixPoint::of(10, 990));
        assert_eq!(universe, state_3);
        universe_toggle_by_matrix_point(&mut universe, &s1, MatrixPoint::of(990, 990));
        assert_eq!(universe, state_4);
        universe_toggle_by_matrix_point(&mut universe, &s1, MatrixPoint::of(110, 110));
        assert_eq!(universe, state_5);
        universe_toggle_by_matrix_point(&mut universe, &s1, MatrixPoint::of(890, 110));
        assert_eq!(universe, state_6);
        universe_toggle_by_matrix_point(&mut universe, &s1, MatrixPoint::of(110, 890));
        assert_eq!(universe, state_7);
        universe_toggle_by_matrix_point(&mut universe, &s1, MatrixPoint::of(890, 890));
        assert_eq!(universe, state_8);
        universe_toggle_by_matrix_point(&mut universe, &s1, MatrixPoint::of(710, 350));
        assert_eq!(universe, state_9);
        universe_toggle_by_matrix_point(&mut universe, &s2, MatrixPoint::of(110, 890));
        assert_eq!(universe, state_10);
        universe_toggle_by_matrix_point(&mut universe, &s3, MatrixPoint::of(110, 890));
        assert_eq!(universe, state_11);
    }

    #[test]
    fn test_universe_toggle_by_matrix_point_float_cell_size() {
        let mut state_1 = universe_from_str([
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
        ]);
        let state_2 = universe_from_str([
            "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
            "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
            "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
            "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
            "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
            "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
            "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
            "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
            "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
            "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
        ]);
        let s = RenderSettings { cam: RectI32::of(-5, -5, 4, 4), dim: 996, gap: 0 };
        universe_toggle_by_matrix_point(&mut state_1, &s, MatrixPoint::of(1, 1));
        universe_toggle_by_matrix_point(&mut state_1, &s, MatrixPoint::of(897, 99));
        universe_toggle_by_matrix_point(&mut state_1, &s, MatrixPoint::of(99, 897));
        universe_toggle_by_matrix_point(&mut state_1, &s, MatrixPoint::of(995, 995));
        universe_toggle_by_matrix_point(&mut state_1, &s, MatrixPoint::of(100, 100));
        universe_toggle_by_matrix_point(&mut state_1, &s, MatrixPoint::of(797, 199));
        universe_toggle_by_matrix_point(&mut state_1, &s, MatrixPoint::of(199, 797));
        universe_toggle_by_matrix_point(&mut state_1, &s, MatrixPoint::of(895, 895));
        assert_eq!(state_1, state_2);
    }

    #[test]
    fn universe_iterate_cell() {
        let mut universe = universe_from_str(["⬜"]);
        let mut state_1 = universe_from_str(["⬛"]);
        state_1.age = 1;
        universe_iterate(&mut universe);
        assert_eq!(universe, state_1);
    }

    #[test]
    fn universe_iterate_empty() {
        let mut universe = universe_from_str(["⬜⬜", "⬜⬜"]);
        let mut state_1 = universe_from_str(["⬜⬜", "⬜⬜"]);
        state_1.age = 1;
        universe_iterate(&mut universe);
        assert_eq!(universe, state_1);
    }

    #[test]
    fn universe_iterate_blinker() {
        let mut universe = universe_from_str([
            "⬛⬜⬛", //
            "⬛⬜⬛", //
            "⬛⬜⬛", //
        ]);
        let mut state_1 = universe_from_str([
            "⬛⬛⬛", //
            "⬜⬜⬜", //
            "⬛⬛⬛", //
        ]);
        state_1.age = 1;
        let mut state_2 = universe_from_str([
            "⬛⬜⬛", //
            "⬛⬜⬛", //
            "⬛⬜⬛", //
        ]);
        state_2.age = 2;
        universe_iterate(&mut universe);
        assert_eq!(universe, state_1);
        universe_iterate(&mut universe);
        assert_eq!(universe, state_2);
    }

    #[test]
    fn test_universe_l_shape() {
        let mut universe = universe_from_str([
            "⬛⬛⬜", //
            "⬜⬜⬜", //
            "⬛⬛⬛", //
        ]);
        let mut state_1 = universe_from_str([
            "⬛⬛⬜", //
            "⬛⬜⬜", //
            "⬛⬜⬛", //
        ]);
        state_1.age = 1;
        let mut state_2 = universe_from_str([
            "⬛⬜⬜", //
            "⬛⬜⬜", //
            "⬛⬜⬜", //
        ]);
        state_2.age = 2;
        universe_iterate(&mut universe);
        assert_eq!(universe, state_1);
        universe_iterate(&mut universe);
        assert_eq!(universe, state_2);
    }

    #[test]
    fn test_universe_iterate() {
        let mut model3x3_5_iter0 = universe_from_str([
            "⬜⬜⬛", //
            "⬜⬜⬜", //
            "⬛⬜⬛", //
        ]);
        let mut model3x3_5_iter1 = universe_from_str([
            "⬜⬛⬜", //
            "⬛⬛⬜", //
            "⬜⬜⬜", //
        ]);
        model3x3_5_iter1.age = 1;
        universe_iterate(&mut model3x3_5_iter0);
        assert_eq!(model3x3_5_iter0, model3x3_5_iter1);
    }

    #[test]
    fn test_universe_get_camera() {
        let preset_block = universe_from_str([
            "⬛⬛⬛⬛⬛⬛",
            "⬛⬛⬛⬛⬛⬛",
            "⬛⬛⬜⬜⬛⬛",
            "⬛⬛⬜⬜⬛⬛",
            "⬛⬛⬛⬛⬛⬛",
            "⬛⬛⬛⬛⬛⬛",
        ]);
        let preset_blinker_1 = universe_from_str([
            "⬛⬛⬛⬛⬛",
            "⬛⬛⬜⬛⬛",
            "⬛⬛⬜⬛⬛",
            "⬛⬛⬜⬛⬛",
            "⬛⬛⬛⬛⬛",
        ]);
        let preset_blinker_2 = universe_from_str([
            "⬛⬛⬛⬛⬛",
            "⬛⬛⬛⬛⬛",
            "⬛⬜⬜⬜⬛",
            "⬛⬛⬛⬛⬛",
            "⬛⬛⬛⬛⬛",
        ]);
        let preset_cross = universe_from_str([
            "⬛⬛⬛⬛⬛",
            "⬛⬛⬜⬛⬛",
            "⬛⬜⬜⬜⬛",
            "⬛⬛⬜⬛⬛",
            "⬛⬛⬛⬛⬛",
        ]);
        let preset_cell = universe_from_str(["⬜"]);
        assert_eq!(universe_get_camera(&preset_block), RectI32::of(-5, -5, 4, 4));
        assert_eq!(universe_get_camera(&preset_blinker_1), RectI32::of(-5, -5, 5, 5));
        assert_eq!(universe_get_camera(&preset_blinker_2), RectI32::of(-5, -5, 5, 5));
        assert_eq!(universe_get_camera(&preset_cross), RectI32::of(-5, -5, 5, 5));
        assert_eq!(universe_get_camera(&preset_cell), RectI32::of(-4, -4, 4, 4));
        assert_eq!(
            universe_get_camera(&Universe::from([
                CartesianPoint::of(2, 2),
                CartesianPoint::of(3, 5),
                CartesianPoint::of(5, 3),
            ])),
            RectI32::of(-2, -2, 9, 9)
        );
        assert_eq!(
            universe_get_camera(&Universe::from([
                CartesianPoint::of(2, 2),
                CartesianPoint::of(3, 4),
                CartesianPoint::of(5, 3),
            ])),
            RectI32::of(-2, -2, 9, 9)
        );
        assert_eq!(
            universe_get_camera(&Universe::from([
                CartesianPoint::of(2, 2),
                CartesianPoint::of(3, 4),
                CartesianPoint::of(4, 3),
            ])),
            RectI32::of(-2, -2, 8, 8)
        );
    }
}
