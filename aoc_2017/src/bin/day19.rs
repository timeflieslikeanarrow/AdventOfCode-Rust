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
        
        x += dx;
        y += dy;

        if y < 0 || y >= max_y as i32 || x < 0 || x >= map[y as usize].len() as i32 {
            break;
        }

        let current = map[y as usize][x as usize];
        match current {
            ' ' => break, //the end
            c if c.is_alphabetic() => signs.push(c),
            '+' => {    
                let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];
                for (dx2, dy2) in neighbors {
                    if dx2 != dx && dy2 != dy && dx2 != -dx && dy2 != -dy && map[(y + dy2) as usize][(x + dx2) as usize] != ' ' {
                        dx = dx2;
                        dy = dy2;
                        break;
                    }
                }
            }
            _ => ()
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