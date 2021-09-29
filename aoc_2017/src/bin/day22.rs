use std::fs;
use std::collections::HashMap;
use std::iter::FromIterator;
use std::time::Instant;

fn main() {
    let input = fs::read_to_string("day22.input").unwrap();
    println!("part 1: {}", part1(&input, 10000));
    println!("part 2: {}", part2(&input, 10000000));
}

fn part1(input: &str, burst: usize) -> usize {
    bursts_with_infection(input, burst, burst_logic_part1)
}

fn part2(input: &str, burst: usize) -> usize {
    bursts_with_infection(input, burst, burst_logic_part2)
}

const INFECTED: char = '#';
const CLEAN:    char = '.';
const FLAGGED:  char = '*';
const WEAKENED: char = '$';

fn parse(input: &str) -> (usize, HashMap<(i32, i32), char>) {
    (input.trim().lines().count(), HashMap::from_iter( 
        input.trim().lines()
            .zip(0..)
            .flat_map(|(line, row)| {
                line.chars()
                    .zip(0..)
                    .map(|(c, col)| ((col, row), c))
                    .collect::<Vec<_>>()
            }))
    )
}

fn turn_left(dx: &mut i32, dy: &mut i32) {
    //[0   1]
    //[-1  0]
    let temp = *dx;
    *dx = *dy;
    *dy = -temp;
}

fn turn_right(dx: &mut i32, dy: &mut i32) {
    //[0 -1]
    //[1  0]
    let temp = *dx;
    *dx = -*dy;
    *dy = temp;
}

fn burst_logic_part1(x: i32, y: i32, dx: &mut i32, dy: &mut i32, map: &mut HashMap<(i32, i32), char>) -> bool {
    if !map.contains_key(&(x, y)) || map[&(x,y)] == CLEAN {
        turn_left(dx, dy);
        map.insert((x, y), INFECTED);
        true
    } else  { //map[&(x,y)] == INFECTED
        turn_right(dx, dy);
        map.remove(&(x, y));
        false
    }
}

fn burst_logic_part2(x: i32, y: i32, dx: &mut i32, dy: &mut i32, map: &mut HashMap<(i32, i32), char>) -> bool {
    if !map.contains_key(&(x, y)) || map[&(x,y)] == CLEAN {
        turn_left(dx, dy);
        map.insert((x, y), WEAKENED);
    } else if map[&(x,y)] == WEAKENED {
        map.insert((x, y), INFECTED);
        return true;
    } else if map[&(x,y)] == FLAGGED {
        *dx = -*dx;
        *dy = -*dy;
        map.insert((x, y), CLEAN);
    } else  { //map[&(x,y)] == INFECTED
        turn_right(dx, dy);
        map.insert((x, y), FLAGGED);
    }

    false
}

fn bursts_with_infection(input: &str, burst: usize, burst_logic: fn(i32, i32, &mut i32, &mut i32, &mut HashMap<(i32, i32), char>) -> bool) -> usize {
    let start_time = Instant::now();
    let (height, mut map) = parse(input);
    let center = (height / 2) as i32;

    let mut infected_count = 0;
    let mut x = center;
    let mut y = center;

    let mut dx = 0;
    let mut dy = -1;

    //println!("{} {}", x, y);
    //println!("{:?}", map);
    for _ in 0..burst {
        
        if burst_logic(x, y, &mut dx, &mut dy, &mut map) {
            infected_count += 1;
        }

        x += dx;
        y += dy;

        //println!("({}, {}) {:?}", x, y, map);
    }

    println!("running time: {}", start_time.elapsed().as_secs());

    infected_count
}

#[cfg(test)]
mod day22_tests {
    use super::*;

    #[test]
    fn part1_examples() {
        let input = "\
..#
#..
...";
        assert_eq!(part1(input, 7), 5);
        assert_eq!(part1(input, 10000), 5587);
    }

    #[test]
    fn part2_examples() {
        let input = "\
..#
#..
...";
        assert_eq!(part2(input, 100), 26);
        assert_eq!(part2(input, 10000000), 2511944);
    }
}