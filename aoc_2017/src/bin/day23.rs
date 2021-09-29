use std::fs;
use std::collections::HashMap;

use aoc_2017::processor::Value::{self, *};
use aoc_2017::processor::{parse_input};

fn main() {
    let input = fs::read_to_string("day23.input").unwrap();
    let program = Program::new(&input);
    println!("instructions count: {}", program.instructions.len());
    println!("part 1: {}", program.run(0));

    println!("part 2: {}", simulate(1));
}


struct Program<'a> {
    instructions: Vec<(&'a str, Value, Value)>,
}

impl<'a> Program<'a> {
    pub fn new(input: &'a str) -> Self {
        Program { 
            instructions: parse_input(input),
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

    fn run(&self, a: i64) -> usize {
        let mut registers = HashMap::new();
        registers.insert('a', a);

        ('b'..='h').for_each(|c| {
            registers.insert(c, 0);
        });

        let mut ip: i64 = 0;
        let mut mul_count = 0;
        while 0 <= ip && ip < self.instructions.len() as i64 {
            let mut jumped = false;
            let (inst, value1, value2) = &self.instructions[ip as usize];
            //println!("{:?}", registers);
            //println!("{} {} {:?} {:?}",  ip + 1, inst, value1, value2);
            match *inst {
                "set" => {
                    let register = Program::get_register(value1);
                    let value =  Program::get_value(value2, &registers);

                    registers.insert(register, value);
                }
                "sub" => {
                    let register = Program::get_register(value1);
                    let value =  Program::get_value(value2, &registers);

                    let entry= registers.entry(register).or_insert(0);
                    *entry -= value;

                    if register == 'h' {
                        println!("{:?}", registers);
                    }
                }
                // "mod" => {
                //     let register = Program::get_register(value1);
                //     let value =  Program::get_value(value2, &registers);

                //     let entry= registers.entry(register).or_insert(0);
                //     *entry %= value;
                // }
                "mul" => {
                    let register = Program::get_register(value1);
                    let value =  Program::get_value(value2, &registers);

                    let entry= registers.entry(register).or_insert(0);
                    *entry *= value;
                    mul_count += 1;
                }
                "jnz" => {
                    let value1 =  Program::get_value(value1, &registers);
                    let value2 = Program::get_value(value2, &registers);

                    if value1 != 0 {
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

        mul_count
    }
}


fn simulate(a: i64) -> i64 {
    let mut b: i64;
    let mut c: i64;
    let mut d: i64;
    //let mut e: i64;
    let mut f: i64;
    let mut g: i64;
    let mut h: i64 = 0; 

    b = 79;
    c = b;

    if a == 1
    {
        b *= 100;
        b -= -100000;
        c = b;
        c -= -17000;
    }

    loop
    {
        f = 1;
        d = 2;

        loop
        {
            //e = 2;
            
            if b % d == 0 {
                f = 0;
            }

            d -= -1;
            g = d;
            g -= b;

            if g == 0 {
                break
            }
        } 

        if f == 0 {
            h -= -1;
        }
        
        g = b - c;

        if g == 0 {
            break;
        }

        b -= -17;
    
        //println!("a={} b={} c={} d={} e={} f={} g={} h={}", a, b, c, d, e, f, g, h);
    } 

    h
}
