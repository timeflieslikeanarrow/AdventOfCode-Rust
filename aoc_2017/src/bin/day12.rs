use std::fs;
use std::collections::{HashMap, HashSet};

fn main() {
    let input = fs::read_to_string("day12.input").unwrap();
    let graph = parse_graph(&input);
    println!("part 1: {}", groups_with(&graph, 0));
    println!("part 2: {}", total_groups(&graph));
}


fn parse_graph(input: &str) -> HashMap<usize, Vec<usize>> {
    let mut graph = HashMap::new();

    for line in input.trim().lines() {
        let parts: Vec<_> = line.split(" <-> ").collect();
        let node: usize = parts[0].trim().parse().unwrap();
        let neigbors: Vec<_> = parts[1].trim().split(", ").map(|s| s.parse().unwrap()).collect();

        for neighbor in neigbors {
            let entry = graph.entry(node).or_insert(vec![]);
            entry.push(neighbor);

            let entry = graph.entry(neighbor).or_insert(vec![]);
            entry.push(node);
        }
    }
    
    graph
}

fn groups_with(graph: &HashMap<usize, Vec<usize>>, root: usize) -> usize {
    let mut visited = HashSet::new();
    visited.insert(root);

    dfs(&graph, root, &mut visited);
    visited.len()
}

fn total_groups(graph: &HashMap<usize, Vec<usize>>) -> usize {
    let mut visited = HashSet::new();
    let mut groups = 0;

    for (node, _) in graph.iter() {
        if !visited.contains(node) {
            visited.insert(*node);
            dfs(&graph, *node, &mut visited);
            groups += 1;
        }
    }

    groups
}

fn dfs(graph: &HashMap<usize, Vec<usize>>, root: usize, visited: &mut HashSet<usize>) {
    for neighbor in graph.get(&root).unwrap_or(&vec![]) {
        if !visited.contains(neighbor) {
            visited.insert(*neighbor);
            dfs(graph, *neighbor, visited);
        }
    }
}


#[cfg(test)]
mod day12_tests {
    use super::*;

    #[test]
    fn examples() {
        let input = "\
0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5";

        let graph = parse_graph(input);

        assert_eq!(groups_with(&graph, 0), 6);
        assert_eq!(total_groups(&graph), 2);
    }
}