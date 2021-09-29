use std::collections::HashMap;

#[derive(Debug)]
pub enum Value {
    Register(char),
    Number(i64),
}

use self::Value::*;

pub fn parse_value(input: &str) -> Value {
    match input.parse() {
        Err(_) => Register(input.chars().next().unwrap()),
        Ok(n) => Number(n),
    }
}

pub fn parse_input(input: &str) -> Vec<(&str, Value, Value)> {
    input.trim().lines().map(|line| {
        let parts: Vec<_> = line.split(" ").collect();
        let value1 = parse_value(parts[1]);
        let value2 = if parts.len() > 2 {
            parse_value(parts[2])
        } else {
            Number(i64::MIN)
        };

        (parts[0], value1, value2)
    }).collect()
}