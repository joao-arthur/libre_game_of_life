use crate::{cell::State, geometry::coordinate::CartesianPoint, universe::Universe};

use super::universe::universe_get_value;

fn number_of_alive(neighbors: [State; 8]) -> u8 {
    neighbors.iter().filter(|neighbor| neighbor == &&State::Alive).count() as u8
}

fn neighbors_in_rectangular_grid(universe: &Universe, point: &CartesianPoint) -> [State; 8] {
    [
        universe_get_value(universe, &CartesianPoint::of(point.x - 1, point.y + 1)),
        universe_get_value(universe, &CartesianPoint::of(point.x, point.y + 1)),
        universe_get_value(universe, &CartesianPoint::of(point.x + 1, point.y + 1)),
        universe_get_value(universe, &CartesianPoint::of(point.x - 1, point.y)),
        universe_get_value(universe, &CartesianPoint::of(point.x + 1, point.y)),
        universe_get_value(universe, &CartesianPoint::of(point.x - 1, point.y - 1)),
        universe_get_value(universe, &CartesianPoint::of(point.x, point.y - 1)),
        universe_get_value(universe, &CartesianPoint::of(point.x + 1, point.y - 1)),
    ]
}

pub fn number_of_alive_from_model(universe: &Universe, point: &CartesianPoint) -> u8 {
    number_of_alive(neighbors_in_rectangular_grid(universe, point))
}

#[cfg(test)]
mod tests {
    use crate::cell::State;

    use super::number_of_alive;

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
