use std::fs;
fn main() {
    let input = fs::read_to_string("day5.input").unwrap();
    let mut instructions = parse(&input);
    let mut instructions2 = instructions.clone();
    println!("part 1: {}", escape_maze(&mut instructions, increment_rule_part1));
    println!("part 2: {}", escape_maze(&mut instructions2, increment_rule_part2));
}

fn escape_maze(instructions: &mut Vec<i32>, increment_rule: fn(i32)->i32) -> usize {
    let len = instructions.len() as i32;
    let mut ip = 0;
    let mut steps = 0;
    
    while 0 <= ip && ip < len {
        steps += 1;

        let old_ip = ip as usize;
        ip += instructions[old_ip];
        instructions[old_ip] += increment_rule(instructions[old_ip]);
    }

    steps
}

fn parse(input: &str) -> Vec<i32> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn increment_rule_part1(_jump: i32) -> i32 {
    1
}

fn increment_rule_part2(jump: i32) -> i32 {
    if jump >= 3 {
        -1
    } else {
        1
    }
}


#[cfg(test)]
mod day5_tests {
    use super::*;

    #[test]
    fn part1_examples() {
       let mut instructions = vec![0, 3, 0, 1, -3];
       let steps = escape_maze(&mut instructions, increment_rule_part1);
       assert_eq!(steps, 5);
       assert_eq!(instructions, vec![2,5,0,1,-2]);
    }

    #[test]
    fn part2_examples() {
       let mut instructions = vec![0, 3, 0, 1, -3];
       let steps = escape_maze(&mut instructions, increment_rule_part2);
       assert_eq!(steps, 10);
       assert_eq!(instructions, vec![2, 3, 2, 3, -1]);
    }
}