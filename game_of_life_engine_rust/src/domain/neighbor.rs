use crate::domain::{cell::State, coordinate::CartesianP, universe::Universe};

pub type Neighbor<'a> = Option<&'a State>;

fn number_of_alive(neighbors: [Neighbor; 8]) -> u8 {
    neighbors
        .iter()
        .filter(|neighbor| neighbor == &&Some(&State::Alive))
        .count() as u8
}

fn from_point_in_rectangular_grid(u: &Universe, p: CartesianP) -> [Neighbor; 8] {
    [
        u.value.get(&CartesianP::from(p.x - 1, p.y + 1)),
        u.value.get(&CartesianP::from(p.x, p.y + 1)),
        u.value.get(&CartesianP::from(p.x + 1, p.y + 1)),
        u.value.get(&CartesianP::from(p.x - 1, p.y)),
        u.value.get(&CartesianP::from(p.x + 1, p.y)),
        u.value.get(&CartesianP::from(p.x - 1, p.y - 1)),
        u.value.get(&CartesianP::from(p.x, p.y - 1)),
        u.value.get(&CartesianP::from(p.x + 1, p.y - 1)),
    ]
}

pub fn number_of_alive_from_model(u: &Universe, p: CartesianP) -> u8 {
    number_of_alive(from_point_in_rectangular_grid(u, p))
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
