#[derive(Debug, PartialEq, Clone, Copy)]
pub enum State {
    Dead,
    Alive,
}

pub fn cell_iterate(state: State, number_of_alive_neighbors: u8) -> State {
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

pub fn cell_toggle(state: &State) -> State {
    match state {
        State::Alive => State::Dead,
        State::Dead => State::Alive,
    }
}

pub fn cell_try_of(c: char) -> Option<State> {
    match c {
        '⬜' => Some(State::Alive),
        '⬛' => Some(State::Dead),
        _ => None,
    }
}

pub fn cell_of(c: char) -> State {
    cell_try_of(c).unwrap()
}

#[cfg(test)]
mod tests {
    use super::{State, cell_iterate, cell_of, cell_toggle, cell_try_of};

    #[test]
    fn any_live_cell_with_fewer_than_two_live_neighbours_dies() {
        assert_eq!(cell_iterate(State::Alive, 0), State::Dead);
        assert_eq!(cell_iterate(State::Alive, 1), State::Dead);
    }

    #[test]
    fn any_live_cell_with_two_or_three_live_neighbours_lives() {
        assert_eq!(cell_iterate(State::Alive, 2), State::Alive);
        assert_eq!(cell_iterate(State::Alive, 3), State::Alive);
    }

    #[test]
    fn any_live_cell_with_more_than_three_live_neighbours_dies() {
        assert_eq!(cell_iterate(State::Alive, 4), State::Dead);
        assert_eq!(cell_iterate(State::Alive, 5), State::Dead);
        assert_eq!(cell_iterate(State::Alive, 6), State::Dead);
        assert_eq!(cell_iterate(State::Alive, 7), State::Dead);
        assert_eq!(cell_iterate(State::Alive, 8), State::Dead);
    }

    #[test]
    fn any_dead_cell_with_exactly_three_live_neighbours_becomes_a_live_cell() {
        assert_eq!(cell_iterate(State::Dead, 0), State::Dead);
        assert_eq!(cell_iterate(State::Dead, 1), State::Dead);
        assert_eq!(cell_iterate(State::Dead, 2), State::Dead);
        assert_eq!(cell_iterate(State::Dead, 3), State::Alive);
        assert_eq!(cell_iterate(State::Dead, 4), State::Dead);
        assert_eq!(cell_iterate(State::Dead, 5), State::Dead);
        assert_eq!(cell_iterate(State::Dead, 6), State::Dead);
        assert_eq!(cell_iterate(State::Dead, 7), State::Dead);
        assert_eq!(cell_iterate(State::Dead, 8), State::Dead);
    }

    #[test]
    fn test_cell_toggle() {
        assert_eq!(cell_toggle(&State::Alive), State::Dead);
        assert_eq!(cell_toggle(&State::Dead), State::Alive);
    }

    #[test]
    fn test_cell_try_of() {
        assert_eq!(cell_try_of('⬛'), Some(State::Dead));
        assert_eq!(cell_try_of('⬜'), Some(State::Alive));
        assert_eq!(cell_try_of(' '), None);
    }

    #[test]
    fn test_cell_of() {
        assert_eq!(cell_of('⬛'), State::Dead);
        assert_eq!(cell_of('⬜'), State::Alive);
    }
}
