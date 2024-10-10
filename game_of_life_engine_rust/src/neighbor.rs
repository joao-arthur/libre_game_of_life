use crate::{cartesian_plane::Point, cell::State, model::Model};

pub type Neighbor<'a> = Option<&'a State>;

pub type Neighbors<'a> = [Neighbor<'a>; 8];

fn number_of_alive(neighbors: Neighbors) -> u8 {
    return neighbors
        .iter()
        .filter(|n| n == &&Some(&State::ALIVE))
        .count() as u8;
}

fn from_model(model: &Model, point: Point) -> Neighbors {
    return [
        model.value.get(&Point::from(point.x - 1, point.y + 1)),
        model.value.get(&Point::from(point.x, point.y + 1)),
        model.value.get(&Point::from(point.x + 1, point.y + 1)),
        model.value.get(&Point::from(point.x - 1, point.y)),
        model.value.get(&Point::from(point.x + 1, point.y)),
        model.value.get(&Point::from(point.x - 1, point.y - 1)),
        model.value.get(&Point::from(point.x, point.y - 1)),
        model.value.get(&Point::from(point.x + 1, point.y - 1)),
    ];
}

pub fn number_of_alive_from_model(model: &Model, point: Point) -> u8 {
    number_of_alive(from_model(model, point))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_number_of_alive() {
        assert_eq!(
            number_of_alive([None, None, None, None, None, None, None, None,]),
            0
        );
        assert_eq!(
            number_of_alive([
                Some(&State::DEAD),
                Some(&State::DEAD),
                Some(&State::DEAD),
                Some(&State::DEAD),
                Some(&State::DEAD),
                Some(&State::DEAD),
                Some(&State::DEAD),
                Some(&State::DEAD),
            ]),
            0
        );
        assert_eq!(
            number_of_alive([
                Some(&State::ALIVE),
                Some(&State::ALIVE),
                Some(&State::ALIVE),
                Some(&State::ALIVE),
                Some(&State::ALIVE),
                Some(&State::ALIVE),
                Some(&State::ALIVE),
                Some(&State::ALIVE),
            ]),
            8
        );
        assert_eq!(
            number_of_alive([
                Some(&State::ALIVE),
                Some(&State::DEAD),
                Some(&State::DEAD),
                Some(&State::DEAD),
                Some(&State::DEAD),
                Some(&State::DEAD),
                Some(&State::DEAD),
                None,
            ]),
            1
        );
    }
}
