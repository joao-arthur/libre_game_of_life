use std::{
    collections::{HashMap, HashSet},
    fmt,
};

use crate::{
    cell::{self, toggle, State},
    geometry::{
        coordinate::{matrix_to_cartesian, CartesianP, MatrixP},
        poligon::rect::{get_length, Rect},
    },
    neighbor::number_of_alive_from_model,
};

use super::render::RenderSettings;

#[derive(Debug, PartialEq, Clone)]
pub struct Universe {
    pub value: HashMap<CartesianP, State>,
    pub age: u64,
}

impl From<HashMap<CartesianP, State>> for Universe {
    fn from(value: HashMap<CartesianP, State>) -> Self {
        Universe { value, ..Default::default() }
    }
}

impl Default for Universe {
    fn default() -> Self {
        Universe { value: HashMap::new(), age: 0 }
    }
}

#[derive(Debug, PartialEq)]
pub struct InvalidCharacterErr;

impl fmt::Display for InvalidCharacterErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Only \"⬜\" and \"⬛\" characters are allowed!")
    }
}

#[derive(Debug, PartialEq)]
pub struct InvalidLengthErr;

impl fmt::Display for InvalidLengthErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The length of every line and the number of lines must be equal!")
    }
}

#[derive(Debug, PartialEq)]
pub enum FromStringErr {
    InvalidCharacter(InvalidCharacterErr),
    InvalidLength(InvalidLengthErr),
}

pub fn from_string(as_str: Vec<String>) -> Result<Universe, FromStringErr> {
    if !as_str.join("").replace("⬜", "").replace("⬛", "").is_empty() {
        return Err(FromStringErr::InvalidCharacter(InvalidCharacterErr));
    }
    let len = as_str.len();
    let lines_len: HashSet<usize> = as_str.iter().map(|row| row.chars().count()).collect();
    if lines_len.len() > 1 {
        return Err(FromStringErr::InvalidLength(InvalidLengthErr));
    }
    let lines_len = as_str.get(0).unwrap().chars().count();
    if lines_len != len {
        return Err(FromStringErr::InvalidLength(InvalidLengthErr));
    }
    let rect_len = len as i64;
    let half = rect_len / 2;
    let cam = Rect { x1: -half, y1: -half, x2: -half + rect_len - 1, y2: -half + rect_len - 1 };
    let row_iter = as_str.iter().enumerate();
    let mut value = HashMap::<CartesianP, State>::new();
    for (row, row_str) in row_iter {
        let col_iter = row_str.chars().enumerate();
        for (col, col_str) in col_iter {
            if col_str == '⬜' {
                value.insert(
                    matrix_to_cartesian(&MatrixP { row: row as u64, col: col as u64 }, &cam),
                    State::Alive,
                );
            }
        }
    }
    Ok(Universe::from(value))
}

pub fn get_value(u: &Universe, p: &CartesianP) -> State {
    if u.value.get(&p).unwrap_or(&State::Dead) == &State::Alive {
        State::Alive
    } else {
        State::Dead
    }
}

pub fn iterate(u: &mut Universe) {
    let points: HashSet<CartesianP> = u
        .value
        .keys()
        .flat_map(|point| {
            [
                CartesianP::of(point.x - 1, point.y + 1),
                CartesianP::of(point.x, point.y + 1),
                CartesianP::of(point.x + 1, point.y + 1),
                CartesianP::of(point.x - 1, point.y),
                point.clone(),
                CartesianP::of(point.x + 1, point.y),
                CartesianP::of(point.x - 1, point.y - 1),
                CartesianP::of(point.x, point.y - 1),
                CartesianP::of(point.x + 1, point.y - 1),
            ]
        })
        .collect();
    let entries: HashMap<CartesianP, State> = points
        .iter()
        .filter_map(|point| {
            let s = get_value(&u, point);
            let number_of_alive_neighbors = number_of_alive_from_model(u, point);
            let new_cell = cell::iterate(s.clone(), number_of_alive_neighbors);
            match new_cell {
                State::Dead => None,
                State::Alive => Some((point.clone(), State::Alive)),
            }
        })
        .collect();
    u.age += 1;
    u.value = entries;
}

pub fn toggle_cell(u: &mut Universe, p: CartesianP) {
    let new_cell = toggle(&get_value(u, &p));
    match new_cell {
        State::Dead => {
            u.value.remove(&p);
        }
        State::Alive => {
            u.value.insert(p, new_cell);
        }
    }
}

pub fn toggle_cell_by_absolute_point(u: &mut Universe, s: &RenderSettings, p: MatrixP) {
    let dim = f64::from(s.dim);
    let len = get_length(&s.cam) as f64;
    let cell_size = dim / len;
    let row = p.row as f64 / cell_size;
    let col = p.col as f64 / cell_size;
    let matrix_point = MatrixP { row: row as u64, col: col as u64 };
    let cartesian_point = matrix_to_cartesian(&matrix_point, &s.cam);
    toggle_cell(u, cartesian_point);
}

pub fn get_camera(u: &Universe) -> Rect {
    let xx: Vec<i64> = u.value.iter().map(|v| v.0.x).collect();
    let yy: Vec<i64> = u.value.iter().map(|v| v.0.y).collect();
    let mut min_x = xx.iter().min().unwrap().to_owned();
    let mut min_y = yy.iter().min().unwrap().to_owned();
    let mut max_x = xx.iter().max().unwrap().to_owned();
    let mut max_y = yy.iter().max().unwrap().to_owned();
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
    Rect { x1: min_x - 4, y1: min_y - 4, x2: max_x + 4, y2: max_y + 4 }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cell::State;

    #[test]
    fn test_model() {
        assert_eq!(Universe::default(), Universe { value: HashMap::new(), age: 0 });
        assert_eq!(
            Universe::from(HashMap::from([
                (CartesianP::of(-1, -1), State::Alive),
                (CartesianP::of(-1, 1), State::Alive),
                (CartesianP::of(1, -1), State::Alive),
                (CartesianP::of(1, 1), State::Alive),
            ])),
            Universe {
                value: HashMap::from([
                    (CartesianP::of(-1, -1), State::Alive),
                    (CartesianP::of(-1, 1), State::Alive),
                    (CartesianP::of(1, -1), State::Alive),
                    (CartesianP::of(1, 1), State::Alive),
                ]),
                age: 0,
            }
        );
    }

    #[test]
    fn test_from_string_err() {
        assert_eq!(
            format!("{}", InvalidCharacterErr),
            "Only \"⬜\" and \"⬛\" characters are allowed!"
        );
        assert_eq!(
            format!("{}", InvalidLengthErr),
            "The length of every line and the number of lines must be equal!"
        );
        assert_eq!(
            from_string(vec![String::from("")]),
            Err(FromStringErr::InvalidLength(InvalidLengthErr)),
        );
        assert_eq!(
            from_string(vec![String::from("abcdefg")]),
            Err(FromStringErr::InvalidCharacter(InvalidCharacterErr)),
        );
        assert_eq!(
            from_string(vec![
                String::from("⬛⬛⬛⬛"),
                String::from("⬛⬛⬛⬛⬛"),
                String::from("⬛⬛⬛"),
            ]),
            Err(FromStringErr::InvalidLength(InvalidLengthErr)),
        );
        assert_eq!(
            from_string(vec![
                String::from("⬛⬛⬛⬛⬛"),
                String::from("⬛⬛⬛⬛⬛"),
                String::from("⬛⬛⬛⬛⬛"),
                String::from("⬛⬛⬛⬛⬛"),
            ]),
            Err(FromStringErr::InvalidLength(InvalidLengthErr)),
        );
    }

    #[test]
    fn test_from_string() {
        assert_eq!(from_string(vec![String::from("⬛")]).unwrap(), Universe::default());
        assert_eq!(
            from_string(vec![String::from("⬜")]).unwrap(),
            Universe::from(HashMap::from([(CartesianP::of(0, 0), State::Alive)])),
        );
        assert_eq!(
            from_string(vec![
                String::from("⬛⬛⬛⬜"),
                String::from("⬜⬛⬛⬛"),
                String::from("⬛⬛⬜⬛"),
                String::from("⬛⬛⬛⬛"),
            ])
            .unwrap(),
            Universe::from(HashMap::from([
                (CartesianP::of(-2, 0), State::Alive),
                (CartesianP::of(0, -1), State::Alive),
                (CartesianP::of(1, 1), State::Alive),
            ]))
        );
        assert_eq!(
            from_string(vec![
                String::from("⬛⬛⬛⬛⬛⬛"),
                String::from("⬛⬛⬛⬛⬛⬛"),
                String::from("⬛⬛⬜⬜⬛⬛"),
                String::from("⬛⬛⬜⬜⬛⬛"),
                String::from("⬛⬛⬛⬛⬛⬛"),
                String::from("⬛⬛⬛⬛⬛⬛"),
            ])
            .unwrap(),
            Universe::from(HashMap::from([
                (CartesianP::of(-1, -1), State::Alive),
                (CartesianP::of(-1, 0), State::Alive),
                (CartesianP::of(0, -1), State::Alive),
                (CartesianP::of(0, 0), State::Alive),
            ]),)
        );
        assert_eq!(
            from_string(vec![
                String::from("⬛⬛⬛⬛⬛⬛⬛"),
                String::from("⬛⬛⬛⬛⬛⬛⬛"),
                String::from("⬛⬛⬛⬜⬛⬛⬛"),
                String::from("⬛⬛⬛⬜⬛⬛⬛"),
                String::from("⬛⬛⬛⬜⬛⬛⬛"),
                String::from("⬛⬛⬛⬛⬛⬛⬛"),
                String::from("⬛⬛⬛⬛⬛⬛⬛"),
            ])
            .unwrap(),
            Universe::from(HashMap::from([
                (CartesianP::of(0, -1), State::Alive),
                (CartesianP::of(0, 0), State::Alive),
                (CartesianP::of(0, 1), State::Alive),
            ]),)
        );
    }

    #[test]
    fn test_toggle_model() {
        let mut u = from_string(vec![
            String::from("⬛⬛⬛⬛"),
            String::from("⬛⬛⬛⬛"),
            String::from("⬜⬜⬜⬜"),
            String::from("⬜⬜⬜⬜"),
        ])
        .unwrap();
        let state1 = from_string(vec![
            String::from("⬜⬛⬛⬛"),
            String::from("⬛⬛⬛⬛"),
            String::from("⬜⬜⬜⬜"),
            String::from("⬜⬜⬜⬜"),
        ])
        .unwrap();
        let state2 = from_string(vec![
            String::from("⬜⬛⬛⬛"),
            String::from("⬛⬜⬛⬛"),
            String::from("⬜⬜⬜⬜"),
            String::from("⬜⬜⬜⬜"),
        ])
        .unwrap();
        let state3 = from_string(vec![
            String::from("⬜⬛⬛⬛"),
            String::from("⬛⬜⬛⬛"),
            String::from("⬜⬜⬛⬜"),
            String::from("⬜⬜⬜⬜"),
        ])
        .unwrap();
        let state4 = from_string(vec![
            String::from("⬜⬛⬛⬛"),
            String::from("⬛⬜⬛⬛"),
            String::from("⬜⬜⬛⬜"),
            String::from("⬜⬜⬜⬛"),
        ])
        .unwrap();
        let state5 = from_string(vec![
            String::from("⬜⬛⬛⬛"),
            String::from("⬛⬜⬛⬛"),
            String::from("⬜⬜⬛⬜"),
            String::from("⬛⬜⬜⬛"),
        ])
        .unwrap();
        let state6 = from_string(vec![
            String::from("⬜⬛⬛⬛"),
            String::from("⬛⬜⬛⬛"),
            String::from("⬜⬛⬛⬜"),
            String::from("⬛⬜⬜⬛"),
        ])
        .unwrap();
        let state7 = from_string(vec![
            String::from("⬜⬛⬛⬛"),
            String::from("⬛⬜⬜⬛"),
            String::from("⬜⬛⬛⬜"),
            String::from("⬛⬜⬜⬛"),
        ])
        .unwrap();
        let state8 = from_string(vec![
            String::from("⬜⬛⬛⬜"),
            String::from("⬛⬜⬜⬛"),
            String::from("⬜⬛⬛⬜"),
            String::from("⬛⬜⬜⬛"),
        ])
        .unwrap();
        toggle_cell(&mut u, CartesianP::of(-2, 1));
        assert_eq!(u, state1);
        toggle_cell(&mut u, CartesianP::of(-1, 0));
        assert_eq!(u, state2);
        toggle_cell(&mut u, CartesianP::of(0, -1));
        assert_eq!(u, state3);
        toggle_cell(&mut u, CartesianP::of(1, -2));
        assert_eq!(u, state4);
        toggle_cell(&mut u, CartesianP::of(-2, -2));
        assert_eq!(u, state5);
        toggle_cell(&mut u, CartesianP::of(-1, -1));
        assert_eq!(u, state6);
        toggle_cell(&mut u, CartesianP::of(0, 0));
        assert_eq!(u, state7);
        toggle_cell(&mut u, CartesianP::of(1, 1));
        assert_eq!(u, state8);
    }

    #[test]
    fn test_toggle_cell_by_absolute_point() {
        let mut u = Universe::default();
        let state1 = from_string(vec![
            String::from("⬜⬛⬛⬛⬛⬛⬛⬛⬛⬛"),
            String::from("⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛"),
            String::from("⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛"),
            String::from("⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛"),
            String::from("⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛"),
            String::from("⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛"),
            String::from("⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛"),
            String::from("⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛"),
            String::from("⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛"),
            String::from("⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛"),
        ])
        .unwrap();
        let state2 = from_string(vec![
            String::from("⬜⬛⬛⬛⬛⬛⬛⬛⬛⬛"),
            String::from("⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛"),
            String::from("⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛"),
            String::from("⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛"),
            String::from("⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛"),
            String::from("⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛"),
            String::from("⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛"),
            String::from("⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛"),
            String::from("⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛"),
            String::from("⬜⬛⬛⬛⬛⬛⬛⬛⬛⬛"),
        ])
        .unwrap();
        let state3 = from_string(vec![
            String::from("⬜⬛⬛⬛⬛⬛⬛⬛⬛⬜"),
            String::from("⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛"),
            String::from("⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛"),
            String::from("⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛"),
            String::from("⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛"),
            String::from("⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛"),
            String::from("⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛"),
            String::from("⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛"),
            String::from("⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛"),
            String::from("⬜⬛⬛⬛⬛⬛⬛⬛⬛⬛"),
        ])
        .unwrap();
        let state4 = from_string(vec![
            String::from("⬜⬛⬛⬛⬛⬛⬛⬛⬛⬜"),
            String::from("⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛"),
            String::from("⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛"),
            String::from("⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛"),
            String::from("⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛"),
            String::from("⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛"),
            String::from("⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛"),
            String::from("⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛"),
            String::from("⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛"),
            String::from("⬜⬛⬛⬛⬛⬛⬛⬛⬛⬜"),
        ])
        .unwrap();
        let state5 = from_string(vec![
            String::from("⬜⬛⬛⬛⬛⬛⬛⬛⬛⬜"),
            String::from("⬛⬜⬛⬛⬛⬛⬛⬛⬛⬛"),
            String::from("⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛"),
            String::from("⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛"),
            String::from("⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛"),
            String::from("⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛"),
            String::from("⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛"),
            String::from("⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛"),
            String::from("⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛"),
            String::from("⬜⬛⬛⬛⬛⬛⬛⬛⬛⬜"),
        ])
        .unwrap();
        let state6 = from_string(vec![
            String::from("⬜⬛⬛⬛⬛⬛⬛⬛⬛⬜"),
            String::from("⬛⬜⬛⬛⬛⬛⬛⬛⬛⬛"),
            String::from("⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛"),
            String::from("⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛"),
            String::from("⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛"),
            String::from("⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛"),
            String::from("⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛"),
            String::from("⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛"),
            String::from("⬛⬜⬛⬛⬛⬛⬛⬛⬛⬛"),
            String::from("⬜⬛⬛⬛⬛⬛⬛⬛⬛⬜"),
        ])
        .unwrap();
        let state7 = from_string(vec![
            String::from("⬜⬛⬛⬛⬛⬛⬛⬛⬛⬜"),
            String::from("⬛⬜⬛⬛⬛⬛⬛⬛⬜⬛"),
            String::from("⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛"),
            String::from("⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛"),
            String::from("⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛"),
            String::from("⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛"),
            String::from("⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛"),
            String::from("⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛"),
            String::from("⬛⬜⬛⬛⬛⬛⬛⬛⬛⬛"),
            String::from("⬜⬛⬛⬛⬛⬛⬛⬛⬛⬜"),
        ])
        .unwrap();
        let state8 = from_string(vec![
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
        let state9 = from_string(vec![
            String::from("⬜⬛⬛⬛⬛⬛⬛⬛⬛⬜"),
            String::from("⬛⬜⬛⬛⬛⬛⬛⬛⬜⬛"),
            String::from("⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛"),
            String::from("⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛"),
            String::from("⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛"),
            String::from("⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛"),
            String::from("⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛"),
            String::from("⬛⬛⬛⬜⬛⬛⬛⬛⬛⬛"),
            String::from("⬛⬜⬛⬛⬛⬛⬛⬛⬜⬛"),
            String::from("⬜⬛⬛⬛⬛⬛⬛⬛⬛⬜"),
        ])
        .unwrap();
        let state10 = from_string(vec![
            String::from("⬜⬛⬛⬛⬛⬛⬛⬛⬛⬛"),
            String::from("⬛⬜⬛⬛⬛⬛⬛⬛⬜⬛"),
            String::from("⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛"),
            String::from("⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛"),
            String::from("⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛"),
            String::from("⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛"),
            String::from("⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛"),
            String::from("⬛⬛⬛⬜⬛⬛⬛⬛⬛⬛"),
            String::from("⬛⬜⬛⬛⬛⬛⬛⬛⬜⬛"),
            String::from("⬜⬛⬛⬛⬛⬛⬛⬛⬛⬜"),
        ])
        .unwrap();
        let state11 = from_string(vec![
            String::from("⬜⬛⬛⬛⬛⬛⬛⬛⬜⬛"),
            String::from("⬛⬜⬛⬛⬛⬛⬛⬛⬜⬛"),
            String::from("⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛"),
            String::from("⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛"),
            String::from("⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛"),
            String::from("⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛"),
            String::from("⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛"),
            String::from("⬛⬛⬛⬜⬛⬛⬛⬛⬛⬛"),
            String::from("⬛⬜⬛⬛⬛⬛⬛⬛⬜⬛"),
            String::from("⬜⬛⬛⬛⬛⬛⬛⬛⬛⬜"),
        ])
        .unwrap();
        let s1 = RenderSettings { cam: Rect::of(-5, -5, 4, 4), dim: 1000, gap: 0 };
        let s2 = RenderSettings { cam: Rect::of(-4, -4, 5, 5), dim: 1000, gap: 0 };
        let s3 = RenderSettings { cam: Rect::of(-5, -4, 4, 5), dim: 1000, gap: 0 };
        toggle_cell_by_absolute_point(&mut u, &s1, MatrixP::of(10, 10));
        assert_eq!(u, state1);
        toggle_cell_by_absolute_point(&mut u, &s1, MatrixP::of(990, 10));
        assert_eq!(u, state2);
        toggle_cell_by_absolute_point(&mut u, &s1, MatrixP::of(10, 990));
        assert_eq!(u, state3);
        toggle_cell_by_absolute_point(&mut u, &s1, MatrixP::of(990, 990));
        assert_eq!(u, state4);
        toggle_cell_by_absolute_point(&mut u, &s1, MatrixP::of(110, 110));
        assert_eq!(u, state5);
        toggle_cell_by_absolute_point(&mut u, &s1, MatrixP::of(890, 110));
        assert_eq!(u, state6);
        toggle_cell_by_absolute_point(&mut u, &s1, MatrixP::of(110, 890));
        assert_eq!(u, state7);
        toggle_cell_by_absolute_point(&mut u, &s1, MatrixP::of(890, 890));
        assert_eq!(u, state8);
        toggle_cell_by_absolute_point(&mut u, &s1, MatrixP::of(710, 350));
        assert_eq!(u, state9);
        toggle_cell_by_absolute_point(&mut u, &s2, MatrixP::of(110, 890));
        assert_eq!(u, state10);
        toggle_cell_by_absolute_point(&mut u, &s3, MatrixP::of(110, 890));
        assert_eq!(u, state11);
    }

    #[test]
    fn test_toggle_cell_by_absolute_point_float_cell_size() {
        let mut state1 = from_string(vec![
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
        let state2 = from_string(vec![
            String::from("⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛"),
            String::from("⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛"),
            String::from("⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛"),
            String::from("⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛"),
            String::from("⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛"),
            String::from("⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛"),
            String::from("⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛"),
            String::from("⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛"),
            String::from("⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛"),
            String::from("⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛"),
        ])
        .unwrap();
        let s = RenderSettings { cam: Rect::of(-5, -5, 4, 4), dim: 996, gap: 0 };
        toggle_cell_by_absolute_point(&mut state1, &s, MatrixP::of(1, 1));
        toggle_cell_by_absolute_point(&mut state1, &s, MatrixP::of(897, 99));
        toggle_cell_by_absolute_point(&mut state1, &s, MatrixP::of(99, 897));
        toggle_cell_by_absolute_point(&mut state1, &s, MatrixP::of(995, 995));
        toggle_cell_by_absolute_point(&mut state1, &s, MatrixP::of(100, 100));
        toggle_cell_by_absolute_point(&mut state1, &s, MatrixP::of(797, 199));
        toggle_cell_by_absolute_point(&mut state1, &s, MatrixP::of(199, 797));
        toggle_cell_by_absolute_point(&mut state1, &s, MatrixP::of(895, 895));
        assert_eq!(state1, state2);
    }

    #[test]
    fn test_iterate() {
        let mut model1x1iter0 = from_string(vec![String::from("⬜")]).unwrap();
        let mut model1x1iter1 = from_string(vec![String::from("⬛")]).unwrap();
        model1x1iter1.age = 1;
        iterate(&mut model1x1iter0);
        assert_eq!(model1x1iter0, model1x1iter1);

        let mut model2x2iter0 =
            from_string(vec![String::from("⬜⬜"), String::from("⬜⬜")]).unwrap();
        let mut model2x2iter1 =
            from_string(vec![String::from("⬜⬜"), String::from("⬜⬜")]).unwrap();
        model2x2iter1.age = 1;
        iterate(&mut model2x2iter0);
        assert_eq!(model2x2iter0, model2x2iter1);

        let mut model3x3_1_iter0 = from_string(vec![
            String::from("⬛⬜⬛"),
            String::from("⬛⬜⬛"),
            String::from("⬛⬜⬛"),
        ])
        .unwrap();
        let mut model3x3_1_iter1 = from_string(vec![
            String::from("⬛⬛⬛"),
            String::from("⬜⬜⬜"),
            String::from("⬛⬛⬛"),
        ])
        .unwrap();
        model3x3_1_iter1.age = 1;
        iterate(&mut model3x3_1_iter0);
        assert_eq!(model3x3_1_iter0, model3x3_1_iter1);

        let mut model3x3_2_iter0 = from_string(vec![
            String::from("⬛⬛⬛"),
            String::from("⬜⬜⬜"),
            String::from("⬛⬛⬛"),
        ])
        .unwrap();
        let mut model3x3_2_iter1 = from_string(vec![
            String::from("⬛⬜⬛"),
            String::from("⬛⬜⬛"),
            String::from("⬛⬜⬛"),
        ])
        .unwrap();
        model3x3_2_iter1.age = 1;
        iterate(&mut model3x3_2_iter0);
        assert_eq!(model3x3_2_iter0, model3x3_2_iter1);

        let mut model3x3_3_iter0 = from_string(vec![
            String::from("⬛⬛⬜"),
            String::from("⬜⬜⬜"),
            String::from("⬛⬛⬛"),
        ])
        .unwrap();
        let mut model3x3_3_iter1 = from_string(vec![
            String::from("⬛⬛⬜"),
            String::from("⬛⬜⬜"),
            String::from("⬛⬜⬛"),
        ])
        .unwrap();
        model3x3_3_iter1.age = 1;
        iterate(&mut model3x3_3_iter0);
        assert_eq!(model3x3_3_iter0, model3x3_3_iter1);

        let mut model3x3_4_iter0 = from_string(vec![
            String::from("⬛⬛⬜"),
            String::from("⬛⬜⬜"),
            String::from("⬛⬜⬛"),
        ])
        .unwrap();
        let mut model3x3_4_iter1 = from_string(vec![
            String::from("⬛⬜⬜"),
            String::from("⬛⬜⬜"),
            String::from("⬛⬜⬜"),
        ])
        .unwrap();
        model3x3_4_iter1.age = 1;
        iterate(&mut model3x3_4_iter0);
        assert_eq!(model3x3_4_iter0, model3x3_4_iter1);

        let mut model3x3_5_iter0 = from_string(vec![
            String::from("⬜⬜⬛"),
            String::from("⬜⬜⬜"),
            String::from("⬛⬜⬛"),
        ])
        .unwrap();
        let mut model3x3_5_iter1 = from_string(vec![
            String::from("⬜⬛⬜"),
            String::from("⬛⬛⬜"),
            String::from("⬜⬜⬜"),
        ])
        .unwrap();
        model3x3_5_iter1.age = 1;
        iterate(&mut model3x3_5_iter0);
        assert_eq!(model3x3_5_iter0, model3x3_5_iter1);
    }

    #[test]
    fn test_get_camera() {
        assert_eq!(
            get_camera(
                &from_string(vec![
                    String::from("⬛⬛⬛⬛⬛⬛"),
                    String::from("⬛⬛⬛⬛⬛⬛"),
                    String::from("⬛⬛⬜⬜⬛⬛"),
                    String::from("⬛⬛⬜⬜⬛⬛"),
                    String::from("⬛⬛⬛⬛⬛⬛"),
                    String::from("⬛⬛⬛⬛⬛⬛"),
                ])
                .unwrap()
            ),
            Rect::of(-5, -5, 4, 4)
        );
        assert_eq!(
            get_camera(
                &from_string(vec![
                    String::from("⬛⬛⬛⬛⬛⬛⬛"),
                    String::from("⬛⬛⬛⬛⬛⬛⬛"),
                    String::from("⬛⬛⬛⬜⬛⬛⬛"),
                    String::from("⬛⬛⬛⬜⬛⬛⬛"),
                    String::from("⬛⬛⬛⬜⬛⬛⬛"),
                    String::from("⬛⬛⬛⬛⬛⬛⬛"),
                    String::from("⬛⬛⬛⬛⬛⬛⬛"),
                ])
                .unwrap()
            ),
            Rect::of(-5, -5, 5, 5)
        );
        assert_eq!(
            get_camera(
                &from_string(vec![
                    String::from("⬛⬛⬛"),
                    String::from("⬜⬜⬜"),
                    String::from("⬛⬛⬛"),
                ])
                .unwrap()
            ),
            Rect::of(-5, -5, 5, 5)
        );
        assert_eq!(
            get_camera(
                &from_string(vec![
                    String::from("⬛⬜⬛"),
                    String::from("⬜⬜⬜"),
                    String::from("⬛⬜⬛"),
                ])
                .unwrap()
            ),
            Rect::of(-5, -5, 5, 5)
        );
        assert_eq!(
            get_camera(&from_string(vec![String::from("⬜"),]).unwrap()),
            Rect::of(-4, -4, 4, 4)
        );
        assert_eq!(
            get_camera(&Universe::from(HashMap::from([
                (CartesianP::of(2, 2), State::Alive),
                (CartesianP::of(3, 5), State::Alive),
                (CartesianP::of(5, 3), State::Alive),
            ]))),
            Rect::of(-2, -2, 9, 9)
        );
        assert_eq!(
            get_camera(&Universe::from(HashMap::from([
                (CartesianP::of(2, 2), State::Alive),
                (CartesianP::of(3, 4), State::Alive),
                (CartesianP::of(5, 3), State::Alive),
            ]))),
            Rect::of(-2, -2, 9, 9)
        );
        assert_eq!(
            get_camera(&Universe::from(HashMap::from([
                (CartesianP::of(2, 2), State::Alive),
                (CartesianP::of(3, 4), State::Alive),
                (CartesianP::of(4, 3), State::Alive),
            ]))),
            Rect::of(-2, -2, 8, 8)
        );
    }
}
