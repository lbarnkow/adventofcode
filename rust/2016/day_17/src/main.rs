#![allow(dead_code)]

use std::{collections::VecDeque, ops::Add};

fn main() {
    println!("Advent of Code 2016 - day 17");
}

#[derive(Debug, Clone, Copy)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl From<Dir> for char {
    fn from(value: Dir) -> Self {
        match value {
            Dir::Up => 'U',
            Dir::Down => 'D',
            Dir::Left => 'L',
            Dir::Right => 'R',
        }
    }
}

impl From<Dir> for Pos {
    fn from(value: Dir) -> Self {
        match value {
            Dir::Up => Self { x: 0, y: -1 },
            Dir::Down => Self { x: 0, y: 1 },
            Dir::Left => Self { x: -1, y: 0 },
            Dir::Right => Self { x: 1, y: 0 },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Pos {
    x: isize,
    y: isize,
}

impl Pos {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

impl Add for Pos {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

fn is_open(ch: char) -> bool {
    ch >= 'b' && ch <= 'f'
}

fn compute_possible_dirs(
    pos: Pos,
    width: isize,
    height: isize,
    passcode: &str,
    path: &str,
) -> Vec<Dir> {
    let base = format!("{passcode}{path}");
    let digest = md5::compute(base);
    let hash = format!("{:x}", digest);
    let mut iter = hash.chars();

    let mut dirs = Vec::with_capacity(4);

    if is_open(iter.next().unwrap()) && pos.y > 0 {
        dirs.push(Dir::Up)
    };
    if is_open(iter.next().unwrap()) && pos.y < (height - 1) {
        dirs.push(Dir::Down)
    };
    if is_open(iter.next().unwrap()) && pos.x > 0 {
        dirs.push(Dir::Left)
    };
    if is_open(iter.next().unwrap()) && pos.x < (width - 1) {
        dirs.push(Dir::Right)
    };

    dirs
}

fn shortest_path(width: usize, height: usize, passcode: &str, start: Pos, goal: Pos) -> String {
    let width: isize = width.try_into().expect("width must fit into isize");
    let height: isize = height.try_into().expect("height must fit into isize");

    let mut queue = VecDeque::new();

    queue.push_back((start, String::new()));
    while let Some((pos, path)) = queue.pop_front() {
        let dirs = compute_possible_dirs(pos, width, height, passcode, &path);

        for dir in dirs {
            let mut path = path.clone();
            path.push(dir.into());
            let next_pos = pos + dir.into();
            if next_pos == goal {
                return path;
            }
            queue.push_back((next_pos, path));
        }
    }

    panic!("No valid path found!")
}

fn longest_path(width: usize, height: usize, passcode: &str, start: Pos, goal: Pos) -> usize {
    let width: isize = width.try_into().expect("width must fit into isize");
    let height: isize = height.try_into().expect("height must fit into isize");

    let mut longest_path = 0;
    let mut queue = VecDeque::new();

    queue.push_back((start, String::new()));
    while let Some((pos, path)) = queue.pop_front() {
        let dirs = compute_possible_dirs(pos, width, height, passcode, &path);

        for dir in dirs {
            let mut path = path.clone();
            path.push(dir.into());
            let next_pos = pos + dir.into();
            if next_pos == goal {
                longest_path = longest_path.max(path.len());
            } else {
                queue.push_back((next_pos, path));
            }
        }
    }

    longest_path
}

#[cfg(test)]
mod tests {
    use crate::{longest_path, shortest_path, Pos};

    #[test]
    #[should_panic]
    fn test_panic_1() {
        shortest_path(4, 4, "hijkl", Pos::new(0, 0), Pos::new(3, 3));
    }

    #[test]
    fn test_example() {
        let start = Pos::new(0, 0);
        let vault = Pos::new(3, 3);

        let path = shortest_path(4, 4, "ihgpwlah", start, vault);
        assert_eq!(path, "DDRRRD");
        let path = shortest_path(4, 4, "kglvqrro", start, vault);
        assert_eq!(path, "DDUDRLRRUDRD");
        let path = shortest_path(4, 4, "ulqzkmiv", start, vault);
        assert_eq!(path, "DRURDRUDDLLDLUURRDULRLDUUDDDRR");
    }

    #[test]
    fn test_example_part2() {
        let start = Pos::new(0, 0);
        let vault = Pos::new(3, 3);

        let len = longest_path(4, 4, "ihgpwlah", start, vault);
        assert_eq!(len, 370);
        let len = longest_path(4, 4, "kglvqrro", start, vault);
        assert_eq!(len, 492);
        let len = longest_path(4, 4, "ulqzkmiv", start, vault);
        assert_eq!(len, 830);
    }

    #[test]
    fn test_input() {
        let start = Pos::new(0, 0);
        let vault = Pos::new(3, 3);

        let path = shortest_path(4, 4, "gdjjyniy", start, vault);
        assert_eq!(path, "DUDDRLRRRD");
    }

    #[test]
    fn test_input_part2() {
        let start = Pos::new(0, 0);
        let vault = Pos::new(3, 3);

        let len = longest_path(4, 4, "gdjjyniy", start, vault);
        assert_eq!(len, 578);
    }
}
