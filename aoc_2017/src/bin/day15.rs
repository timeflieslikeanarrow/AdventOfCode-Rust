use std::fs;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref NUMBERS: Regex = Regex::new(r"(\d+)\D+(\d+)").unwrap();
}

fn main() {
    let input = fs::read_to_string("day15.input").unwrap();
    let captures = NUMBERS.captures(&input).unwrap();
    let seed_a = captures[1].parse().unwrap();
    let seed_b = captures[2].parse().unwrap();

    println!("part 1: {}", count_matches(seed_a, 1, seed_b, 1, 40_000_000));
    println!("part 2: {}", count_matches(seed_a, 4, seed_b, 8, 5_000_000));
}

struct Generator {
    number: usize,
    multiplier: usize,
    divisor: usize
}

impl Generator {
    fn new(seed: usize, multiplier: usize, divisor: usize) -> Self {
        Generator { number: seed, multiplier, divisor }
    }
}

impl Iterator for Generator {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let new_number = self.number * self.multiplier % 2147483647;
            self.number = new_number;
            if new_number % self.divisor == 0 {
                return Some(new_number);
            }
        }
    }
}

fn count_matches(seed_a: usize, divisor_a: usize, seed_b: usize, divisor_b: usize, pairs: usize) -> usize {
    let bit_pattern = 0xFFFF;

    let mut generator_a = Generator::new(seed_a, 16807, divisor_a);
    let mut generator_b = Generator::new(seed_b,  48271, divisor_b);

    let mut match_count = 0;
    for _ in 0..pairs {
        let a = generator_a.next().unwrap();
        let b = generator_b.next().unwrap();

        if a & & bit_pattern  == b & bit_pattern {
            match_count += 1;
        }
    }
    
    match_count
}

#[cfg(test)]
mod day15_tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(count_matches(65, 1, 8921, 1, 40_000_000), 588);
        assert_eq!(count_matches(65, 4, 8921, 8, 5_000_000), 309);
    }
}