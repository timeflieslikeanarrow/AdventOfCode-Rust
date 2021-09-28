use std::{fs, thread};
use std::time::Duration;
use std::collections::HashMap;
use std::sync::mpsc::{Receiver, Sender, channel};
fn main() {
    let input = fs::read_to_string("day18.input").unwrap();
    let mut program = Program::new(&input, 0);
    println!("part 1: {}", program.run());

    let (send1, recv1) = channel();
    let (send2, recv2) = channel();

    let input1 = input.clone();
    let handle1 = thread::spawn(move || {
        let mut program = Program::new(&input1, 0);
        program.run_parallel(send1, recv2)
    });

    let input2 = input.clone();
    let handle2 = thread::spawn(move || {
        let mut program = Program::new(&input2, 1);
        program.run_parallel(send2, recv1)
    });
    
    handle1.join().unwrap();
    let result = handle2.join().unwrap();

    println!("part 2: {}", result);
}

#[derive(Debug)]
enum Value {
    Register(char),
    Number(i64),
}

use self::Value::*;

fn parse_value(input: &str) -> Value {
    match input.parse() {
        Err(_) => Register(input.chars().next().unwrap()),
        Ok(n) => Number(n),
    }
}

fn parse(input: &str) -> Vec<(&str, Value, Value)> {
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

struct Program<'a> {
    instructions: Vec<(&'a str, Value, Value)>,
    id: i64
}

impl<'a> Program<'a> {
    fn new(input: &'a str, id: i64) -> Self {
        Program { 
            instructions: parse(input),
            id,
         }
    }

    fn get_value(value: &Value, registers: &HashMap<char, i64>) -> i64 {
        match *value {
            Register(register) => *registers.get(&register).unwrap_or(&0),
            Number(f) =>  f,
        }
    }

    fn get_register(value: &Value) -> char {
        match *value {
            Register(register) => register,
            Number(_) =>  panic!("cannot set to a number"),
        }
    }

    fn run(&mut self) -> i64 {
        let mut registers = HashMap::new();
        let mut ip: i64 = 0;

        let mut last_frequency = i64::MIN;
        while 0 <= ip && ip < self.instructions.len() as i64 {

            let mut jumped = false;
            let (inst, value1, value2) = &self.instructions[ip as usize];
            //println!("{} {} {:?} {:?}",  ip + 1, inst, value1, value2);
            //println!("{:?}", registers);
            match *inst {
                "snd" => {
                    last_frequency = Program::get_value(value1, &registers);
                }
                "set" => {
                    let register = Program::get_register(value1);
                    let value =  Program::get_value(value2, &registers);

                    registers.insert(register, value);
                }
                "add" => {
                    let register = Program::get_register(value1);
                    let value =  Program::get_value(value2, &registers);

                    let entry= registers.entry(register).or_insert(0);
                    *entry += value;
                }
                "mul" => {
                    let register = Program::get_register(value1);
                    let value =  Program::get_value(value2, &registers);

                    let entry= registers.entry(register).or_insert(0);
                    *entry *= value;
                }
                "mod" => {
                    let register = Program::get_register(value1);
                    let value =  Program::get_value(value2, &registers);

                    let entry= registers.entry(register).or_insert(0);
                    *entry %= value;
                }
                "rcv" => {
                    let value = Program::get_value(value1, &registers);
                    if value != 0 {
                        break;
                    }
                }
                "jgz" => {
                    let value1 =  Program::get_value(value1, &registers);
                    let value2 = Program::get_value(value2, &registers);

                    if value1 > 0 {
                        ip += value2;
                        jumped = true;
                    } 
                }
                _ => unreachable!()
            }

            if !jumped {
                ip += 1;
            }
        }

        last_frequency
    }

    fn run_parallel(&mut self, sender: Sender<i64>, reciver: Receiver<i64>) -> i64 {
        let mut registers = HashMap::new();
        registers.insert('p', self.id);

        let mut ip: i64 = 0;

        let mut send_count = 0;
        while 0 <= ip && ip < self.instructions.len() as i64 {

            let mut jumped = false;
            let (inst, value1, value2) = &self.instructions[ip as usize];
            //println!("{} {} {:?} {:?}",  ip + 1, inst, value1, value2);
            //println!("{:?}", registers);
            match *inst {
                "snd" => {
                    let value = Program::get_value(value1, &registers);
                    //println!("{} sending {}", self.id, value);
                    sender.send(value).unwrap();
                    send_count += 1;
                }
                "set" => {
                    let register = Program::get_register(value1);
                    let value =  Program::get_value(value2, &registers);

                    registers.insert(register, value);
                }
                "add" => {
                    let register = Program::get_register(value1);
                    let value =  Program::get_value(value2, &registers);

                    let entry= registers.entry(register).or_insert(0);
                    *entry += value;
                }
                "mul" => {
                    let register = Program::get_register(value1);
                    let value =  Program::get_value(value2, &registers);

                    let entry= registers.entry(register).or_insert(0);
                    *entry *= value;
                }
                "mod" => {
                    let register = Program::get_register(value1);
                    let value =  Program::get_value(value2, &registers);

                    let entry= registers.entry(register).or_insert(0);
                    *entry %= value;
                }
                "rcv" => {
                    //print!("{} receiving, current send count: {}", self.id, send_count);
                    let register = Program::get_register(value1);

                    let d = Duration::from_secs(5);
                    let value = match reciver.recv_timeout(d) {
                        Ok(value)  => value,
                        Err(_) => break,
                    };

                    //println!("{}", value);
                    registers.insert(register, value);
                }
                "jgz" => {
                    let value1 =  Program::get_value(value1, &registers);
                    let value2 = Program::get_value(value2, &registers);

                    if value1 > 0 {
                        ip += value2;
                        jumped = true;
                    } 
                }
                _ => unreachable!()
            }

            if !jumped {
                ip += 1;
            }
        }

       send_count
    }
}

#[cfg(test)]
mod day18_tests {
    use super::*;

    #[test]
    fn part1_examples() {
        let input = "\
set a 1
add a 2
mul a a
mod a 5
snd a
set a 0
rcv a
jgz a -1
set a 1
jgz a -2";
        let mut program = Program::new(input, 0);
        assert_eq!(program.run(), 4);
    } 

    #[test]
    fn part2_examples() {
        let input = "\
snd 1
snd 2
snd p
rcv a
rcv b
rcv c
rcv d";

        let (send1, recv1) = channel();
        let (send2, recv2) = channel();

        let handle1 = thread::spawn(move || {
            let mut program = Program::new(input, 0);
            program.run_parallel(send1, recv2)
        });

        let handle2 = thread::spawn(move || {
            let mut program = Program::new(input, 1);
            program.run_parallel(send2, recv1)
        });
        
        let result = handle1.join().unwrap();
        handle2.join().unwrap();

        assert_eq!(result, 3);
    } 
}