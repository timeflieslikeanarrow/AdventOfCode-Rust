use std::fs;
use aoc_2017::knot_hash::{knot_hash, reverse_and_skip};
fn main() {
    let input = fs::read_to_string("day10.input").unwrap();

    println!("part 1: {}", part1(&input, 255));
    println!("part 2: {}", part2(&input));
}

fn part1(input: &str, max_number: u8) -> usize {
    let lengths: Vec<u8> = input.trim().split(",").map(|s| s.parse().unwrap()).collect();
    let mut numbers: Vec<_> = (0..=max_number).collect();

    reverse_and_skip(&mut numbers, &lengths, 1);

    numbers[0] as usize * numbers[1] as usize
}

fn part2(input: &str) -> String {
    knot_hash(input)
}

#[cfg(test)]
mod day10_tests {
    use super::*;

    #[test]
    fn part1_examples() {
        assert_eq!(part1("3,4,1,5", 4), 12);
    }

    #[test]
    fn part2_tests() {
        assert_eq!(part2(""), "a2582a3a0e66e6e86e3812dcb672a272");
        assert_eq!(part2("AoC 2017"), "33efeb34ea91902bb2f59c9920caa6cd");
        assert_eq!(part2("1,2,3"), "3efbe78a8d82f29979031a4aa0b16a9d");
        assert_eq!(part2("1,2,4"), "63960835bcdc130f0b66d7ff4f6a5a8e");
    }
}

