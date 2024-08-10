use std::collections::HashSet;

use crate::{cartesian_plane::Point, cell::State};

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

/*
pub fn fromString(stringValue: string[])-> Model {
    const length = stringValue.length;

    const aliveCells = stringValue.flatMap(
        (rowValue, row) =>
            rowValue
                .split("")
                .map((colValue, col) =>
                    colValue === "⬜"
                        ? cartesianPlaneFns.indexToPoint(
                            { row, col },
                            length,
                        )
                        : undefined
                )
                .filter((entry) => entry !== undefined)
                .map((entry) => entry as Point),
    );

    const entries: [string, State.ALIVE][] = aliveCells.map(
        (aliveCell) => [
            cartesianPlaneFns.serializePoint(aliveCell),
            State.ALIVE,
        ],
    );
    const value = new Map(entries);

    return {
        value,
        iteration: 0,
        pos: {
            x1: -10,
            y1: -10,
            x2: 10,
            y2: 10,
        },
    };
}
*/

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

pub fn getMiddleCell(model: &Model, total_size: i64) -> Point {
    let cell_size = get_cell_size(model, total_size);
    let middle = get_middle_point(model);

    Point {
        x: middle.x * cell_size,
        y: middle.y * cell_size,
    }
}
/*
pub fn getValue(model: &Model, point: Point,) -> State {
    return model.value.has(cartesianPlaneFns.serialize_point(point))
        ? State.ALIVE
        : State.DEAD;
}

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
        iteration: model.iteration + 1,
        pos: model.pos,
    };
}


pub fn move(
    model: Model,
    delta: Point,
) -> Model {
    return {
        value: model.value,
        iteration: model.iteration,
        pos: {
            x1: model.pos.x1 + delta.x,
            y1: model.pos.y1 + delta.y,
            x2: model.pos.x2 + delta.x,
            y2: model.pos.y2 + delta.y,
        },
    };
}

pub fn toggle(
    model: Model,
    point: Point,
) -> Model {
    let key = cartesianPlaneFns.serializePoint(point);
    let current = map.entries(model.value);

    let entries = model.value.has(key)
        ? current.filter(([valueKey]) => valueKey !== key)
        : current.concat([[key, State.ALIVE]]);

    return {
        value: new Map(entries),
        iteration: model.iteration,
        pos: model.pos,
    };
}


pub fn zoom(model: Model, newSize: number) -> Model {
    let halfNewSize = newSize / 2;
    let halfX = (model.pos.x1 + model.pos.x2) / 2;
    let halfY = (model.pos.y1 + model.pos.y2) / 2;

    let x1 = Math.ceil(halfX - halfNewSize);
    let y1 = Math.ceil(halfY - halfNewSize);
    let x2 = x1 + newSize - 1;
    let y2 = y1 + newSize - 1;

    return {
        value: model.value,
        iteration: model.iteration,
        pos: {
            x1: num.normalizeZero(x1),
            y1: num.normalizeZero(y1),
            x2: num.normalizeZero(x2),
            y2: num.normalizeZero(y2),
        },
    };
}
*/

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
    fn test_get_length() {
        assert_eq!(get_length(&Model::from(Rect::from(-10, -10, 10, 10))), 21);
        assert_eq!(get_length(&Model::from(Rect::from(1, 1, 10, 10))), 10);
        assert_eq!(get_length(&Model::from(Rect::from(4, 4, 5, 5))), 2);
        assert_eq!(get_length(&Model::from(Rect::from(5, 5, 5, 5))), 1);
    }
}
