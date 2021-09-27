use std::fs;
use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref LINE: Regex = Regex::new(r"(\w+) (\w+) ([-]?\d+) if (\w+) (\S+) ([-]?\d+)").unwrap();
}

fn main() {
    let input = fs::read_to_string("day8.input").unwrap();
    let (result, highest) = run(&input);
    println!("part 1: {}", result);
    println!("part 2: {}", highest);
}

fn run(input: &str) -> (i32, i32) {
    let mut highest = i32::MIN;
    let mut registers: HashMap<String, i32> = HashMap::new();

    input.lines().for_each(|line| {
        //println!("**{}**", line);
        let caps = LINE.captures(line).unwrap();
        //println!("{:?}", caps);
        let condition_register = caps[4].to_string();
        let condition_register_value = *registers.get(&condition_register).unwrap_or(&0);
        let condition_value: i32 = caps[6].parse().unwrap();
        let condition = match &caps[5] {
            "<" => condition_register_value < condition_value,
            "<=" => condition_register_value <= condition_value,
            ">" => condition_register_value > condition_value,
            ">=" => condition_register_value >= condition_value,
            "==" => condition_register_value == condition_value,
            "!=" => condition_register_value != condition_value,
            _ => unreachable!()
        };

        if condition {
            let register = caps[1].to_string();
            let original_value = *registers.get(&register).unwrap_or(&0);
            let value: i32 = caps[3].parse().unwrap();
            
            let new_value =  match &caps[2] {
                "dec" => original_value - value,
                "inc" => original_value + value,
                _ => unreachable!()
            };

            registers.insert(register, new_value);
            if new_value > highest {
                highest = new_value;
            }
        }
    });

    (*registers.values().max().unwrap(), highest)
}

#[cfg(test)]
mod day8_tests {
    use super::*;

    #[test]
    fn examples() {
        let input = "\
b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10";

        let (result, highest) = run(input);
        assert_eq!(result, 1);
        assert_eq!(highest, 10);
    }
}