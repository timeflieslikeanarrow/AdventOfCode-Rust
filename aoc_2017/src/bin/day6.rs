use std::fs;
use std::collections::HashSet;

fn main() {
    let input = fs::read_to_string("day6.input").unwrap();
    let mut banks = parse(&input);
    println!("part 1: {}", distribution_cycle(&mut banks, false));
    println!("part 2: {}", distribution_cycle(&mut banks, true));
}

fn parse(input: &str) -> Vec<usize> {
    input.split_whitespace().map(|s| s.parse().unwrap()).collect()
}

fn distribution_cycle(banks: &mut Vec<usize>, use_start_state: bool) -> usize {
    let n = banks.len();
    let mut cycles = 0;
    let mut states = HashSet::new();
    
    let start_state = banks.clone();

    states.insert(banks.clone());

    loop {
        cycles += 1;

        //find max
        let mut max = *&banks[0];
        let mut max_index = 0;
        for i in 1..n {
            if banks[i] > max {
                max = banks[i];
                max_index = i;
            }
        }

        banks[max_index] = 0;
        for i in max_index + 1 ..= max_index + max {
            banks[i % n] += 1;
        }

        let new_state = banks.clone();
        if use_start_state && new_state == start_state {
                return cycles;
        } else if states.contains(&new_state) {
                return cycles;
        }

        states.insert(new_state);
    }

}

#[cfg(test)]
mod day6_tests {
    use super::*;

    #[test]
    fn examples() {
        let mut banks = vec![0,2,7,0];
        let cycle = distribution_cycle(&mut banks, false);

        //part 1
        assert_eq!(cycle, 5);
        assert_eq!(banks, vec![2,4,1,2]);

        //part 2
        let cycle2 = distribution_cycle(&mut banks, true);
        assert_eq!(cycle2, 4);
    }
}