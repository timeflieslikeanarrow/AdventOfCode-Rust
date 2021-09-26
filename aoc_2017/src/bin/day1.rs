use std::fs;
fn main() {
    let input = fs::read_to_string("day1.input").unwrap();
    let digits = parse(&input);
    println!("part 1: {}", decrypt_captcha(digits));
}

fn decrypt_captcha(mut digits: Vec<u32>) -> u32 {
    digits.push(digits[0]);

    digits.iter().zip(digits.iter().skip(1)).map(|(a, b)| 
        if a == b { *a } else { 0 }).sum()
}

fn parse(input: &str) -> Vec<u32> {
    input.trim().chars().map(|c| c.to_digit(10).unwrap()).collect()
}

#[cfg(test)]
mod day1_tests {
    use super::*;

    #[test] 
    fn examples() {
        assert_eq!(decrypt_captcha(vec![1,1,2,2]), 3);
        assert_eq!(decrypt_captcha(vec![1,1,1,1]), 4);
        assert_eq!(decrypt_captcha(vec![1,2,3,4]), 0);
        assert_eq!(decrypt_captcha(vec![9,1,2,1,2,1,2,9]), 9);
    }
}