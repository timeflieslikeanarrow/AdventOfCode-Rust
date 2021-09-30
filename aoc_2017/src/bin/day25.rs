use std::collections::HashMap;
use std::iter::FromIterator;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum State {
    A,
    B,
    C,
    D,
    E,
    F
}

use self::State::*;

fn main() {
    let transitions = HashMap::from_iter([
        (A, vec![(1,  1, B), (0, -1, B)]),
        (B, vec![(0,  1, C), (1, -1, B)]),
        (C, vec![(1,  1, D), (0, -1, A)]),
        (D, vec![(1, -1, E), (1, -1, F)]),
        (E, vec![(1, -1, A), (0, -1, D)]),
        (F, vec![(1,  1, A), (1, -1, E)]),
    ]);

    println!("part 1: {}", turing_machine(&transitions, 12586542));
}

fn turing_machine(transitions: &HashMap<State, Vec<(usize, i32, State)>>, steps: usize) -> usize {
    let mut tape = HashMap::new();
    let mut position = 0;
    let mut current_state = A;

    for _ in 0..steps {
        let current_value = *tape.get(&position).unwrap_or(&0);
        let table = &transitions[&current_state];
        let (value, direction, state) = &table[current_value];
        tape.insert(position, *value);
        position += *direction;
        current_state = *state;
    }

    tape.values().filter(|n| **n == 1).count()
}

#[cfg(test)]
mod day25_tests {
    use super::*;

    #[test]
    fn example() {
        let transitions = HashMap::from_iter([
            (A, vec![(1, 1, B), (0, -1, B)]),
            (B, vec![(1, -1, A), (1, 1, A)]),
        ]);

        assert_eq!(turing_machine(&transitions, 6), 3);
    }
}