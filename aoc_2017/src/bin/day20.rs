use std::fs;
use std::collections::HashSet;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref PARTICLE: Regex = Regex::new(r"p=<([^>]+)>, v=<([^>]+)>, a=<([^>]+)>").unwrap();
}
fn main() {
    let input = fs::read_to_string("day20.input").unwrap();
    println!("part 1: {}", closet(&input, 5000));
    println!("part 2: {}", remain_after_collisons(&input, 1000));
}

#[derive(Debug)]
struct Particle {
    position: Vec<i64>,
    velocity: Vec<i64>,
    acceleration: Vec<i64>
}

#[derive(Debug)]
struct Buffer {
    particles: Vec<Particle>
}

impl Buffer {
    fn new(input: &str) -> Self {
        Buffer { particles: Buffer::parse(input)}
    }

    fn parse(input: &str)-> Vec<Particle> {
        input.trim().lines().map(|line| {
            let captures = PARTICLE.captures(line).unwrap();
            let position = captures[1].split(",").map(|s| s.trim().parse().unwrap()).collect();
            let velocity = captures[2].split(",").map(|s| s.trim().parse().unwrap()).collect();
            let acceleration = captures[3].split(",").map(|s| s.trim().parse().unwrap()).collect();
    
            Particle { position, velocity, acceleration}
        }).collect()
    }

    fn step(&mut self) {
        let n = self.particles.len();
        for i in 0..n {
           for j in 0..3 {
                self.particles[i].velocity[j] += self.particles[i].acceleration[j];
                self.particles[i].position[j] += self.particles[i].velocity[j];
            }
        }
    }

    fn closet(&self) -> usize {
        let positions: Vec<i64> = self.particles.iter()
                .map(|p| 
                    p.position.iter().map(|&pos| pos.abs()).sum()
                ).collect();
       
        let min = *positions.iter().min().unwrap();
        positions.iter().position(|pos| *pos == min).unwrap()
    } 

    fn step_with_collision(&mut self) {
        self.step();

        let n = self.particles.len();
        let mut collided = HashSet::new();
        for i in 0..n {
            for j in i+1 ..n {
               if self.particles[i].position == self.particles[j].position {
                    collided.insert(i);
                    collided.insert(j);
                }
            }
        }

        if collided.len() > 0 {
            for i in (0..n).rev() {
                if collided.contains(&i) {
                    self.particles.swap_remove(i);
                }
            }
        }
    }
}

fn closet(input: &str, steps: usize) -> usize {
    let mut buffer = Buffer::new(input);
    //println!("{:?}", buffer);
    for _ in 0..steps {
        buffer.step();
    }

    buffer.closet()
}

fn remain_after_collisons(input: &str, steps: usize) -> usize {
    let mut buffer = Buffer::new(input);
    //println!("{:?}", buffer);
    for _ in 0..steps {
        buffer.step_with_collision();
    }

    buffer.particles.len()
}


#[cfg(test)]
mod day20_tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = "\
p=< 3,0,0>, v=< 2,0,0>, a=<-1,0,0>
p=< 4,0,0>, v=< 0,0,0>, a=<-2,0,0>";

        assert_eq!(closet(input, 100), 0);
    }

    #[test]
    fn part2_example() {
        let input = "\
p=<-6,0,0>, v=< 3,0,0>, a=< 0,0,0>    
p=<-4,0,0>, v=< 2,0,0>, a=< 0,0,0> 
p=<-2,0,0>, v=< 1,0,0>, a=< 0,0,0>
p=< 3,0,0>, v=<-1,0,0>, a=< 0,0,0>";

        assert_eq!(remain_after_collisons(input, 10), 1);
    }
}