use crate::{cell::State, geometry::coordinate::CartesianPoint, universe::Universe};

use super::universe::get_value;

fn number_of_alive(neighbors: [State; 8]) -> u8 {
    neighbors.iter().filter(|neighbor| neighbor == &&State::Alive).count() as u8
}

fn neighbors_in_rectangular_grid(u: &Universe, p: &CartesianPoint) -> [State; 8] {
    [
        get_value(u, &CartesianPoint::of(p.x - 1, p.y + 1)),
        get_value(u, &CartesianPoint::of(p.x, p.y + 1)),
        get_value(u, &CartesianPoint::of(p.x + 1, p.y + 1)),
        get_value(u, &CartesianPoint::of(p.x - 1, p.y)),
        get_value(u, &CartesianPoint::of(p.x + 1, p.y)),
        get_value(u, &CartesianPoint::of(p.x - 1, p.y - 1)),
        get_value(u, &CartesianPoint::of(p.x, p.y - 1)),
        get_value(u, &CartesianPoint::of(p.x + 1, p.y - 1)),
    ]
}

pub fn number_of_alive_from_model(u: &Universe, p: &CartesianPoint) -> u8 {
    number_of_alive(neighbors_in_rectangular_grid(u, p))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_of_alive() {
        assert_eq!(
            number_of_alive([
                State::Dead,
                State::Dead,
                State::Dead,
                State::Dead,
                State::Dead,
                State::Dead,
                State::Dead,
                State::Dead,
            ]),
            0
        );
        assert_eq!(
            number_of_alive([
                State::Alive,
                State::Alive,
                State::Alive,
                State::Alive,
                State::Alive,
                State::Alive,
                State::Alive,
                State::Alive,
            ]),
            8
        );
        assert_eq!(
            number_of_alive([
                State::Alive,
                State::Dead,
                State::Alive,
                State::Dead,
                State::Alive,
                State::Dead,
                State::Alive,
                State::Dead
            ]),
            4
        );
        assert_eq!(
            number_of_alive([
                State::Dead,
                State::Alive,
                State::Dead,
                State::Alive,
                State::Dead,
                State::Alive,
                State::Dead,
                State::Alive
            ]),
            4
        );
    }
}
