use std::fs;
use std::collections::HashMap;
use std::iter::FromIterator;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref PROGRAM: Regex = Regex::new(r"(\w+)\s+\((\d+)\)").unwrap();
}

fn main() {
    let input = fs::read_to_string("day7.input").unwrap();
    println!("part 1: {}", bottom_program(&input));
}

struct Program<'a> {
    name: &'a str,
    weight: usize,
    children: Vec<Program<'a>>
}

fn parse_line(line: &str) -> (String, usize, Vec<&str>) {
    let parts: Vec<_> = line.trim().split(" -> ").collect();
    let caps = PROGRAM.captures_iter(parts[0]).next().unwrap();
    let name = caps[0].to_owned();
    let weight = *&caps[1].parse().unwrap();
    let mut children = Vec::new();
    if parts.len() > 1 {
        children = parts[1].trim().split(", ").collect();
    } 

    (name, weight, children)
}

fn parse(input: &str) -> HashMap<&str, &str> {
    HashMap::from_iter(
        input.lines()
            .filter(|line| line.contains(" -> "))
            .flat_map(|line| {
                let parts: Vec<_> = line.trim().split(" -> ").collect();
                let parent = parts[0].split("(").into_iter().next().unwrap().trim();
                
                parts[1].trim().split(", ").map(move |child| (child, parent))
            })
        )
}

fn bottom_program(input: &str) -> String {
    let parents = parse(input);
    //println!("{:?}", parents);
    let mut key = parents.keys().next().unwrap();
    while parents.contains_key(key) {
        //println!("key: {}", key);
        key = parents.get(key).unwrap();
    }

    key.to_string()
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

        assert_eq!(bottom_program(input), "tknk");
        //assert_eq!(right_weight_for_unbalanced_program(input), 60);
    }
}