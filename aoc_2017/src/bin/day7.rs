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
    own_weight: usize,
    children: Vec<String>,
    total_weight: usize,
}

fn part1(input: &str) -> String {
    let (programs, _) = parse(input);
    find_root(&programs)
}

fn parse_line(line: &str, siblings: &mut HashMap<String, Vec<String>>) -> Program {
    let parts: Vec<_> = line.trim().split(" -> ").collect();
    let caps = PROGRAM_LINE.captures_iter(parts[0]).next().unwrap();
    let name = caps[1].to_owned();
    let own_weight = *&caps[2].parse().unwrap();
    let mut children = Vec::new();
    if parts.len() > 1 {
        children = parts[1].trim().split(", ").map(|s| s.to_string()).collect();
    } 

    add_siblings(&children, siblings);

    Program { 
        name, 
        own_weight, 
        children,
        total_weight: 0,
    }
}

fn add_siblings(children: &Vec<String>, siblings: &mut HashMap<String, Vec<String>>) {
    for child in children.iter() {
        let s = children.iter()
                    .filter(|child2| *child2 != child).map(|child2| child2.to_owned()).collect();

        siblings.insert(child.to_string(), s);
    }
}

fn parse(input: &str) -> (HashMap<String, Program>, HashMap<String, Vec<String>>) {
    let mut siblings = HashMap::new();
    let mut programs = HashMap::new();
    for line in input.lines() {
        let p = parse_line(line, &mut siblings);
        programs.insert(p.name.clone(), p);
    }

    (programs, siblings)
}

fn find_root(programs: &HashMap<String, Program>) -> String {
    let all_names: HashSet<_> = programs.keys().collect();

    let children_names: HashSet<_> = programs.iter()
            .flat_map(|(_, program)| program.children.iter())
            .collect();

    (&all_names - &children_names).iter().next().unwrap().to_string()
}

fn part2(input: &str) -> usize {
    let (mut programs, siblings) = parse(input);

    let weights: HashMap<String, usize> = programs.iter()
            .map(|(k, p)| (k.to_string(), compute_weights(p, &programs)))
            .collect();

    for (k, p) in programs.iter_mut() {
        p.total_weight = weights[k];
    }

    let unbalanced = find_unbalanced(&programs);
    let the_one = unbalanced.iter().filter(|p| children_balanced(p, &programs)).next().unwrap();
    let siblings = &siblings[&the_one.name];

    the_one.own_weight - (weights[&the_one.name] - weights[&siblings[0]])
}

fn compute_weights(parent: &Program, programs: &HashMap<String, Program>) -> usize {
    let child_weights: usize = parent.children.iter().map(|child| {
        let child = programs.get(child).unwrap();
        compute_weights(child, programs)
    }).sum();

    parent.own_weight + child_weights
}

fn unbalanced_programs<'a>(children: &'a Vec<String>,  programs: &'a HashMap<String, Program>) -> Vec<&'a Program> {
    let children: Vec<_> = children.iter().map(|child| &programs[child]).collect();

    children.iter()
        .filter(|child| {
            children.iter().filter(|child2| child.name != child2.name && child.total_weight == child2.total_weight).count() == 0
        })
        .map(|p| *p )
        .collect()
}

fn find_unbalanced<'a>(programs: &'a HashMap<String, Program>) -> Vec<&'a Program> {
    programs.iter().flat_map(|(_, v)| {
        unbalanced_programs(&v.children, programs)
    }).collect()
}

fn children_balanced(program: &Program, programs: &HashMap<String, Program>) -> bool {
    HashSet::<usize>::from_iter(program.children.iter().map(|child| programs[child].total_weight)).len() == 1
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