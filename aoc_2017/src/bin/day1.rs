use std::fs;
fn main() {
    let input = fs::read_to_string("day1.input").unwrap();
    let digits = parse(&input);
    println!("part 1: {}", decrypt_captcha(&digits, 1));
    println!("part 2: {}", decrypt_captcha(&digits, digits.len() / 2));
}

fn decrypt_captcha(digits: &Vec<u32>, distance: usize) -> u32 {
    let n = digits.len();
    (0..n).map(|i|
        if digits[i] == digits[(i + distance) % n] { digits[i] } else { 0 }
    ).sum()
}

fn parse(input: &str) -> Vec<u32> {
    input.trim().chars().map(|c| c.to_digit(10).unwrap()).collect()
}

#[cfg(test)]
mod day1_tests {
    use super::*;

    #[test] 
    fn part1_examples() {
        assert_eq!(decrypt_captcha(&vec![1,1,2,2], 1), 3);
        assert_eq!(decrypt_captcha(&vec![1,1,1,1], 1), 4);
        assert_eq!(decrypt_captcha(&vec![1,2,3,4], 1), 0);
        assert_eq!(decrypt_captcha(&vec![9,1,2,1,2,1,2,9], 1), 9);
    }

    #[test] 
    fn part2_examples() {
        assert_eq!(decrypt_captcha(&vec![1,2,1,2], 2), 6);
        assert_eq!(decrypt_captcha(&vec![1,2,2,1], 2), 0);
        assert_eq!(decrypt_captcha(&vec![1,2,3,4,2,5], 3), 4);
        assert_eq!(decrypt_captcha(&vec![1,2,3,1,2,3], 3), 12);
        assert_eq!(decrypt_captcha(&vec![1,2,1,3,1,4,1,5], 4), 4);
    }
}