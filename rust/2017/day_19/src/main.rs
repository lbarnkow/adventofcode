#![allow(dead_code)]

use std::fmt::Display;

fn main() {
    println!("Advent of Code 2017 - day 19");
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
struct Pos {
    x: isize,
    y: isize,
}

impl From<Dir> for Pos {
    fn from(value: Dir) -> Self {
        match value {
            Dir::Up => Self::new(0, -1),
            Dir::Down => Self::new(0, 1),
            Dir::Left => Self::new(-1, 0),
            Dir::Right => Self::new(1, 0),
        }
    }
}

impl std::ops::Add for Pos {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Pos {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn go(&self, dir: Dir) -> Self {
        *self + dir.into()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Cell {
    Empty,
    Vertical,
    Horizontal,
    Letter(char),
    Turn,
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        match value {
            ' ' => Self::Empty,
            '|' => Self::Vertical,
            '-' => Self::Horizontal,
            '+' => Self::Turn,
            c if c.is_alphabetic() => Self::Letter(c),
            _ => panic!("Illegal cell character: '{value}'!"),
        }
    }
}

impl From<Cell> for char {
    fn from(value: Cell) -> Self {
        match value {
            Cell::Empty => ' ',
            Cell::Vertical => '|',
            Cell::Horizontal => '-',
            Cell::Letter(c) => c,
            Cell::Turn => '+',
        }
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<char>::into(*self))
    }
}

struct Network {
    width: usize,
    height: usize,
    map: Vec<Cell>,
    entry: Pos,
}

impl From<&str> for Network {
    fn from(value: &str) -> Self {
        let mut map: Vec<Cell> = Vec::new();
        let mut width = None;
        let mut height = 0;
        let mut entry = None;
        for line in value.lines() {
            if width.is_none() {
                width = Some(line.len());
            } else if let Some(width) = width {
                if width != line.len() {
                    panic!("Line #{height} has bad length!");
                }
            }
            height += 1;

            for (idx, c) in line.chars().enumerate() {
                if c == '|' && entry.is_none() {
                    entry = Some(Pos::new(idx.try_into().unwrap(), 0));
                }
                map.push(c.into());
            }
        }

        let width = width.unwrap();
        let entry = entry.unwrap();

        Self {
            width,
            height,
            map,
            entry,
        }
    }
}

impl Display for Network {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut i = 0;
        for _ in 0..self.height {
            for _ in 0..self.width {
                write!(f, "{}", self.map[i]).unwrap();
                i += 1;
            }
            writeln!(f, "").unwrap();
        }

        Ok(())
    }
}

impl Network {
    fn is_in_bounds(&self, pos: Pos) -> bool {
        let width = self.width.try_into().unwrap();
        let height = self.height.try_into().unwrap();
        pos.x >= 0 && pos.y >= 0 && pos.x < width && pos.y < height
    }

    fn is_accessible_cell(&self, pos: Pos) -> bool {
        if self.is_in_bounds(pos) {
            if self.lookup_cell(pos) != Cell::Empty {
                return true;
            }
        }
        false
    }

    fn pos_to_idx(&self, pos: Pos) -> usize {
        let x: usize = pos.x.try_into().unwrap();
        let y: usize = pos.y.try_into().unwrap();
        y * self.width + x
    }

    fn lookup_cell(&self, pos: Pos) -> Cell {
        self.map[self.pos_to_idx(pos)]
    }

    fn resolve_turn(&self, pos: Pos, dir: Dir) -> Dir {
        if let Cell::Turn = self.lookup_cell(pos) {
            if dir == Dir::Up || dir == Dir::Down {
                let pos_left = pos + Dir::Left.into();
                let pos_right = pos + Dir::Right.into();
                if self.is_accessible_cell(pos_left) && self.is_accessible_cell(pos_right) {
                    panic!("Could move into two directions at {pos:?}, got here by going {dir:?}!");
                } else if self.is_accessible_cell(pos_left) {
                    Dir::Left
                } else if self.is_accessible_cell(pos_right) {
                    Dir::Right
                } else {
                    panic!("Can't turn at {pos:?}, got here by going {dir:?}!");
                }
            } else {
                let pos_up = pos + Dir::Up.into();
                let pos_down = pos + Dir::Down.into();
                if self.is_accessible_cell(pos_up) && self.is_accessible_cell(pos_down) {
                    panic!("Could move into two directions at {pos:?}, got here by going {dir:?}!");
                } else if self.is_accessible_cell(pos_up) {
                    Dir::Up
                } else if self.is_accessible_cell(pos_down) {
                    Dir::Down
                } else {
                    panic!("Can't turn at {pos:?}, got here by going {dir:?}!");
                }
            }
        } else {
            dir
        }
    }

    fn follow_path(&self) -> (String, usize) {
        let mut out = String::new();

        let mut steps = 0;
        let mut pos = self.entry;
        let mut dir = Dir::Down;

        while self.is_accessible_cell(pos) {
            steps += 1;
            if let Cell::Letter(c) = self.lookup_cell(pos) {
                out.push(c);
            }

            dir = self.resolve_turn(pos, dir);
            pos = pos + dir.into();
        }

        (out, steps)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Network, Pos};

    #[test]
    fn test_examples() {
        let pipes = "    |          
    |  +--+    
    A  |  C    
F---|----E|--+ 
    |  |  |  D 
    +B-+  +--+ ";

        let pipes = Network::from(pipes);
        assert_eq!(pipes.entry, Pos::new(4, 0));

        let (output, steps) = pipes.follow_path();
        assert_eq!(output, "ABCDEF");
        assert_eq!(steps, 38);
    }

    #[test]
    fn test_input() {
        let pipes = std::fs::read_to_string("input/pipes.txt").unwrap();

        let pipes = Network::from(pipes.as_str());
        assert_eq!(pipes.entry, Pos::new(107, 0));

        let (output, steps) = pipes.follow_path();
        assert_eq!(output, "HATBMQJYZ");
        assert_eq!(steps, 16332);
    }
}
