use std::collections::HashSet;

use aoc_2017::knot_hash::knot_hash_u8;

fn main() {
    println!("part 1: {}", part1("ugkiagan"));
    println!("part 2: {}", part2("ugkiagan"));
}

fn part1(seed: &str) -> usize {
    let grid = generate_grid(seed);

    grid.iter().map(|v| v.iter().filter(|c| **c == '1').count()).sum()
}

fn part2(seed: &str) -> usize {
    let grid = generate_grid(seed);

    count_region(&grid)
}

fn dfs(row: i32, col: i32, max_row: i32, max_col: i32, grid: &Vec<Vec<char>>, visited: &mut HashSet<(i32, i32)>) {
    let neigbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    for (dr, dc) in neigbors {
        let (col, row) = (col + dc, row + dr);
        if col >= 0 && col < max_col && row >= 0 && row < max_row 
            && grid[row as usize][col as usize] == '1' 
            && !visited.contains(&(row, col)) {
            visited.insert((row, col));
            dfs(row, col, max_row, max_col, grid, visited);
        }
    }
}

fn count_region(grid: & Vec<Vec<char>>) -> usize {
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;

    let mut visited = HashSet::new();
    let mut regions = 0;

    for row in 0..rows {
        for col in 0..cols {
            if grid[row as usize][col as usize] == '1' && !visited.contains(&(row, col)) {
                visited.insert((row, col));

                dfs(row , col , rows, cols, &grid, &mut visited);
                regions += 1;
            }
        }
    }

    regions
}

fn byte_to_bits(b: u8) -> String {
    format!("{:08b}", b)
}

fn generate_grid(seed: &str) -> Vec<Vec<char>> {
    (0..128).map(|row| {
        let row : Vec<_> = knot_hash_u8(&format!("{}-{}", seed, row)).iter()
                .map(|b| byte_to_bits(*b))
                .collect();
        row.join("").chars().collect()
    }).collect()
}

#[cfg(test)]
mod day14_tests {
    use super::*;

    #[test]
    fn hex_to_bits_tests() {
        assert_eq!(u8::from_str_radix("a", 16).unwrap(), 10);
        assert_eq!(format!("{:04b}", 0), "0000");
        assert_eq!(format!("{:04b}", 1), "0001");
        assert_eq!(format!("{:04b}", 10), "1010");

        assert_eq!(byte_to_bits(4), "00000100");
        assert_eq!(byte_to_bits(11), "00001011");
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1("flqrgnkx"),  8108);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2("flqrgnkx"),  1242);
    }

}