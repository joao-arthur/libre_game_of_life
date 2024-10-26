use crate::domain::{cell::State, plane::cartesian::CartesianPoint, universe::Universe};

pub type Neighbor<'a> = Option<&'a State>;

fn number_of_alive(neighbors: [Neighbor; 8]) -> u8 {
    neighbors
        .iter()
        .filter(|neighbor| neighbor == &&Some(&State::Alive))
        .count() as u8
}

fn from_point_in_rectangular_grid(u: &Universe, point: CartesianPoint) -> [Neighbor; 8] {
    [
        u.value.get(&CartesianPoint::from(point.x - 1, point.y + 1)),
        u.value.get(&CartesianPoint::from(point.x, point.y + 1)),
        u.value.get(&CartesianPoint::from(point.x + 1, point.y + 1)),
        u.value.get(&CartesianPoint::from(point.x - 1, point.y)),
        u.value.get(&CartesianPoint::from(point.x + 1, point.y)),
        u.value.get(&CartesianPoint::from(point.x - 1, point.y - 1)),
        u.value.get(&CartesianPoint::from(point.x, point.y - 1)),
        u.value.get(&CartesianPoint::from(point.x + 1, point.y - 1)),
    ]
}

pub fn number_of_alive_from_model(u: &Universe, point: CartesianPoint) -> u8 {
    number_of_alive(from_point_in_rectangular_grid(u, point))
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
