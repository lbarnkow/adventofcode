#![allow(dead_code)]

use std::fmt::Display;

fn main() {
    println!("Advent of Code 2020 - day 12");
}

#[derive(Debug)]
struct TryFromError {
    msg: String,
}

impl From<&str> for TryFromError {
    fn from(msg: &str) -> Self {
        Self {
            msg: msg.to_owned(),
        }
    }
}

impl From<String> for TryFromError {
    fn from(msg: String) -> Self {
        Self { msg }
    }
}

impl Display for TryFromError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ERR: {}", &self.msg)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Action {
    N,
    S,
    E,
    W,
    R,
    L,
    F,
}

impl TryFrom<char> for Action {
    type Error = TryFromError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'N' => Ok(Self::N),
            'S' => Ok(Self::S),
            'E' => Ok(Self::E),
            'W' => Ok(Self::W),
            'R' => Ok(Self::R),
            'L' => Ok(Self::L),
            'F' => Ok(Self::F),
            _ => Err(format!("Invalid action char: '{value}'!").into()),
        }
    }
}

#[derive(Debug)]
struct Instruction {
    action: Action,
    value: isize,
}

impl TryFrom<&str> for Instruction {
    type Error = TryFromError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() < 2 {
            return Err("Instructions must be at least 2 characters long!".into());
        }

        let action: Action = value.chars().next().unwrap().try_into()?;
        let value = value[1..]
            .parse::<isize>()
            .map_err(|_| -> TryFromError { format!("Can't parse value for: '{value}'!").into() })?;

        Ok(Self { action, value })
    }
}

impl Instruction {
    fn try_from_batch(batch: &str) -> Result<Vec<Self>, TryFromError> {
        let mut v = Vec::new();

        for line in batch.lines() {
            v.push(line.try_into()?);
        }

        Ok(v)
    }
}

#[derive(Debug)]
struct Ship {
    x: isize,
    y: isize,
    dir: isize,
    w_x: isize,
    w_y: isize,
}

impl Default for Ship {
    fn default() -> Self {
        Self {
            x: Default::default(),
            y: Default::default(),
            dir: Default::default(),
            w_x: 10,
            w_y: -1,
        }
    }
}

impl Ship {
    fn follow_part1(&mut self, instructions: &[Instruction]) {
        for instruction in instructions {
            match (instruction.action, instruction.value) {
                (Action::N, value) => self.y -= value,
                (Action::S, value) => self.y += value,
                (Action::E, value) => self.x += value,
                (Action::W, value) => self.x -= value,
                (Action::L, value) => self.dir -= value,
                (Action::R, value) => self.dir += value,
                (Action::F, value) => match self.dir {
                    0 => self.x += value,
                    90 => self.y += value,
                    180 => self.x -= value,
                    270 => self.y -= value,
                    _ => panic!("Illegal direction: {}!", self.dir),
                },
            }
            while self.dir < 0 {
                self.dir += 360;
            }
            while self.dir > 270 {
                self.dir -= 360;
            }
        }
    }

    fn follow_part2(&mut self, instructions: &[Instruction]) {
        for instruction in instructions {
            match instruction.action {
                Action::N => self.w_y -= instruction.value,
                Action::S => self.w_y += instruction.value,
                Action::E => self.w_x += instruction.value,
                Action::W => self.w_x -= instruction.value,
                Action::R => {
                    for _ in 0..(instruction.value / 90) {
                        (self.w_x, self.w_y) = (-self.w_y, self.w_x);
                    }
                }
                Action::L => {
                    for _ in 0..(instruction.value / 90) {
                        (self.w_x, self.w_y) = (self.w_y, -self.w_x);
                    }
                }
                Action::F => {
                    for _ in 0..instruction.value {
                        self.x += self.w_x;
                        self.y += self.w_y;
                    }
                }
            }
        }
    }

    const fn dist_from_origin(&self) -> usize {
        self.x.unsigned_abs() + self.y.unsigned_abs()
    }
}

#[cfg(test)]
mod tests {
    use crate::{Instruction, Ship, TryFromError};

    #[test]
    fn test_examples() -> Result<(), TryFromError> {
        let instructions = "\
            F10\n\
            N3\n\
            F7\n\
            R90\n\
            F11\
        ";
        let instructions = Instruction::try_from_batch(instructions)?;

        let mut ship = Ship::default();
        ship.follow_part1(&instructions);

        assert_eq!(ship.dist_from_origin(), 25);

        let mut ship = Ship::default();
        ship.follow_part2(&instructions);

        assert_eq!(ship.dist_from_origin(), 286);

        Ok(())
    }

    #[test]
    fn test_input() -> Result<(), TryFromError> {
        let instructions = std::fs::read_to_string("input/instructions.txt").unwrap();
        let instructions = Instruction::try_from_batch(&instructions)?;

        let mut ship = Ship::default();
        ship.follow_part1(&instructions);

        assert_eq!(ship.dist_from_origin(), 796);

        let mut ship = Ship::default();
        ship.follow_part2(&instructions);

        assert_eq!(ship.dist_from_origin(), 39446);

        Ok(())
    }
}
