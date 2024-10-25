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
    },
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
            Universe {
                value: HashMap::new(),
                iter: 0,
            }
        );
        assert_eq!(
            from_string(vec!["⬛".to_string()]).unwrap(),
            Universe {
                value: HashMap::new(),
                iter: 0,
            }
        );
        assert_eq!(
            from_string(vec!["⬜".to_string()]).unwrap(),
            Universe {
                value: HashMap::from([(CartesianPoint::from(0, 0), State::Alive)]),
                iter: 0,
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
            }
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
}
