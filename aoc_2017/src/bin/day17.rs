fn main() {
    println!("part 1: {}", spin_lock(328));
}

fn spin_lock(steps: usize) -> usize {
    let mut state = vec![0];
    let mut current_pos = 0;
    for i in 1..=2017 {
        current_pos = (current_pos + steps) % state.len();
        state.insert(current_pos + 1, i);
        current_pos  = (current_pos + 1) % state.len();
    }

    state[(current_pos + 1) % state.len()]
}

#[cfg(test)]
mod day17_tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(spin_lock(3), 638);
    }
}