fn main() {
    let input = 312051;
    println!("part 1: {}", spiral_memory_manhattan_distance(input));
}

fn spiral_memory_manhattan_distance(number: i32) -> i32 {
    if number == 1 {
        return 0;
    }

    let directions = [(1, 0), (0, 1), (-1, 0), (0, -1)];

    let mut i = 1;
    let (mut x, mut y, mut side) = (0i32, 0i32, 0u32);
    loop {
        for (n, (dx, dy)) in directions.iter().enumerate() {
            if n % 2 == 0 { //left and right
                side += 1;
            }

            for _ in 0..side {
                i += 1;
                x += dx;
                y += dy;

                if i == number {
                    return x.abs() + y.abs();
                }
            }    
        }
    }
}

fn spiral_memory_stored_value(number: i32) -> i32 {
    if number == 1 {
        return 1;
    }

    0
}

#[cfg(test)]
mod day3_tests {
    use super::*;

    #[test]
    fn part1_examples() {
        assert_eq!(spiral_memory_manhattan_distance(1), 0);
        assert_eq!(spiral_memory_manhattan_distance(12), 3);
        assert_eq!(spiral_memory_manhattan_distance(23), 2);
        assert_eq!(spiral_memory_manhattan_distance(1024), 31);    
    }

    #[test]
    fn part2_examples() {
        assert_eq!(spiral_memory_stored_value(1), 1);
        assert_eq!(spiral_memory_stored_value(2), 1);
        assert_eq!(spiral_memory_stored_value(3), 2);
        assert_eq!(spiral_memory_stored_value(4), 4);    
        assert_eq!(spiral_memory_stored_value(5), 5 ); 
        assert_eq!(spiral_memory_stored_value(6), 10 ); 
        assert_eq!(spiral_memory_stored_value(7), 11 ); 
        assert_eq!(spiral_memory_stored_value(8), 23 ); 
        assert_eq!(spiral_memory_stored_value(9), 25 ); 
        assert_eq!(spiral_memory_stored_value(23), 806 ); 
    }
}