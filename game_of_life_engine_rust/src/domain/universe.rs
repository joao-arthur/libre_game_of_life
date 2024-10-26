use std::{
    collections::{HashMap, HashSet},
    fmt,
};

use crate::domain::{
    camera::get_center,
    cell::{self, State},
    neighbor::number_of_alive_from_model,
    plane::{
        cartesian::{from_matrix, CartesianPoint},
        matrix::MatrixPoint,
    },
};

use super::{
    camera::{center, get_length, get_subdivision_size, Rect},
    plane::cartesian::subdivide,
};

#[derive(Debug, PartialEq, Clone)]
pub struct Universe {
    pub value: HashMap<CartesianPoint, State>,
    pub iter: u64,
}

impl Universe {
    pub fn from(value: HashMap<CartesianPoint, State>) -> Self {
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
    Ok(Universe { value, iter: 0 })
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

pub fn toggle_cell_by_absolute_point(
    u: &mut Universe,
    point: CartesianPoint,
    camera: &Rect,
    size: u16,
) {
    let length = get_length(camera);
    let center = get_center(camera);
    let subdivision_size = get_subdivision_size(camera, size);
    if subdivision_size <= 0 {
        return;
    }
    let col = subdivide(point.x, subdivision_size.into());
    let row = subdivide(point.y, subdivision_size.into());
    if col > 0 && row > 0 {
        let point = from_matrix(
            MatrixPoint {
                row: row.try_into().unwrap(),
                col: col.try_into().unwrap(),
            },
            length.into(),
        );
        let cell = CartesianPoint {
            x: point.x + center.x,
            y: point.y + center.y,
        };
        toggle_cell(u, cell);
    }
}

pub fn get_camera(u: &Universe) -> Rect {
    let xx: Vec<i64> = u.value.iter().map(|v| v.0.x).collect();
    let yy: Vec<i64> = u.value.iter().map(|v| v.0.y).collect();
    let min_x = xx.iter().min().unwrap().to_owned();
    let min_y = yy.iter().min().unwrap().to_owned();
    let mut max_x = xx.iter().max().unwrap().to_owned();
    let mut max_y = yy.iter().max().unwrap().to_owned();
    let points = Rect {
        x1: min_x,
        y1: min_y,
        x2: max_x,
        y2: max_y,
    };
    let center_point = get_center(&points);
    let len_x = max_x - min_x + 1;
    let len_y = max_y - min_y + 1;
    if len_x > len_y {
        let diff = len_x - len_y;
        max_y += diff;
    }
    if len_y > len_x {
        let diff = len_y - len_x;
        max_x += diff;
    }
    let mut centered = Rect {
        x1: min_x,
        y1: min_y,
        x2: max_x,
        y2: max_y,
    };
    if len_x != len_y {
        center(&mut centered, center_point);
    }
    Rect {
        x1: centered.x1 - 2,
        y1: centered.y1 - 2,
        x2: centered.x2 + 2,
        y2: centered.y2 + 2,
    }
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
            }
        );
        assert_eq!(
            Universe::from(HashMap::from([
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
            }
        );
    }

    #[test]
    fn test_from_string() {
        assert_eq!(
            from_string(vec!["".to_string()]).unwrap(),
            Universe::default(),
        );
        assert_eq!(
            from_string(vec!["⬛".to_string()]).unwrap(),
            Universe::default()
        );
        assert_eq!(
            from_string(vec!["⬜".to_string()]).unwrap(),
            Universe::from(HashMap::from([(CartesianPoint::from(0, 0), State::Alive)])),
        );
        assert_eq!(
            from_string(vec![
                "⬛⬛⬛⬜".to_string(),
                "⬜⬛⬛⬛".to_string(),
                "⬛⬛⬜⬛".to_string(),
                "⬛⬛⬛⬛".to_string(),
            ])
            .unwrap(),
            Universe::from(HashMap::from([
                (CartesianPoint::from(1, 2), State::Alive),
                (CartesianPoint::from(-2, 1), State::Alive),
                (CartesianPoint::from(0, 0), State::Alive),
            ]))
        );
        assert_eq!(
            from_string(vec![
                "⬛⬛⬛⬛⬛⬛".to_string(),
                "⬛⬛⬛⬛⬛⬛".to_string(),
                "⬛⬛⬜⬜⬛⬛".to_string(),
                "⬛⬛⬜⬜⬛⬛".to_string(),
                "⬛⬛⬛⬛⬛⬛".to_string(),
                "⬛⬛⬛⬛⬛⬛".to_string(),
            ])
            .unwrap(),
            Universe::from(HashMap::from([
                (CartesianPoint::from(0, 0), State::Alive),
                (CartesianPoint::from(0, 1), State::Alive),
                (CartesianPoint::from(-1, 0), State::Alive),
                (CartesianPoint::from(-1, 1), State::Alive),
            ]),)
        );
        assert_eq!(
            from_string(vec![
                "⬛⬛⬛⬛⬛⬛⬛".to_string(),
                "⬛⬛⬛⬛⬛⬛⬛".to_string(),
                "⬛⬛⬛⬜⬛⬛⬛".to_string(),
                "⬛⬛⬛⬜⬛⬛⬛".to_string(),
                "⬛⬛⬛⬜⬛⬛⬛".to_string(),
                "⬛⬛⬛⬛⬛⬛⬛".to_string(),
                "⬛⬛⬛⬛⬛⬛⬛".to_string(),
            ])
            .unwrap(),
            Universe::from(HashMap::from([
                (CartesianPoint::from(0, -1), State::Alive),
                (CartesianPoint::from(0, 0), State::Alive),
                (CartesianPoint::from(0, 1), State::Alive),
            ]),)
        );
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

    #[test]
    fn test_get_camera() {
        assert_eq!(
            get_camera(
                &from_string(vec![
                    "⬛⬛⬛⬛⬛⬛".to_string(),
                    "⬛⬛⬛⬛⬛⬛".to_string(),
                    "⬛⬛⬜⬜⬛⬛".to_string(),
                    "⬛⬛⬜⬜⬛⬛".to_string(),
                    "⬛⬛⬛⬛⬛⬛".to_string(),
                    "⬛⬛⬛⬛⬛⬛".to_string(),
                ])
                .unwrap()
            ),
            Rect::from(-3, -2, 2, 3)
        );
        assert_eq!(
            get_camera(
                &from_string(vec![
                    "⬛⬛⬛⬛⬛⬛⬛".to_string(),
                    "⬛⬛⬛⬛⬛⬛⬛".to_string(),
                    "⬛⬛⬛⬜⬛⬛⬛".to_string(),
                    "⬛⬛⬛⬜⬛⬛⬛".to_string(),
                    "⬛⬛⬛⬜⬛⬛⬛".to_string(),
                    "⬛⬛⬛⬛⬛⬛⬛".to_string(),
                    "⬛⬛⬛⬛⬛⬛⬛".to_string(),
                ])
                .unwrap()
            ),
            Rect::from(-3, -3, 3, 3)
        );
        assert_eq!(
            get_camera(&Universe::from(HashMap::from([
                (CartesianPoint::from(2, 2), State::Alive),
                (CartesianPoint::from(3, 5), State::Alive),
                (CartesianPoint::from(5, 3), State::Alive),
            ]))),
            Rect::from(0, 0, 7, 7)
        );
        assert_eq!(
            get_camera(&Universe::from(HashMap::from([
                (CartesianPoint::from(2, 2), State::Alive),
                (CartesianPoint::from(3, 4), State::Alive),
                (CartesianPoint::from(5, 3), State::Alive),
            ]))),
            Rect::from(0, 0, 6, 6) //Rect::from(0, 0, 7, 7)
        );
        assert_eq!(
            get_camera(&Universe::from(HashMap::from([
                (CartesianPoint::from(2, 2), State::Alive),
                (CartesianPoint::from(3, 4), State::Alive),
                (CartesianPoint::from(4, 3), State::Alive),
            ]))),
            Rect::from(0, 0, 6, 6) //Rect::from(0, 0, 7, 7)
        );
    }
}
