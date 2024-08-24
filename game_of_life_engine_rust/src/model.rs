use std::collections::HashSet;

use crate::{
    cartesian_plane::{index_to_point, serialize_point, ArrPos, Point},
    cell::State,
};

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub struct Model {
    pub value: HashSet<String>,
    pub iter: u64,
    pub pos: Rect,
}

impl Model {
    pub fn from(pos: Rect) -> Self {
        Model {
            pos,
            ..Default::default()
        }
    }
}

impl Default for Model {
    fn default() -> Self {
        Model {
            value: HashSet::new(),
            iter: 0,
            pos: Rect::from(0, 0, 0, 0),
        }
    }
}

pub fn from_string(model_as_str: Vec<String>) -> Model {
    let mut value = HashSet::<String>::new();

    let len = model_as_str.len() as i64;
    let row_iter = model_as_str.iter().enumerate();
    for (row, row_str) in row_iter {
        let col_iter = row_str.chars().enumerate();
        for (col, col_str) in col_iter {
            if col_str == '⬜' {
                value.insert(serialize_point(index_to_point(ArrPos { col: col as i64, row: row as i64 }, len)));
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
    if model.value.contains(&serialize_point(point)) {
        State::ALIVE
    } else {
        State::DEAD
    }
}

/*
pub fn iterate(
    model: Model,
) -> Model {
    let iteratingPoints = map
        .keys(model.value)
        .map(cartesianPlaneFns.deserialize_point)
        .flatMap((point) => [
            { x: point.x - 1, y: point.y + 1 },
            { x: point.x, y: point.y + 1 },
            { x: point.x + 1, y: point.y + 1 },
            { x: point.x - 1, y: point.y },
            { x: point.x, y: point.y },
            { x: point.x + 1, y: point.y },
            { x: point.x - 1, y: point.y - 1 },
            { x: point.x, y: point.y - 1 },
            { x: point.x + 1, y: point.y - 1 },
        ]);
    let uniquePoints = arr
        .unique(iteratingPoints.map(cartesianPlaneFns.serializePoint))
        .map(cartesianPlaneFns.deserializePoint);
    let entries = uniquePoints
        .map((point) => {
            let state = getValue(model, point);
            let neighbors = neighborsFns.aliveFromModel(
                model,
                point,
            );
            let newCell = cellFns.iterate(state, neighbors);
            return newCell === State.ALIVE
                ? [cartesianPlaneFns.serializePoint(point), newCell]
                : undefined;
        })
        .filter((value) => value !== undefined)
        .map((value) => value as [string, State.ALIVE]);

    return {
        value: new Map(entries),
        iter: model.iter + 1,
        pos: model.pos,
    };
}
*/

pub fn move_in_plane(model: Model, delta: Point) -> Model {
    Model {
        value: model.value,
        iter: model.iter,
        pos: Rect::from(
            model.pos.x1 + delta.x,
            model.pos.y1 + delta.y,
            model.pos.x2 + delta.x,
            model.pos.y2 + delta.y,
        ),
    }
}

// pub fn toggle_cell(model: Model,point: Point,) -> Model {
//     let key = serialize_point(point);
//     let current = map.entries(model.value);
//
//     let entries = if model.value.contains(key) {
//         current.filter(([valueKey]) => valueKey !== key)
//     } else {
//         current.concat([[key, State.ALIVE]]);
//     }
//
//     return {
//         value: new Map(entries),
//         iter: model.iter,
//         pos: model.pos,
//     };
// }

pub fn zoom(model: Model, new_size: i64) -> Model {
    let half_new_size = new_size as f64 / 2 as f64;
    let half_x = (model.pos.x1 + model.pos.x2) as f64 / 2 as f64;
    let half_y = (model.pos.y1 + model.pos.y2) as f64 / 2 as f64;

    let x1 = (half_x - half_new_size).ceil() as i64;
    let y1 = (half_y - half_new_size).ceil() as i64;
    let x2 = x1 + new_size as i64 - 1;
    let y2 = y1 + new_size as i64 - 1;

    Model {
        value: model.value,
        iter: model.iter,
        pos: Rect { x1, y1, x2, y2 },
    }
}

#[cfg(test)]
mod test {
    use super::*;

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
                value: HashSet::new(),
                iter: 0,
                pos: Rect::from(0, 0, 0, 0)
            }
        );
        assert_eq!(
            Model::from(Rect::from(-23, 38, 198, 7)),
            Model {
                value: HashSet::new(),
                iter: 0,
                pos: Rect::from(-23, 38, 198, 7)
            }
        );
    }

    #[test]
    fn test_from_string() {
    assert_eq!(
        from_string([""]),Model { value: new Map(), iteration: 0, position: { x1: -10, y1: -10, x2: 10, y2: 10 },});
    assert_eq!(
        from_string(["⬛"]),Model { value: new Map(), iteration: 0, position: { x1: -10, y1: -10, x2: 10, y2: 10 },});
    assert_eq!(
        from_string(["⬜"]), Model { value: new Map([["(x: 0, y: 0)", State.ALIVE]]), iteration: 0, position: { x1: -10, y1: -10, x2: 10, y2: 10 }, });
    assert_eq!(
        from_string([
            "⬛⬛⬛⬜",
            "⬜⬛⬛⬛",
            "⬛⬛⬜⬛",
            "⬛⬛⬛⬛",
        ]),
    {
        value: new Map([
            ["(x: 1, y: 2)", State.ALIVE],
            ["(x: -2, y: 1)", State.ALIVE],
            ["(x: 0, y: 0)", State.ALIVE],
        ]),
        iteration: 0,
        position: { x1: -10, y1: -10, x2: 10, y2: 10 },
    });
});
    }

    #[test]
    fn test_get_length() {
        assert_eq!(get_length(&Model::from(Rect::from(-10, -10, 10, 10))), 21);
        assert_eq!(get_length(&Model::from(Rect::from(1, 1, 10, 10))), 10);
        assert_eq!(get_length(&Model::from(Rect::from(4, 4, 5, 5))), 2);
        assert_eq!(get_length(&Model::from(Rect::from(5, 5, 5, 5))), 1);
    }

    #[test]
    fn test_get_cell_size() {
        let model = Model {
            pos: Rect::from(1, 1, 10, 10),
            ..Default::default()
        };
        assert_eq!(get_cell_size(&model, 100), 10);
        assert_eq!(get_cell_size(&model, 90), 9);
        assert_eq!(get_cell_size(&model, 50), 5);
        assert_eq!(get_cell_size(&model, 10), 1);
    }

    #[test]
    fn test_get_middle_point() {
        assert_eq!(
            get_middle_point(&Model {
                pos: Rect::from(-10, -10, 10, 10),
                ..Default::default()
            }),
            Point::from(0, 0)
        );
        // assert_eq!(
        //     get_middle_point(&Model {
        //         pos: Rect::from(1, 1, 10, 10),
        //         ..Default::default()
        //     }),
        //     Point::from(5.5, 5.5)
        // );
        // assert_eq!(
        //     get_middle_point(&Model {
        //         pos: Rect::from(4, 4, 5, 5),
        //         ..Default::default()
        //     }),
        //     Point::from(4.5, 4.5)
        // );
        assert_eq!(
            get_middle_point(&Model {
                pos: Rect::from(5, 5, 5, 5),
                ..Default::default()
            }),
            Point::from(5, 5)
        );
    }

    #[test]
    fn test_get_middle_cell() {
        assert_eq!(
            get_middle_cell(
                &Model {
                    pos: Rect::from(-10, -10, 10, 10),
                    ..Default::default()
                },
                100,
            ),
            Point::from(0, 0)
        );
        //assert_eq!(
        //    get_middle_cell(
        //        &Model {
        //            pos: Rect::from(1, 1, 10, 10),
        //            ..Default::default()
        //        },
        //        50
        //    ),
        //    Point::from(27.5, 27.5)
        //);
        //assert_eq!(
        //    get_middle_cell(
        //        &Model {
        //            pos: Rect::from(4, 4, 5, 5),
        //            ..Default::default()
        //        },
        //        10
        //    ),
        //    Point::from(22.5, 22.5)
        //);
        assert_eq!(
            get_middle_cell(
                &Model {
                    pos: Rect::from(5, 5, 5, 5),
                    ..Default::default()
                },
                1
            ),
            Point::from(5, 5)
        );
    }

    #[test]
    fn test_move_in_plane() {
        assert_eq!(
            move_in_plane(
                Model {
                    pos: Rect::from(-10, -10, 10, 10),
                    ..Default::default()
                },
                Point { x: 1, y: 0 }
            ),
            Model {
                pos: Rect::from(-9, -10, 11, 10),
                ..Default::default()
            }
        );
        assert_eq!(
            move_in_plane(
                Model {
                    pos: Rect::from(-10, -10, 10, 10),
                    ..Default::default()
                },
                Point { x: -1, y: 0 }
            ),
            Model {
                pos: Rect::from(-11, -10, 9, 10),
                ..Default::default()
            }
        );
        assert_eq!(
            move_in_plane(
                Model {
                    pos: Rect::from(-10, -10, 10, 10),
                    ..Default::default()
                },
                Point { x: 0, y: 1 }
            ),
            Model {
                pos: Rect::from(-10, -9, 10, 11),
                ..Default::default()
            }
        );
        assert_eq!(
            move_in_plane(
                Model {
                    pos: Rect::from(-10, -10, 10, 10),
                    ..Default::default()
                },
                Point { x: 0, y: -1 }
            ),
            Model {
                pos: Rect::from(-10, -11, 10, 9),
                ..Default::default()
            }
        );

        assert_eq!(
            move_in_plane(
                Model {
                    pos: Rect::from(-10, -10, 10, 10),
                    ..Default::default()
                },
                Point { x: 11, y: 0 }
            ),
            Model {
                pos: Rect::from(1, -10, 21, 10),
                ..Default::default()
            }
        );
        assert_eq!(
            move_in_plane(
                Model {
                    pos: Rect::from(-10, -10, 10, 10),
                    ..Default::default()
                },
                Point { x: -11, y: 0 }
            ),
            Model {
                pos: Rect::from(-21, -10, -1, 10),
                ..Default::default()
            }
        );
        assert_eq!(
            move_in_plane(
                Model {
                    pos: Rect::from(-10, -10, 10, 10),
                    ..Default::default()
                },
                Point { x: 0, y: 11 }
            ),
            Model {
                pos: Rect::from(-10, 1, 10, 21),
                ..Default::default()
            }
        );
        assert_eq!(
            move_in_plane(
                Model {
                    pos: Rect::from(-10, -10, 10, 10),
                    ..Default::default()
                },
                Point { x: 0, y: -11 }
            ),
            Model {
                pos: Rect::from(-10, -21, 10, -1),
                ..Default::default()
            }
        );
    }

    #[test]
    fn test_zoom_odd_size() {
        assert_eq!(
            zoom(
                Model {
                    pos: Rect::from(-10, -10, 10, 10),
                    ..Default::default()
                },
                1
            ),
            Model {
                pos: Rect::from(0, 0, 0, 0),
                ..Default::default()
            }
        );
        assert_eq!(
            zoom(
                Model {
                    pos: Rect::from(-10, -10, 10, 10),
                    ..Default::default()
                },
                2
            ),
            Model {
                pos: Rect::from(-1, -1, 0, 0),
                ..Default::default()
            }
        );
        assert_eq!(
            zoom(
                Model {
                    pos: Rect::from(-10, -10, 10, 10),
                    ..Default::default()
                },
                3
            ),
            Model {
                pos: Rect::from(-1, -1, 1, 1),
                ..Default::default()
            }
        );

        assert_eq!(
            zoom(
                Model {
                    pos: Rect::from(-10, -10, 10, 10),
                    ..Default::default()
                },
                19
            ),
            Model {
                pos: Rect::from(-9, -9, 9, 9),
                ..Default::default()
            }
        );
        assert_eq!(
            zoom(
                Model {
                    pos: Rect::from(-10, -10, 10, 10),
                    ..Default::default()
                },
                21
            ),
            Model {
                pos: Rect::from(-10, -10, 10, 10),
                ..Default::default()
            }
        );
        assert_eq!(
            zoom(
                Model {
                    pos: Rect::from(-10, -10, 10, 10),
                    ..Default::default()
                },
                23
            ),
            Model {
                pos: Rect::from(-11, -11, 11, 11),
                ..Default::default()
            }
        );
    }

    #[test]
    fn test_zoom_even_size() {
        assert_eq!(
            zoom(
                Model {
                    pos: Rect::from(10, 10, 19, 19),
                    ..Default::default()
                },
                1
            ),
            Model {
                pos: Rect::from(14, 14, 14, 14),
                ..Default::default()
            }
        );
        assert_eq!(
            zoom(
                Model {
                    pos: Rect::from(10, 10, 19, 19),
                    ..Default::default()
                },
                2
            ),
            Model {
                pos: Rect::from(14, 14, 15, 15),
                ..Default::default()
            }
        );
        assert_eq!(
            zoom(
                Model {
                    pos: Rect::from(10, 10, 19, 19),
                    ..Default::default()
                },
                3
            ),
            Model {
                pos: Rect::from(13, 13, 15, 15),
                ..Default::default()
            }
        );

        assert_eq!(
            zoom(
                Model {
                    pos: Rect::from(10, 10, 19, 19),
                    ..Default::default()
                },
                8
            ),
            Model {
                pos: Rect::from(11, 11, 18, 18),
                ..Default::default()
            }
        );
        assert_eq!(
            zoom(
                Model {
                    pos: Rect::from(10, 10, 19, 19),
                    ..Default::default()
                },
                10
            ),
            Model {
                pos: Rect::from(10, 10, 19, 19),
                ..Default::default()
            }
        );
        assert_eq!(
            zoom(
                Model {
                    pos: Rect::from(10, 10, 19, 19),
                    ..Default::default()
                },
                12
            ),
            Model {
                pos: Rect::from(9, 9, 20, 20),
                ..Default::default()
            }
        );
    }
}
