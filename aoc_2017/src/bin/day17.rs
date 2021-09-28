fn main() {
    let puzzle_input = 328;
    println!("part 1: {}", spin_lock(puzzle_input, 2017));
    println!("part 2: {}", spin_lock2(puzzle_input, 50_000_000));
}

fn spin_lock(steps: usize, count: usize) -> usize {
    let mut state = vec![0];
    let mut current_pos = 0;

    for i in 1..=count {
        current_pos = (current_pos + steps) % state.len();
        state.insert(current_pos + 1, i);
        current_pos  = (current_pos + 1) % state.len();
    }

    state[(current_pos + 1) % state.len()]
}

fn spin_lock2(steps: usize, count: usize) -> usize {
    let mut state = vec![0, 0];
    let mut current_pos = 0;

    for i in 1..=count {
        current_pos = (current_pos + steps) % i + 1;
        if current_pos == 1 {
            state[current_pos] = i;
        }
    }

    state[1]
}


#[cfg(test)]
mod day17_tests {
    use super::*;

    #[test]
    fn part_example() {
        assert_eq!(spin_lock(3, 2017), 638);
    }
}