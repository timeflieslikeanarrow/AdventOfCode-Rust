use std::fs;
use std::str;

fn main() {
    let input = fs::read_to_string("day16.input").unwrap();
    println!("part 1: {}", run(&input, 1));
    println!("part 2: {}", run(&input, 1_000_000_000));
}

enum Moves {
    Spin(usize),
    Exchange(usize, usize),
    Partner(u8, u8)
}

use self::Moves::*;

fn run(input: &str, times: usize) -> String {
    let moves: Vec<_> = input.trim().split(",").map(|m| parse_move(m)).collect();
    let programs = (b'a' ..= b'p').collect();
    let result = permute(programs, moves, times);
    String::from_utf8(result).unwrap()
}

fn parse_move(input: &str) -> Moves {
    let bytes= input.as_bytes();
    let substr = str::from_utf8(&bytes[1..]).unwrap();
    match bytes[0] {
        b's' => Spin(substr.parse().unwrap()),
        b'x' => {
            let parts:Vec<_> = substr.split("/").collect();
            Exchange(parts[0].parse().unwrap(), parts[1].parse().unwrap() )
        }
        b'p' => Partner(bytes[1], bytes[3]),
        _ => unreachable!()
    }
}

fn permute(mut programs: Vec<u8>, moves: Vec<Moves>, times: usize) -> Vec<u8> {
    let n = programs.len();

    for t in 0..times {
        for m in moves.iter() {
            match m {
                Spin(last_n) => {
                    let drain: Vec<_> = programs.drain(0..n - last_n).collect();
                    programs.extend(drain);
                },
                Exchange(i, j) => { programs.swap(*i, *j); },
                Partner(a, b) => {
                    let i = programs.iter().position(|c| c == a).unwrap();
                    let j = programs.iter().position(|c| c == b).unwrap();
                    programs.swap(i, j);
                }
            }
        }

        if t % 1_000_000 == 0 {
            println!("{}", t);
        }
    }

    programs
}

#[cfg(test)]
mod day16_tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(permute(vec![b'a',b'b',b'c', b'd', b'e'], vec![Spin(1), Exchange(3,4), Partner(b'e', b'b')], 1), vec![b'b',b'a', b'e', b'd', b'c']);
        assert_eq!(permute(vec![b'a',b'b',b'c', b'd', b'e'], vec![Spin(1), Exchange(3,4), Partner(b'e', b'b')], 2), vec![b'c',b'e', b'a', b'd', b'b']);
    }
}