use std::fs;
fn main() {
    let input = fs::read_to_string("day5.input").unwrap();
    let mut instructions = parse(&input);
    println!("part 1: {}", escape_maze(&mut instructions));
}

fn escape_maze(instructions: &mut Vec<i32>) -> usize {
    let len = instructions.len() as i32;
    let mut ip = 0;
    let mut steps = 0;
    
    while 0 <= ip && ip < len {
        steps += 1;

        let old_ip = ip as usize;
        ip += instructions[old_ip];
        instructions[old_ip] += 1;
    }

    steps
}

fn parse(input: &str) -> Vec<i32> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[cfg(test)]
mod day5_tests {
    use super::*;

    #[test]
    fn part1_examples() {
       let mut instructions = vec![0, 3, 0, 1, -3];
       let steps = escape_maze(&mut instructions);
       assert_eq!(steps, 5);
       assert_eq!(instructions, vec![2,5,0,1,-2]);
    }
}