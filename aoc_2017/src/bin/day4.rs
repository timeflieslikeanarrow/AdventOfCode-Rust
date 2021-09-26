use std::fs;
use std::collections::HashSet;
use std::iter::FromIterator;

fn main() {
    let input = fs::read_to_string("day4.input").unwrap();
    println!("part 1: {}", part1(&input));
    println!("part 2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    input.lines().filter(|line| valid_passphrase(line)).count()
}

fn part2(input: &str) -> usize {
    input.lines().filter(|line| valid_passphrase2(line)).count()
}

fn valid_passphrase(input: &str) -> bool {
    let phrases: Vec<_> = input.trim().split_whitespace().collect();
    phrases.len() == HashSet::<&&str>::from_iter(phrases.iter()).len()
}

fn valid_passphrase2(input: &str) -> bool {
    let phrases: Vec<_> = input.trim().split_whitespace().map(|line| {
        let mut chars: Vec<_> = line.chars().collect();
        chars.sort();
        String::from_iter(chars)
    }).collect();

    phrases.len() == HashSet::<&String>::from_iter(phrases.iter()).len()
}

#[cfg(test)]
mod day4_tests {
    use super::*;

    #[test]
    fn part1_examples() {
        assert!(valid_passphrase("aa bb cc dd ee"));
        assert!(!valid_passphrase("aa bb cc dd aa"));
        assert!(valid_passphrase("aa bb cc dd aaa"));
    }

    #[test]
    fn part2_examples() {
        assert!(valid_passphrase2("abcde fghij"));
        assert!(!valid_passphrase2("abcde xyz ecdab"));
        assert!(valid_passphrase2("a ab abc abd abf abj"));
        assert!(valid_passphrase2("iiii oiii ooii oooi oooo"));
        assert!(!valid_passphrase2("oiii ioii iioi iiio"));
    }
}

