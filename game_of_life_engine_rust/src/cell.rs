#[derive(Debug, PartialEq, Clone)]
pub enum State {
    DEAD,
    ALIVE,
}

pub fn iterate(state: State, number_of_alive_neighbors: u8) -> State {
    match state {
        State::ALIVE => match number_of_alive_neighbors {
            2 => State::ALIVE,
            3 => State::ALIVE,
            _ => State::DEAD,
        },
        State::DEAD => match number_of_alive_neighbors {
            3 => State::ALIVE,
            _ => State::DEAD,
        },
    }
}

pub fn toggle(state: &State) -> State {
    match state {
        State::ALIVE => State::DEAD,
        State::DEAD => State::ALIVE,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_iterate_any_live_cell_with_fewer_than_two_live_neighbours_dies() {
        assert_eq!(iterate(State::ALIVE, 0), State::DEAD);
        assert_eq!(iterate(State::ALIVE, 1), State::DEAD);
    }

    #[test]
    fn test_iterate_any_live_cell_with_two_or_three_live_neighbours_lives() {
        assert_eq!(iterate(State::ALIVE, 2), State::ALIVE);
        assert_eq!(iterate(State::ALIVE, 3), State::ALIVE);
    }

    #[test]
    fn test_iterate_any_live_cell_with_more_than_three_live_neighbours_dies() {
        assert_eq!(iterate(State::ALIVE, 4), State::DEAD);
        assert_eq!(iterate(State::ALIVE, 5), State::DEAD);
        assert_eq!(iterate(State::ALIVE, 6), State::DEAD);
        assert_eq!(iterate(State::ALIVE, 7), State::DEAD);
        assert_eq!(iterate(State::ALIVE, 8), State::DEAD);
    }

    #[test]
    fn test_iterate_any_dead_cell_with_exactly_three_live_neighbours_becomes_a_live_cell() {
        assert_eq!(iterate(State::DEAD, 0), State::DEAD);
        assert_eq!(iterate(State::DEAD, 1), State::DEAD);
        assert_eq!(iterate(State::DEAD, 2), State::DEAD);
        assert_eq!(iterate(State::DEAD, 3), State::ALIVE);
        assert_eq!(iterate(State::DEAD, 4), State::DEAD);
        assert_eq!(iterate(State::DEAD, 5), State::DEAD);
        assert_eq!(iterate(State::DEAD, 6), State::DEAD);
        assert_eq!(iterate(State::DEAD, 7), State::DEAD);
        assert_eq!(iterate(State::DEAD, 8), State::DEAD);
    }

    #[test]
    fn test_toggle() {
        assert_eq!(toggle(&State::ALIVE), State::DEAD);
        assert_eq!(toggle(&State::DEAD), State::ALIVE);
    }
}
