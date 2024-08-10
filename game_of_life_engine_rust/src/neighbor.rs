use crate::cell::State;

pub type Neighbor = Option<State>;

pub type Neighbors = [Neighbor; 8];

fn number_of_alive(neighbors: &Neighbors) -> u8 {
    return neighbors
        .iter()
        .filter(|n| n == &&Some(State::ALIVE))
        .count() as u8;
}

/*
pub fn number_of_alive_from_model(model: Model, point: CartesianPoint) -> u8 {
    number_of_alive(from_Model(model, point))
}

fn from_Model(
    model: Model,
    point: CartesianPoint,
): Neighbors {
    const { getValue } = modelFns;

    return [
        getValue(model, { x: point.x - 1, y: point.y + 1 }),
        getValue(model, { x: point.x, y: point.y + 1 }),
        getValue(model, { x: point.x + 1, y: point.y + 1 }),

        getValue(model, { x: point.x - 1, y: point.y }),
        getValue(model, { x: point.x + 1, y: point.y }),

        getValue(model, { x: point.x - 1, y: point.y - 1 }),
        getValue(model, { x: point.x, y: point.y - 1 }),
        getValue(model, { x: point.x + 1, y: point.y - 1 }),
    ];
}


*/

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_number_of_alive() {
        assert_eq!(
            number_of_alive(&[None, None, None, None, None, None, None, None,]),
            0
        );
        assert_eq!(
            number_of_alive(&[
                Some(State::DEAD),
                Some(State::DEAD),
                Some(State::DEAD),
                Some(State::DEAD),
                Some(State::DEAD),
                Some(State::DEAD),
                Some(State::DEAD),
                Some(State::DEAD),
            ]),
            0
        );
        assert_eq!(
            number_of_alive(&[
                Some(State::ALIVE),
                Some(State::ALIVE),
                Some(State::ALIVE),
                Some(State::ALIVE),
                Some(State::ALIVE),
                Some(State::ALIVE),
                Some(State::ALIVE),
                Some(State::ALIVE),
            ]),
            8
        );
        assert_eq!(
            number_of_alive(&[
                Some(State::ALIVE),
                Some(State::DEAD),
                Some(State::DEAD),
                Some(State::DEAD),
                Some(State::DEAD),
                Some(State::DEAD),
                Some(State::DEAD),
                None,
            ]),
            1
        );
    }
}
