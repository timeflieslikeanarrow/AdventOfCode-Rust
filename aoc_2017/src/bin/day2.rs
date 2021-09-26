use std::fs;
fn main() {
    let input = fs::read_to_string("day2.input").unwrap();
    let spreadsheet = parse(&input);
    println!("part 1: {}", checksum(&spreadsheet));
    println!("part 2: {}", even_divisible_sum(&spreadsheet));
}

fn checksum(spreadsheet: &Vec<Vec<i32>>) -> i32 {
    spreadsheet
        .iter()
        .map(|row| row.iter().max().unwrap() - row.iter().min().unwrap())
        .sum()
}

fn even_divisible_sum(spreadsheet: &Vec<Vec<i32>>) -> i32 {
    spreadsheet.iter()
        .map(|row| even_divisible(row))
        .sum()
}

fn even_divisible(row: &Vec<i32>) -> i32 {
    for a in row {
        for b in row {
            if a > b && a / b * b == *a {
                return a / b;
            }
        }
    }

    unreachable!("No such numbers!");
}

fn parse(input: &str) -> Vec<Vec<i32>> {
    input.lines().map(|line| 
        line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect()
    ).collect()
}

#[cfg(test)]
mod day2_tests {
    use super::*;

    #[test]
    fn part1_examples() {
        assert_eq!(checksum(&vec![
            vec![5,1,9,5], 
            vec![7,5,3],
            vec![2,4,6,8]
        ]), 8 + 4 + 6);
    }

    #[test]
    fn part2_examples() {
        assert_eq!(even_divisible_sum(&vec![
            vec![5,9,2,8], 
            vec![9,4,7,3],
            vec![3,8,6,5]
        ]), 4 + 3 + 2);
    }
}