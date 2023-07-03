#![allow(dead_code)]
use std::collections::HashSet;

fn main() {
    println!("Advent of Code 2015 - day 3");
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Pos {
    x: i64,
    y: i64,
}

impl Pos {
    fn go(&self, dir: char) -> Self {
        match dir {
            '^' => Self {
                x: self.x,
                y: self.y - 1,
            },
            'v' => Self {
                x: self.x,
                y: self.y + 1,
            },
            '<' => Self {
                x: self.x - 1,
                y: self.y,
            },
            '>' => Self {
                x: self.x + 1,
                y: self.y,
            },
            _ => panic!("Invalid input!"),
        }
    }
}

struct Deliveries {
    unique_houses: usize,
}

fn deliver_alone(moves: &str) -> Deliveries {
    let mut pos = Pos { x: 0, y: 0 };
    let mut visited = HashSet::new();

    visited.insert(pos);

    for dir in moves.chars() {
        pos = pos.go(dir);
        visited.insert(pos);
    }

    Deliveries {
        unique_houses: visited.len(),
    }
}

fn deliver_tag_team(moves: &str) -> Deliveries {
    let mut pos_santa = Pos { x: 0, y: 0 };
    let mut pos_robot = Pos { x: 0, y: 0 };
    let mut visited = HashSet::new();

    visited.insert(pos_santa);

    for (idx, dir) in moves.chars().enumerate() {
        let actor = if idx % 2 == 0 {
            &mut pos_santa
        } else {
            &mut pos_robot
        };
        *actor = actor.go(dir);

        visited.insert(*actor);
    }

    Deliveries {
        unique_houses: visited.len(),
    }
}

#[cfg(test)]
mod tests {
    use crate::{deliver_alone, deliver_tag_team};

    #[test]
    fn test_examples() {
        assert_eq!(deliver_alone(">").unique_houses, 2);
        assert_eq!(deliver_alone("^>v<").unique_houses, 4);
        assert_eq!(deliver_alone("^v^v^v^v^v").unique_houses, 2);
    }

    #[test]
    fn test_elf_input() {
        let input = std::fs::read_to_string("input/moves.txt").unwrap();

        assert_eq!(deliver_alone(&input).unique_houses, 2081);
        assert_eq!(deliver_tag_team(&input).unique_houses, 2341);
    }
}
