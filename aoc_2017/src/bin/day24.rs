use std::fs;
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let input = fs::read_to_string("day24.input").unwrap();
    let (max_length, longest_strength) = bridge(&input);
    println!("part 1: {}", max_length);
    println!("part 1: {}", longest_strength);
}

fn parse(input: &str) -> HashMap<usize, Vec<(usize, usize)>> {
    let components: Vec<(usize, usize)> = input.trim().lines().map(|line| {
        let parts: Vec<_> = line.split('/').collect();
        let left = parts[0].parse().unwrap();
        let right = parts[1].parse().unwrap();

        (left, right)
    }).collect();

    let mut graph = HashMap::new();
    for (index, (left, right)) in components.iter().enumerate() {
        let entry = graph.entry(*left).or_insert(vec![]);
        entry.push((*right, index));
        let entry = graph.entry(*right).or_insert(vec![]);
        entry.push((*left, index));
    }

    graph
}

fn bridge(input: &str) -> (usize, usize) {
    let mut strengths: HashMap<usize, usize> = HashMap::new();

    let components = parse(input);
    let mut queue = VecDeque::new();

    for (port, index) in components[&0].iter() {
        let mut visited = HashSet::new();
        visited.insert(*index);
        queue.push_back((*port, *port, 1, visited));
    }

    while !queue.is_empty() {
        //println!("{:?}", queue);
        let (from_port, strength, length, visited) = queue.pop_front().unwrap();
        
        let entry = strengths.entry(length).or_insert(0);
        if strength > *entry {
            *entry = strength;
        }

        if components.contains_key(&from_port) {
            for (to_port, index) in components[&from_port].iter() {
                if !visited.contains(index) {
                    let mut new_visited = visited.clone();
                    new_visited.insert(*index);
                    queue.push_back((*to_port, strength + from_port + to_port, length + 1, new_visited));
                }
            }
        } 
    }

    let longest_key = strengths.keys().max().unwrap();
    (*strengths.values().max().unwrap(), strengths[longest_key])
}


#[cfg(test)]
mod day24_tests {
    use super::*;

    #[test]
    fn example() {
        let input = "\
0/2
2/2
2/3
3/4
3/5
0/1
10/1
9/10";

        assert_eq!(bridge(input), (31, 19));
        
    }
}