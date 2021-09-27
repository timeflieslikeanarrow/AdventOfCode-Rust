use std::fs;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref PROGRAM_LINE: Regex = Regex::new(r"(\w+)\s+\((\d+)\)").unwrap();
}

fn main() {
    let input = fs::read_to_string("day7.input").unwrap();
    println!("part 1: {}", part1(&input));
    println!("part 2: {}", part2(&input));
}

#[derive(Debug)]
struct Program {
    name: String,
    weight: usize,
    children: Vec<String>
}

fn part1(input: &str) -> String {
    let programs = parse(input);
    find_root(&programs)
}

fn parse_line(line: &str) -> Program {
    let parts: Vec<_> = line.trim().split(" -> ").collect();
    let caps = PROGRAM_LINE.captures_iter(parts[0]).next().unwrap();
    let name = caps[1].to_owned();
    let weight = *&caps[2].parse().unwrap();
    let mut children = Vec::new();
    if parts.len() > 1 {
        children = parts[1].trim().split(", ").map(|s| s.to_string()).collect();
    } 

    Program { 
        name, 
        weight, 
        children
    }
}

fn parse(input: &str) -> HashMap<String, Program> {
    HashMap::from_iter(
        input.lines().map(|line| {
            let p = parse_line(line);
            (p.name.clone(), p)
        })
    )
}

fn find_root(programs: &HashMap<String, Program>) -> String {
    let all_names: HashSet<_> = programs.keys().collect();

    let children_names: HashSet<_> = programs.iter()
            .flat_map(|(_, program)| program.children.iter())
            .collect();

    (&all_names - &children_names).iter().next().unwrap().to_string()
}

fn part2(input: &str) -> usize {
    let programs = parse(input);
    let weights: HashMap<String, usize> = programs.iter()
            .map(|(k, p)| (k.to_string(), compute_weights(p, &programs)))
            .collect();
    let siblings: HashMap<String, Vec<String>> = compute_siblings(&programs);

    let unbalanced = find_unbalanced(&programs, &weights);
    let the_one = unbalanced.iter().filter(|p| children_balanced(p, &weights)).next().unwrap();
    let siblings = &siblings[&the_one.name];

    the_one.weight - (weights[&the_one.name] - weights[&siblings[0]])
}

fn compute_weights(parent: &Program, programs: &HashMap<String, Program>) -> usize {
    let child_weights: usize = parent.children.iter().map(|child| {
        let child = programs.get(child).unwrap();
        compute_weights(child, programs)
    }).sum();

    parent.weight + child_weights
}

fn compute_siblings(programs: &HashMap<String, Program>) -> HashMap<String, Vec<String>> {
    programs.values().flat_map(|p|
        p.children.iter()
            .map(move |c| 
                (c.to_string(), 
                 p.children.iter()
                        .filter(|c2| *c2 != c)
                        .map(|c2| c2.to_string()).collect()))
    ).collect()
}

fn unbalanced_weights<'a>(children: &'a Vec<String>,  programs: &'a HashMap<String, Program>, weights: &HashMap<String, usize>) -> Vec<&'a Program> {
    let mut children_weights = HashMap::new();

    for child in children {
        let weight = weights[child];
        let entry = children_weights.entry(weight).or_insert(0);
        *entry += 1;
    }

    let weight_set: HashSet<usize> = HashSet::from_iter(
        children_weights.iter()
            .filter(|(_, v)| **v == 1)
            .map(|(k, _)| *k));

    children.iter()
        .filter(|child| weight_set.contains(&weights[*child]))
        .map(|child| &programs[child])
        .collect()
}

fn find_unbalanced<'a>(programs: &'a HashMap<String, Program>, weights: &'a HashMap<String, usize>) -> Vec<&'a Program> {
    programs.iter().flat_map(|(_, v)| {
        unbalanced_weights(&v.children, programs, weights)
    }).collect()
}

fn children_balanced(program: &Program, weights: &HashMap<String, usize>) -> bool {
    HashSet::<usize>::from_iter(program.children.iter().map(|child| weights[child])).len() == 1
}

#[cfg(test)]
mod day7_tests {
    use  super::*;

    #[test]
    fn examples() {
        let input = "\
pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)";

        assert_eq!(part1(input), "tknk");
        assert_eq!(part2(input), 60);
    }
}