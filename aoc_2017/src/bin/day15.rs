use std::fs;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref NUMBERS: Regex = Regex::new(r"(\d+)\D+(\d+)").unwrap();
}

fn main() {
    let input = fs::read_to_string("day15.input").unwrap();
    println!("part 1: {}", part1(&input));
}

fn part1(input: &str) -> usize {
    let cpatures = NUMBERS.captures(input).unwrap();
    let seed_a = cpatures[1].parse().unwrap();
    let seed_b = cpatures[2].parse().unwrap();

    //println!("{} {}",seed_a, seed_b);
    count_matches(seed_a, seed_b, 40_000_000)
}

fn count_matches(seed_a: usize, seed_b: usize, pairs: usize) -> usize {
    let m = 2147483647;
    let bit_pattern = 0xFFFF;
    let (multiplier_a, multiplier_b) = (16807, 48271);
    let (mut a, mut b) = (seed_a, seed_b);

    let mut match_count = 0;
    for _ in 0..pairs {
        a = a * multiplier_a % m;
        b = b * multiplier_b % m;

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
    fn part1_examples() {
        assert_eq!(count_matches(65, 8921, 40_000_000), 588);
    }
}