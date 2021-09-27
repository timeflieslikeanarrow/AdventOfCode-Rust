use std::fs;

fn main() {
    let input = fs::read_to_string("day11.input").unwrap();
    let (distance, farthest) = steps(&input);
    println!("part 1: {}", distance);
    println!("part 2: {}", farthest);
}

fn how_far(mut x: i32, mut y: i32) -> i32 {
    x = x.abs();
    y = y.abs();

    let min = x.min(y);
    let max = x.max(y);

    min + (max - min) /2
}

fn steps(input: &str) -> (i32, i32) {
    let (x, y, farthest) = input.trim()
        .split(",")
        .fold((0i32, 0i32, 0i32), |(x, y, far), direction| {
            let (x, y) = match direction {
                "n" =>  (x, y + 2),
                "ne" => (x + 1, y + 1),
                "nw" => (x - 1, y + 1),
                "s" =>  (x, y - 2),
                "se" => (x + 1, y - 1),
                "sw" => (x - 1, y - 1),
                _ => unreachable!()
            };

            (x, y, far.max(how_far(x, y)))
        });

   (how_far(x, y), farthest)
}

#[cfg(test)]
mod day11_tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(steps("ne,ne,ne").0, 3);
        assert_eq!(steps("ne,ne,sw,sw").0, 0);
        assert_eq!(steps("ne,ne,s,s").0, 2);
        assert_eq!(steps("se,sw,se,sw,sw").0, 3);
    }
}