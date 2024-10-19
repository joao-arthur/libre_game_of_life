use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
#[wasm_bindgen]
pub enum State {
    Dead,
    Alive,
}

pub fn iterate(state: State, number_of_alive_neighbors: u8) -> State {
    match state {
        State::Alive => match number_of_alive_neighbors {
            2 => State::Alive,
            3 => State::Alive,
            _ => State::Dead,
        },
        State::Dead => match number_of_alive_neighbors {
            3 => State::Alive,
            _ => State::Dead,
        },
    }
}

pub fn toggle(state: &State) -> State {
    match state {
        State::Alive => State::Dead,
        State::Dead => State::Alive,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_iterate_any_live_cell_with_fewer_than_two_live_neighbours_dies() {
        assert_eq!(iterate(State::Alive, 0), State::Dead);
        assert_eq!(iterate(State::Alive, 1), State::Dead);
    }

    #[test]
    fn test_iterate_any_live_cell_with_two_or_three_live_neighbours_lives() {
        assert_eq!(iterate(State::Alive, 2), State::Alive);
        assert_eq!(iterate(State::Alive, 3), State::Alive);
    }

    #[test]
    fn test_iterate_any_live_cell_with_more_than_three_live_neighbours_dies() {
        assert_eq!(iterate(State::Alive, 4), State::Dead);
        assert_eq!(iterate(State::Alive, 5), State::Dead);
        assert_eq!(iterate(State::Alive, 6), State::Dead);
        assert_eq!(iterate(State::Alive, 7), State::Dead);
        assert_eq!(iterate(State::Alive, 8), State::Dead);
    }

    #[test]
    fn test_iterate_any_dead_cell_with_exactly_three_live_neighbours_becomes_a_live_cell() {
        assert_eq!(iterate(State::Dead, 0), State::Dead);
        assert_eq!(iterate(State::Dead, 1), State::Dead);
        assert_eq!(iterate(State::Dead, 2), State::Dead);
        assert_eq!(iterate(State::Dead, 3), State::Alive);
        assert_eq!(iterate(State::Dead, 4), State::Dead);
        assert_eq!(iterate(State::Dead, 5), State::Dead);
        assert_eq!(iterate(State::Dead, 6), State::Dead);
        assert_eq!(iterate(State::Dead, 7), State::Dead);
        assert_eq!(iterate(State::Dead, 8), State::Dead);
    }

    #[test]
    fn test_toggle() {
        assert_eq!(toggle(&State::Alive), State::Dead);
        assert_eq!(toggle(&State::Dead), State::Alive);
    }
}
