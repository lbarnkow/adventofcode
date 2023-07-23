#![allow(dead_code)]

use core::panic;

fn main() {
    println!("Advent of Code 2016 - day 01");
}

#[derive(Debug, Clone)]
enum Dir {
    N,
    E,
    S,
    W,
}

impl Dir {
    fn turn(&self, instruction: &Instruction) -> Self {
        match (self, instruction) {
            (Dir::N, Instruction::L(_)) => Self::W,
            (Dir::N, Instruction::R(_)) => Self::E,
            (Dir::E, Instruction::L(_)) => Self::N,
            (Dir::E, Instruction::R(_)) => Self::S,
            (Dir::S, Instruction::L(_)) => Self::E,
            (Dir::S, Instruction::R(_)) => Self::W,
            (Dir::W, Instruction::L(_)) => Self::S,
            (Dir::W, Instruction::R(_)) => Self::N,
        }
    }
}

#[derive(Debug, Clone)]
struct Pos {
    dir: Dir,
    x: isize,
    y: isize,
}

impl PartialEq for Pos {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Pos {
    fn default() -> Self {
        Self {
            dir: Dir::N,
            x: 0,
            y: 0,
        }
    }

    fn apply(&mut self, instruction: &Instruction) {
        self.turn(instruction);

        let dist = match instruction {
            Instruction::L(d) => *d,
            Instruction::R(d) => *d,
        };

        self.step(dist);
    }

    fn turn(&mut self, instruction: &Instruction) {
        self.dir = self.dir.turn(&instruction);
    }

    fn step(&mut self, dist: usize) {
        let dist: isize = dist.try_into().unwrap();
        match self.dir {
            Dir::N => self.y += dist,
            Dir::E => self.x += dist,
            Dir::S => self.y -= dist,
            Dir::W => self.x -= dist,
        };
    }

    fn dist_to_origin(&self) -> usize {
        self.x.unsigned_abs() + self.y.unsigned_abs()
    }
}

#[derive(Debug)]
enum Instruction {
    L(usize),
    R(usize),
}

impl Instruction {
    fn parse(instruction: &str) -> Self {
        let instruction = instruction.trim();
        match &instruction[0..1] {
            "L" => Instruction::L((&instruction[1..]).parse().unwrap()),
            "R" => Instruction::R((&instruction[1..]).parse().unwrap()),
            _ => panic!("illegal instruction {instruction}"),
        }
    }

    fn steps(&self) -> usize {
        match self {
            Instruction::L(steps) => *steps,
            Instruction::R(steps) => *steps,
        }
    }
}

fn block_dist(instructions: &str) -> usize {
    let mut pos = Pos::default();

    instructions
        .split(',')
        .map(|s| Instruction::parse(s))
        .for_each(|i| pos.apply(&i));

    pos.dist_to_origin()
}

fn block_dist_2(instructions: &str) -> usize {
    let mut pos = Pos::default();
    let mut visited = vec![pos.clone()];

    let instructions: Vec<Instruction> = instructions
        .split(',')
        .map(|s| Instruction::parse(s))
        .collect();

    for instruction in instructions {
        pos.turn(&instruction);
        for _ in 0..instruction.steps() {
            pos.step(1);
            if visited.contains(&pos) {
                return pos.dist_to_origin();
            }
            visited.push(pos.clone());
        }
    }

    panic!("no location visited twice")
}

#[cfg(test)]
mod tests {
    use crate::{block_dist, block_dist_2};

    #[test]
    fn test_examples() {
        let instructions = "R2, L3";
        assert_eq!(block_dist(instructions), 5);

        let instructions = "R2, R2, R2";
        assert_eq!(block_dist(instructions), 2);

        let instructions = "R5, L5, R5, R3";
        assert_eq!(block_dist(instructions), 12);

        let instructions = "R8, R4, R4, R8";
        assert_eq!(block_dist_2(instructions), 4);
    }

    #[test]
    fn test_input() {
        let instructions = std::fs::read_to_string("input/instructions.txt").unwrap();
        assert_eq!(block_dist(&instructions), 273);

        let instructions = std::fs::read_to_string("input/instructions.txt").unwrap();
        assert_eq!(block_dist_2(&instructions), 115);
    }
}
