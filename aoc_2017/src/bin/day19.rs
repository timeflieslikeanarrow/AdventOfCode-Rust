use std::fs;
use std::iter::FromIterator;
fn main() {
    let input = fs::read_to_string("day19.input").unwrap();
    let (signs, steps) = route(&input);
    println!("part 1: {}", signs);
    println!("part 2: {}", steps);
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().filter(|line| !line.is_empty()).map(|line| line.chars().collect()).collect()
}

fn route(input: &str) -> (String, usize) {
    let map = parse(input);

    let mut signs: Vec<char> = Vec::new();
    let mut steps = 0;

    let max_y = map.len();
    let mut y = 0;
    let mut x = map[0].iter().position(|c| *c == '|').unwrap() as i32; 
    //println!("starting at {:?}", (x, y));

    let mut dy: i32 = 1;
    let mut dx: i32 = 0;
    loop {
        steps += 1;

        // if steps % 10_000 == 0 {
        //     println!("steps so far: {} ({}, {}) **{}**", steps, x, y, map[y as usize][x as usize]);
        //     println!("signs: {:?}",signs);
        // }

        x += dx;
        y += dy;

        if y < 0 || y >= max_y as i32 || x < 0 || x >= map[y as usize].len() as i32 {
            break;
        }

        let x = x as usize;
        let y = y as usize;

        let current = map[y][x];
        match current {
            '+' => {    
                if dx == 0 { //vertical
                    if x > 0 && map[y][x - 1] != ' ' {
                        dx = -1;
                    } else if x + 1 < map[y].len() && map[y][x + 1] != ' ' {
                        dx = 1;
                    } else {
                        panic!("don't know where to go: ({}, {})", x, y);
                    }

                    dy = 0;
                } else {  //dy == 0 horizontal
                    if y > 0 && map[y - 1][x] != ' ' {
                        dy = -1;
                    } else if y + 1 < max_y && map[y + 1][x] != ' ' {
                        dy = 1;
                    } else {
                        panic!("don't know where to go: ({}, {})", x, y);
                    }

                    dx = 0;
                }
            }
            '-' => {}
            '|' => {}
            ' ' => break, //the end
            _ => { signs.push(current) }
        }
    }

    (String::from_iter(signs), steps)
}

#[cfg(test)]
mod day19_tests {
    use super::*;

    #[test]
    fn examples() {
        let input = "
    |          
    |  +--+    
    A  |  C    
F---|----E|--+ 
    |  |  |  D 
    +B-+  +--+ ";

        let (signs, steps) = route(input);
        assert_eq!(signs, "ABCDEF");
        assert_eq!(steps, 38);
    }
}