use std::fs;

fn main() {
    let input = fs::read_to_string("day13.input").unwrap();
    println!("part 1: {}", part1(&input));
    println!("part 2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    let firewall = parse(input);
    trip_severity(&firewall)
}

fn part2(input: &str) -> usize {
    let firewall = parse(input);
    (0..).filter(|delay| 
        firewall.iter().all(|(depth, range)| !caught(*depth, *range, *delay))
    ).next().unwrap()
}

fn parse(input:&str) -> Vec<(usize, usize)> {
    input.trim().lines().map(|line| {
        let parts: Vec<_> = line.split(": ").collect();
        let depth: usize = parts[0].parse().unwrap();
        let range = parts[1].parse().unwrap();

        (depth, range)
    }).collect()
}

fn caught(depth: usize, range: usize, delay: usize) -> bool {
    (delay + depth) % (2 * (range - 1)) == 0
}

fn trip_severity(firewall: &Vec<(usize, usize)>) -> usize {
    firewall.iter().map(|(depth, range)| 
        if caught(*depth, *range, 0) { depth * range } else { 0 }
    ).sum()
}

#[cfg(test)]
mod day13_tests {
    use super::*;

    #[test]
    fn trip_severity_test() {
        let input = "\
0: 3
1: 2
4: 4
6: 4";
        let firewall = parse(input);
        assert_eq!(firewall, vec![(0,3), (1,2), (4,4), (6,4)]);
        assert_eq!(trip_severity(&firewall), 24);
    }

    #[test]
    fn part2_test() {
        let input = "\
0: 3
1: 2
4: 4
6: 4";
        assert_eq!(part2(input), 10);
    }

}