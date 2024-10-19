use crate::domain::{cell::State, model::Model, plane::cartesian::CartesianPoint};

pub type Neighbor<'a> = Option<&'a State>;

pub type Neighbors<'a> = [Neighbor<'a>; 8];

fn number_of_alive(neighbors: Neighbors) -> u8 {
    return neighbors
        .iter()
        .filter(|n| n == &&Some(&State::Alive))
        .count() as u8;
}

fn from_model(m: &Model, point: CartesianPoint) -> Neighbors {
    return [
        m.value.get(&CartesianPoint::from(point.x - 1, point.y + 1)),
        m.value.get(&CartesianPoint::from(point.x, point.y + 1)),
        m.value.get(&CartesianPoint::from(point.x + 1, point.y + 1)),
        m.value.get(&CartesianPoint::from(point.x - 1, point.y)),
        m.value.get(&CartesianPoint::from(point.x + 1, point.y)),
        m.value.get(&CartesianPoint::from(point.x - 1, point.y - 1)),
        m.value.get(&CartesianPoint::from(point.x, point.y - 1)),
        m.value.get(&CartesianPoint::from(point.x + 1, point.y - 1)),
    ];
}

pub fn number_of_alive_from_model(m: &Model, point: CartesianPoint) -> u8 {
    number_of_alive(from_model(m, point))
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
                Some(&State::Dead),
                Some(&State::Dead),
                Some(&State::Dead),
                Some(&State::Dead),
                Some(&State::Dead),
                Some(&State::Dead),
                Some(&State::Dead),
                Some(&State::Dead),
            ]),
            0
        );
        assert_eq!(
            number_of_alive([
                Some(&State::Alive),
                Some(&State::Alive),
                Some(&State::Alive),
                Some(&State::Alive),
                Some(&State::Alive),
                Some(&State::Alive),
                Some(&State::Alive),
                Some(&State::Alive),
            ]),
            8
        );
        assert_eq!(
            number_of_alive([
                Some(&State::Alive),
                Some(&State::Dead),
                Some(&State::Dead),
                Some(&State::Dead),
                Some(&State::Dead),
                Some(&State::Dead),
                Some(&State::Dead),
                None,
            ]),
            1
        );
    }
}
