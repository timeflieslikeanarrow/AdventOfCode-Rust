use std::fs;
use std::time::Instant;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

fn main() {
    
    let rule_input = fs::read_to_string("day21.input").unwrap();
    let rules = parse_rules(&rule_input);
    //println!("rule count: {}", rules.len());
    let input = "\
.#.
..#
###";

    let mut pattern = Pattern::new(input);

    let start = Instant::now();
    for i in 0..18 {
        let patterns = pattern.divide();
        let patterns: Vec<Vec<_>> = patterns.iter()
            .map(|v| 
                v.iter().map(|p| p.match_rule(&rules)).collect()
            ).collect();

        pattern = Pattern::merge(&patterns);
        //println!("{} pixels: {}", i, pattern.patterns.len());

        if i == 4 {
            println!("part 1: {}", pattern.count_pixels());
        }
    }
        
    println!("part 2: {}", pattern.count_pixels());
    println!("running time: {}s", start.elapsed().as_secs());
}

#[derive(Debug)]
struct Pattern {
    patterns: Vec<String>
}

fn vertical_flip(pattern: &Vec<String>) -> Vec<String> {
    pattern.iter().map(|row| row.to_string()).rev().collect()
}

fn flip_string(row: &str) -> String {
    row.chars().rev().collect()
}

fn horizontal_flip(pattern: &Vec<String>) -> Vec<String> {
    pattern.iter().map(|row| flip_string(row)).collect()
}

fn rotate_90(pattern: &Vec<String>) -> Vec<String> {
    let chars: Vec<Vec<char>> = pattern.iter().map(|row| row.chars().collect()).collect();
    let m = pattern.len();
    let n = pattern[0].len();

    (0..n).map(|c| 
        String::from_iter((0..m).rev().map(|r| chars[r][c]))
    ).collect()
}

fn divide(s: &str, size: usize) -> Vec<String> {
    //println!("**{} {}**", s.len(), size);
    assert!(s.len() % size == 0);
    let count = s.len() / size;
    (0..count).map(|i| 
        String::from_iter(s.chars().skip(i*size).take(size))
    ).collect()
}

impl Pattern {
    fn new(input: &str) -> Self {
        let patterns = input.trim().lines().map(|line| line.to_string()).collect();
        Pattern { patterns }
    }

    fn merge(patterns: &Vec<Vec<Pattern>>) -> Self {
        let mut rows = Vec::new();

        let n = patterns.len();
        let m = patterns[0].len();
        let pattern_len = patterns[0][0].patterns.len();

        //println!("n: {}, m: {}", n, m);
        for i in 0..n {
           for row in 0..pattern_len {
               let line: Vec<_> = (0..m).map(|k| (&patterns[i][k].patterns[row]).to_string()).collect();
               //println!("line in merge: {:?}", line);
               rows.push(line.join(""));
           }
        }

        //println!("rows: {:?}", rows);
        Pattern { patterns: rows }
    }

    fn variations(&self) -> HashSet<String> {
        let mut vars = HashSet::new();

        vars.insert(self.patterns.clone().join("/"));
        vars.insert(vertical_flip(&self.patterns).join("/"));
        vars.insert(horizontal_flip(&self.patterns).join("/"));

        let rotation = rotate_90(&self.patterns);
        vars.insert(rotation.clone().join("/"));
        vars.insert(vertical_flip(&rotation).join("/"));
        vars.insert(horizontal_flip(&rotation).join("/"));

        vars
    }

    fn divide(&self) -> Vec<Vec<Pattern>> {
        let div = if self.patterns.len() % 2 == 0 { 2 } else { 3 };
        
        let divisions = self.patterns.len() / div;
        //println!("div: {} {} {} {:?}", div, divisions, self.patterns.len(), self.patterns);

        let sections: Vec<_> = self.patterns.iter().map(|s| divide(s, div)).collect();
        (0..divisions).map(|row| 
            (0..divisions).map(|col|
                Pattern { 
                    patterns: (0..div)
                                .map(|drow| (&sections[row*div + drow][col]).to_owned())
                                .collect()
                }
            ).collect()
        ).collect()
    }

    fn match_rule(&self, rules: &HashMap<String, String>) -> Pattern {
        let variations = self.variations();

        for var in variations.iter() {
            if rules.contains_key(var) {
                let pattern = &rules[var];
                //println!("matched pattern: {}", pattern);
                return Pattern::new(&pattern.replace("/", "\n"));
            }
        }

        panic!("didn't find matching rule")
    }

    fn count_pixels(&self) -> usize {
        self.patterns.iter().map(|p| p.chars().filter(|c| *c == '#').count()).sum()
    }
}

impl ToString for Pattern {
    fn to_string(&self) -> String {
        self.patterns.join("/")
    }
}

fn parse_rules(input: &str) -> HashMap<String, String> {
    input.trim().lines().map(|line| {
        let parts: Vec<_> = line.trim().split(" => ").collect();
        (parts[0].to_string(), parts[1].to_string())
    }).collect()
}

#[cfg(test)]
mod day21_tests {
    use super::*;

    #[test]
    fn part1_example() {
        let rule_input = "\
../.# => ##./#../...
.#./..#/### => #..#/..../..../#..#";
        
        let rules = parse_rules(rule_input);
        assert_eq!(rules.len(), 2);

        let input = "\
.#.
..#
###";
        let mut pattern = Pattern::new(input);
        for _ in 0..2 {
            println!("original: {}", pattern.to_string());
            let patterns = pattern.divide();
            println!("after divide: {:?}", patterns);
            let patterns: Vec<Vec<_>> = patterns.iter()
                .map(|v| 
                    v.iter().map(|p| p.match_rule(&rules)).collect()
                ).collect();
            
            println!("after rule matching: {:?}", patterns);
            pattern = Pattern::merge(&patterns);
        }
       
        println!("after two: {}", pattern.to_string());
        assert_eq!(pattern.count_pixels(), 12);
    }
}