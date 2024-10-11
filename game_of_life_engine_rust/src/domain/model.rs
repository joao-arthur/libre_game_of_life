use std::collections::{HashMap, HashSet};

use crate::domain::{
    cartesian_plane::{index_to_point, ArrPos, Point},
    cell::{self, toggle, State},
    neighbor::number_of_alive_from_model,
};

#[derive(Debug, PartialEq, Clone)]
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
    pub value: HashMap<Point, State>,
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

    pub fn from_value(value: HashMap<Point, State>) -> Self {
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
    let mut value = HashMap::<Point, State>::new();
    let len = model_as_str.len() as i64;
    let row_iter = model_as_str.iter().enumerate();
    for (row, row_str) in row_iter {
        let col_iter = row_str.chars().enumerate();
        for (col, col_str) in col_iter {
            if col_str == '⬜' {
                value.insert(
                    index_to_point(
                        ArrPos {
                            col: col as i64,
                            row: row as i64,
                        },
                        len,
                    ),
                    State::ALIVE,
                );
            }
        }
    }
    Model {
        value,
        iter: 0,
        pos: Rect {
            x1: -10,
            y1: -10,
            x2: 10,
            y2: 10,
        },
    }
}

pub fn get_length(model: &Model) -> i64 {
    model.pos.x2 - model.pos.x1 + 1
}

pub fn get_cell_size(model: &Model, total_size: i64) -> i64 {
    total_size / get_length(model)
}

pub fn get_middle_point(model: &Model) -> Point {
    Point {
        x: (model.pos.x1 + model.pos.x2) / 2,
        y: (model.pos.y1 + model.pos.y2) / 2,
    }
}

pub fn get_middle_cell(model: &Model, total_size: i64) -> Point {
    let cell_size = get_cell_size(model, total_size);
    let middle = get_middle_point(model);
    Point {
        x: middle.x * cell_size,
        y: middle.y * cell_size,
    }
}

pub fn get_value(model: &Model, point: Point) -> State {
    if model.value.get(&point).unwrap_or(&State::DEAD) == &State::ALIVE {
        State::ALIVE
    } else {
        State::DEAD
    }
}

// TODO: optimize by mutating the current hashmap instead of creating a new one
pub fn iterate(model: &mut Model) {
    let points: HashSet<Point> = model
        .value
        .keys()
        .flat_map(|p| {
            [
                Point::from(p.x - 1, p.y + 1),
                Point::from(p.x, p.y + 1),
                Point::from(p.x + 1, p.y + 1),
                Point::from(p.x - 1, p.y),
                p.clone(),
                Point::from(p.x + 1, p.y),
                Point::from(p.x - 1, p.y - 1),
                Point::from(p.x, p.y - 1),
                Point::from(p.x + 1, p.y - 1),
            ]
        })
        .collect();
    let entries: HashMap<Point, State> = points
        .iter()
        .filter_map(|p| {
            let state = model.value.get(p).unwrap_or(&State::DEAD);
            let number_of_alive_neighbors = number_of_alive_from_model(model, p.clone());
            let new_cell = cell::iterate(state.clone(), number_of_alive_neighbors);
            match new_cell {
                State::DEAD => None,
                State::ALIVE => Some((p.clone(), State::ALIVE)),
            }
        })
        .collect();
    model.iter += 1;
    model.value = entries;
}

pub fn move_in_plane(model: &mut Model, delta: Point) {
    model.pos = Rect::from(
        model.pos.x1 + delta.x,
        model.pos.y1 + delta.y,
        model.pos.x2 + delta.x,
        model.pos.y2 + delta.y,
    )
}

pub fn toggle_cell(model: &mut Model, point: Point) {
    let new_cell = toggle(model.value.get(&point).unwrap_or(&State::DEAD));
    match new_cell {
        State::DEAD => {
            model.value.remove(&point);
        }
        State::ALIVE => {
            model.value.insert(point, new_cell);
        }
    }
}

pub fn zoom(model: &mut Model, new_size: i64) {
    let half_new_size = new_size as f64 / 2 as f64;
    let half_x = (model.pos.x1 + model.pos.x2) as f64 / 2 as f64;
    let half_y = (model.pos.y1 + model.pos.y2) as f64 / 2 as f64;
    let x1 = (half_x - half_new_size).ceil() as i64;
    let y1 = (half_y - half_new_size).ceil() as i64;
    let x2 = x1 + new_size as i64 - 1;
    let y2 = y1 + new_size as i64 - 1;
    model.pos = Rect { x1, y1, x2, y2 };
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
                (Point::from(-1, -1), State::ALIVE),
                (Point::from(-1, 1), State::ALIVE),
                (Point::from(1, -1), State::ALIVE),
                (Point::from(1, 1), State::ALIVE),
            ])),
            Model {
                value: HashMap::from([
                    (Point::from(-1, -1), State::ALIVE),
                    (Point::from(-1, 1), State::ALIVE),
                    (Point::from(1, -1), State::ALIVE),
                    (Point::from(1, 1), State::ALIVE),
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
                value: HashMap::from([(Point::from(0, 0), State::ALIVE)]),
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
                    (Point::from(1, 2), State::ALIVE),
                    (Point::from(-2, 1), State::ALIVE),
                    (Point::from(0, 0), State::ALIVE),
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
        let model = Model::from_pos(Rect::from(1, 1, 10, 10));
        assert_eq!(get_cell_size(&model, 100), 10);
        assert_eq!(get_cell_size(&model, 90), 9);
        assert_eq!(get_cell_size(&model, 50), 5);
        assert_eq!(get_cell_size(&model, 10), 1);
    }

    #[test]
    fn test_get_middle_point() {
        assert_eq!(
            get_middle_point(&Model::from_pos(Rect::from(-10, -10, 10, 10))),
            Point::from(0, 0)
        );
        // assert_eq!(
        //     get_middle_point(&Model::from_pos(Rect::from(1, 1, 10, 10))),
        //     Point::from(5.5, 5.5)
        // );
        // assert_eq!(
        //     get_middle_point(&Model::from_pos(Rect::from(4, 4, 5, 5))),
        //     Point::from(4.5, 4.5)
        // );
        assert_eq!(
            get_middle_point(&Model::from_pos(Rect::from(5, 5, 5, 5))),
            Point::from(5, 5)
        );
    }

    #[test]
    fn test_get_middle_cell() {
        assert_eq!(
            get_middle_cell(&Model::from_pos(Rect::from(-10, -10, 10, 10)), 100,),
            Point::from(0, 0)
        );
        // assert_eq!(
        //     get_middle_cell(&Model::from_pos(Rect::from(1, 1, 10, 10)), 50),
        //     Point::from(27.5, 27.5)
        // );
        // assert_eq!(
        //     get_middle_cell(&Model::from_pos(Rect::from(4, 4, 5, 5)), 10),
        //     Point::from(22.5, 22.5)
        // );
        assert_eq!(
            get_middle_cell(&Model::from_pos(Rect::from(5, 5, 5, 5)), 1),
            Point::from(5, 5)
        );
    }

    #[test]
    fn test_move_in_plane() {
        let mut model1 = Model::from_pos(Rect::from(-10, -10, 10, 10));
        let mut model2 = Model::from_pos(Rect::from(-10, -10, 10, 10));
        let mut model3 = Model::from_pos(Rect::from(-10, -10, 10, 10));
        let mut model4 = Model::from_pos(Rect::from(-10, -10, 10, 10));
        let mut model5 = Model::from_pos(Rect::from(-10, -10, 10, 10));
        let mut model6 = Model::from_pos(Rect::from(-10, -10, 10, 10));
        let mut model7 = Model::from_pos(Rect::from(-10, -10, 10, 10));
        let mut model8 = Model::from_pos(Rect::from(-10, -10, 10, 10));

        move_in_plane(&mut model1, Point::from(1, 0));
        move_in_plane(&mut model2, Point::from(-1, 0));
        move_in_plane(&mut model3, Point::from(0, 1));
        move_in_plane(&mut model4, Point::from(0, -1));
        move_in_plane(&mut model5, Point::from(11, 0));
        move_in_plane(&mut model6, Point::from(-11, 0));
        move_in_plane(&mut model7, Point::from(0, 11));
        move_in_plane(&mut model8, Point::from(0, -11));

        assert_eq!(model1.pos, Rect::from(-9, -10, 11, 10));
        assert_eq!(model2.pos, Rect::from(-11, -10, 9, 10));
        assert_eq!(model3.pos, Rect::from(-10, -9, 10, 11));
        assert_eq!(model4.pos, Rect::from(-10, -11, 10, 9));
        assert_eq!(model5.pos, Rect::from(1, -10, 21, 10));
        assert_eq!(model6.pos, Rect::from(-21, -10, -1, 10));
        assert_eq!(model7.pos, Rect::from(-10, 1, 10, 21));
        assert_eq!(model8.pos, Rect::from(-10, -21, 10, -1));
    }

    #[test]
    fn test_zoom_odd_size() {
        let mut model1 = Model::from_pos(Rect::from(-10, -10, 10, 10));
        let mut model2 = Model::from_pos(Rect::from(-10, -10, 10, 10));
        let mut model3 = Model::from_pos(Rect::from(-10, -10, 10, 10));
        let mut model4 = Model::from_pos(Rect::from(-10, -10, 10, 10));
        let mut model5 = Model::from_pos(Rect::from(-10, -10, 10, 10));
        let mut model6 = Model::from_pos(Rect::from(-10, -10, 10, 10));

        zoom(&mut model1, 1);
        zoom(&mut model2, 2);
        zoom(&mut model3, 3);
        zoom(&mut model4, 19);
        zoom(&mut model5, 21);
        zoom(&mut model6, 23);

        assert_eq!(model1.pos, Rect::from(0, 0, 0, 0));
        assert_eq!(model2.pos, Rect::from(-1, -1, 0, 0));
        assert_eq!(model3.pos, Rect::from(-1, -1, 1, 1));
        assert_eq!(model4.pos, Rect::from(-9, -9, 9, 9));
        assert_eq!(model5.pos, Rect::from(-10, -10, 10, 10));
        assert_eq!(model6.pos, Rect::from(-11, -11, 11, 11));
    }

    #[test]
    fn test_zoom_even_size() {
        let mut model1 = Model::from_pos(Rect::from(10, 10, 19, 19));
        let mut model2 = Model::from_pos(Rect::from(10, 10, 19, 19));
        let mut model3 = Model::from_pos(Rect::from(10, 10, 19, 19));
        let mut model4 = Model::from_pos(Rect::from(10, 10, 19, 19));
        let mut model5 = Model::from_pos(Rect::from(10, 10, 19, 19));
        let mut model6 = Model::from_pos(Rect::from(10, 10, 19, 19));

        zoom(&mut model1, 1);
        zoom(&mut model2, 2);
        zoom(&mut model3, 3);
        zoom(&mut model4, 8);
        zoom(&mut model5, 10);
        zoom(&mut model6, 12);

        assert_eq!(model1.pos, Rect::from(14, 14, 14, 14));
        assert_eq!(model2.pos, Rect::from(14, 14, 15, 15));
        assert_eq!(model3.pos, Rect::from(13, 13, 15, 15));
        assert_eq!(model4.pos, Rect::from(11, 11, 18, 18));
        assert_eq!(model5.pos, Rect::from(10, 10, 19, 19));
        assert_eq!(model6.pos, Rect::from(9, 9, 20, 20));
    }

    #[test]
    fn test_toggle_model() {
        let mut model = from_string(vec![
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
        let state2 = from_string(vec![
            "⬜⬛⬛⬛".to_string(),
            "⬛⬜⬛⬛".to_string(),
            "⬜⬜⬜⬜".to_string(),
            "⬜⬜⬜⬜".to_string(),
        ]);
        let state3 = from_string(vec![
            "⬜⬛⬛⬛".to_string(),
            "⬛⬜⬛⬛".to_string(),
            "⬜⬜⬛⬜".to_string(),
            "⬜⬜⬜⬜".to_string(),
        ]);
        let state4 = from_string(vec![
            "⬜⬛⬛⬛".to_string(),
            "⬛⬜⬛⬛".to_string(),
            "⬜⬜⬛⬜".to_string(),
            "⬜⬜⬜⬛".to_string(),
        ]);
        let state5 = from_string(vec![
            "⬜⬛⬛⬛".to_string(),
            "⬛⬜⬛⬛".to_string(),
            "⬜⬜⬛⬜".to_string(),
            "⬛⬜⬜⬛".to_string(),
        ]);
        let state6 = from_string(vec![
            "⬜⬛⬛⬛".to_string(),
            "⬛⬜⬛⬛".to_string(),
            "⬜⬛⬛⬜".to_string(),
            "⬛⬜⬜⬛".to_string(),
        ]);
        let state7 = from_string(vec![
            "⬜⬛⬛⬛".to_string(),
            "⬛⬜⬜⬛".to_string(),
            "⬜⬛⬛⬜".to_string(),
            "⬛⬜⬜⬛".to_string(),
        ]);
        let state8 = from_string(vec![
            "⬜⬛⬛⬜".to_string(),
            "⬛⬜⬜⬛".to_string(),
            "⬜⬛⬛⬜".to_string(),
            "⬛⬜⬜⬛".to_string(),
        ]);
        toggle_cell(&mut model, Point::from(-2, 2));
        assert_eq!(model, state1);
        toggle_cell(&mut model, Point::from(-1, 1));
        assert_eq!(model, state2);
        toggle_cell(&mut model, Point::from(0, 0));
        assert_eq!(model, state3);
        toggle_cell(&mut model, Point::from(1, -1));
        assert_eq!(model, state4);
        toggle_cell(&mut model, Point::from(-2, -1));
        assert_eq!(model, state5);
        toggle_cell(&mut model, Point::from(-1, 0));
        assert_eq!(model, state6);
        toggle_cell(&mut model, Point::from(0, 1));
        assert_eq!(model, state7);
        toggle_cell(&mut model, Point::from(1, 2));
        assert_eq!(model, state8);
    }

    #[test]
    fn test_iterate() {
        let mut model1x1iter0 = from_string(vec!["⬜".to_string()]);
        let mut model1x1iter1 = from_string(vec!["⬛".to_string()]);
        model1x1iter1.iter = 1;

        let mut model2x2iter0 = from_string(vec!["⬜⬜".to_string(), "⬜⬜".to_string()]);
        let mut model2x2iter1 = from_string(vec!["⬜⬜".to_string(), "⬜⬜".to_string()]);
        model2x2iter1.iter = 1;

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

        iterate(&mut model1x1iter0);
        assert_eq!(model1x1iter0, model1x1iter1);
        iterate(&mut model2x2iter0);
        assert_eq!(model2x2iter0, model2x2iter1);
        iterate(&mut model3x3_1_iter0);
        assert_eq!(model3x3_1_iter0, model3x3_1_iter1);
        iterate(&mut model3x3_2_iter0);
        assert_eq!(model3x3_2_iter0, model3x3_2_iter1);
        iterate(&mut model3x3_3_iter0);
        assert_eq!(model3x3_3_iter0, model3x3_3_iter1);
        iterate(&mut model3x3_4_iter0);
        assert_eq!(model3x3_4_iter0, model3x3_4_iter1);
        iterate(&mut model3x3_5_iter0);
        assert_eq!(model3x3_5_iter0, model3x3_5_iter1);
    }
}
