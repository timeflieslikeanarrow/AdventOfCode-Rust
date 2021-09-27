use std::fs;

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

fn reverse_and_skip(numbers: &mut Vec<u8>, lengths: &Vec<u8>, round: usize){
    let n = numbers.len();
    let mut current_position = 0usize;
    let mut skip_size = 0usize;

    for _ in 0..round {
        for length in lengths {
            //reverse current_position until current_position + length
            let length = *length as usize;
            let mut i = current_position;
            let mut j = current_position + length - 1;
            while i < j {
                numbers.swap(i % n, j % n);
                i += 1;
                j -= 1;
            }

            current_position = (current_position + length + skip_size) % n;
            skip_size += 1;
        }
    }
}

fn numbers_to_ascii(input: &str) -> Vec<u8> {
    input.as_bytes().into_iter().map(|b| *b).collect()
}

fn part2(input: &str) -> String {
    let mut lengths = numbers_to_ascii(input);
    lengths.extend_from_slice(&[17,31,73,47,23]);

    let mut numbers: Vec<_> = (0..=255u8).collect();

    reverse_and_skip(&mut numbers, &lengths, 64);

    let result: Vec<_> = (0..256).step_by(16)
            .map(|start| format!("{:02x}", compute_hash(&numbers, start, 16)))
            .collect();

    result.join("")
}

fn compute_hash(numbers: &Vec<u8>, start: usize, length: usize) -> u8 {
    numbers.iter().skip(start).take(length).fold(0, |result, n| result ^ *n)
}

#[cfg(test)]
mod day10_tests {
    use super::*;

    #[test]
    fn part1_examples() {
        assert_eq!(part1("3,4,1,5", 4), 12);
    }

    #[test]
    fn numbers_to_ascii_tests() {
        let input = "1,2,3";
        assert_eq!(numbers_to_ascii(input), vec![49,44,50,44,51]);
    }

    #[test]
    fn computert_hash_tests() {
        let numbers = vec![65,27,9,1,4,3,40,50,91,7,6,0,2,5,68,22];
        assert_eq!(compute_hash(&numbers, 0, 16), 64);
    }

    #[test]
    fn part2_tests() {
        assert_eq!(part2(""), "a2582a3a0e66e6e86e3812dcb672a272");
        assert_eq!(part2("AoC 2017"), "33efeb34ea91902bb2f59c9920caa6cd");
        assert_eq!(part2("1,2,3"), "3efbe78a8d82f29979031a4aa0b16a9d");
        assert_eq!(part2("1,2,4"), "63960835bcdc130f0b66d7ff4f6a5a8e");
    }
}

