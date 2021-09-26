use std::collections::HashMap;

fn main() {
    let input = 312051;
    println!("part 1: {}", spiral_memory_manhattan_distance(input));
    println!("part 2: {}", spiral_memory_stored_value(input, false));
}

//LEFT UP RIGHT DOWN
const DIRECTIONS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];  
const NEIGHBORS:  [(i32, i32); 8] = [(-1, 1),  (0, 1),  (1, 1), 
                                     (-1, 0),           (1, 0),
                                     (-1, -1), (0, -1), (1, -1)];

#[derive(Debug)]
struct SpiralMemory {
    prev_number: i32,
    direction_index: usize,
    side: usize,
    side_index: usize,
    position: (i32, i32)
}

//Poor man's yield
impl Iterator for SpiralMemory {
    type Item = (i32, i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        let (dx, dy) = DIRECTIONS[self.direction_index];

        self.prev_number += 1;
        self.position.0 += dx;
        self.position.1 += dy;

        self.side_index += 1;
        if self.side_index == self.side {
            self.direction_index = (self.direction_index + 1) % DIRECTIONS.len();
            if self.direction_index % 2 == 0 {
                self.side += 1;
            }

            self.side_index = 0;
        }

        //println!("{:?}", self);
        Some((self.prev_number, self.position.0, self.position.1))
    }
}


fn spiral_memory_manhattan_distance(number: i32) -> i32 {
    if number == 1 {
        return 0;
    }

    let spiral_memory = SpiralMemory {
        prev_number: 1,
        direction_index: 0,
        side: 1,
        side_index: 0,
        position: (0, 0),
    };

   let (_, x, y) = spiral_memory.into_iter().filter(|(i, _, _)| *i == number).next().unwrap();
   x.abs() + y.abs()
}

fn spiral_memory_stored_value(number: i32, sequence: bool) -> i32 {
    if number == 1 {
        return 1;
    }

    let mut values = HashMap::new();
    values.insert((0, 0), 1);

    let spiral_memory = SpiralMemory {
        prev_number: 1,
        direction_index: 0,
        side: 1,
        side_index: 0,
        position: (0, 0),
    };

    spiral_memory.into_iter().map(|(i, x, y)| {
        let value = NEIGHBORS.iter()
                            .map(|(dx, dy)|
                                values.get(&(x + dx, y + dy)).unwrap_or(&0)
                            ).sum();
        values.insert((x, y), value);

        (i, value)
    }).filter(|(i, value)| if sequence { *i == number } else { *value > number}).next().unwrap().1
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
        assert_eq!(spiral_memory_stored_value(1, true), 1);
        assert_eq!(spiral_memory_stored_value(2, true), 1);
        assert_eq!(spiral_memory_stored_value(3, true), 2);
        assert_eq!(spiral_memory_stored_value(4, true), 4);    
        assert_eq!(spiral_memory_stored_value(5, true), 5 ); 
        assert_eq!(spiral_memory_stored_value(6, true), 10 ); 
        assert_eq!(spiral_memory_stored_value(7, true), 11 ); 
        assert_eq!(spiral_memory_stored_value(8, true), 23 ); 
        assert_eq!(spiral_memory_stored_value(9, true), 25 ); 
        assert_eq!(spiral_memory_stored_value(23, true), 806 ); 
    }
}