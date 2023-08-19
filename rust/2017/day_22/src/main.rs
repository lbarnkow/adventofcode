#![allow(dead_code)]

use std::{collections::HashMap, fmt::Display};

use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    println!("Advent of Code 2017 - day 22");
}

lazy_static! {
    static ref RE_SPIN: Regex = Regex::new(r"^s(\d+)$").unwrap();
    static ref RE_EXCHANGE: Regex = Regex::new(r"^x(\d+)/(\d+)$").unwrap();
    static ref RE_PARTNER: Regex = Regex::new(r"^p(\w)/(\w)$").unwrap();
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum State {
    Clean,
    Infected,
    Weakened,
    Flagged,
}

impl From<char> for State {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Clean,
            '#' => Self::Infected,
            'W' => Self::Weakened,
            'F' => Self::Flagged,
            _ => panic!("Illegal state char: {value}!"),
        }
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            State::Clean => write!(f, "."),
            State::Infected => write!(f, "#"),
            State::Weakened => write!(f, "W"),
            State::Flagged => write!(f, "F"),
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
struct Pos {
    x: i64,
    y: i64,
}

impl From<(i64, i64)> for Pos {
    fn from((x, y): (i64, i64)) -> Self {
        Self::new(x, y)
    }
}

impl Pos {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    fn offset(&mut self, dir: Dir) {
        match dir {
            Dir::Up => self.y -= 1,
            Dir::Left => self.x -= 1,
            Dir::Down => self.y += 1,
            Dir::Right => self.x += 1,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Dir {
    Up,
    Left,
    Down,
    Right,
}

impl Dir {
    fn turn_left(&self) -> Self {
        match self {
            Dir::Up => Self::Left,
            Dir::Left => Self::Down,
            Dir::Down => Self::Right,
            Dir::Right => Self::Up,
        }
    }

    fn turn_right(&self) -> Self {
        match self {
            Dir::Up => Self::Right,
            Dir::Left => Self::Up,
            Dir::Down => Self::Left,
            Dir::Right => Self::Down,
        }
    }

    fn reverse(&self) -> Self {
        self.turn_left().turn_left()
    }
}

#[derive(Debug)]
struct Cluster {
    map: HashMap<Pos, State>,
    virus: (Pos, Dir),
    bursts: usize,
    infections: usize,
}

impl Display for Cluster {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut min = self.virus.0;
        let mut max = self.virus.0;
        for p in self.map.keys() {
            min.x = min.x.min(p.x);
            min.y = min.y.min(p.y);
            max.x = max.x.max(p.x);
            max.y = max.y.max(p.y);
        }

        min.x -= 1;
        min.y -= 1;
        max.x += 1;
        max.y += 1;

        for y in min.y..=max.y {
            for x in min.x..=max.x {
                let pos = Pos::new(x, y);
                let node = self.get(pos);
                if self.virus.0 == pos {
                    write!(f, "[{node}]").unwrap();
                } else {
                    write!(f, " {node} ").unwrap();
                }
            }
            writeln!(f, "").unwrap();
        }

        Ok(())
    }
}

impl From<&str> for Cluster {
    fn from(value: &str) -> Self {
        let raw_map = value
            .lines()
            .map(|line| line.chars().map(|c| State::from(c)).collect::<Vec<State>>())
            .collect::<Vec<Vec<State>>>();

        let height = raw_map.len() as i64;
        assert!(height % 2 == 1);
        let width = (&raw_map[0]).len() as i64;
        assert!(width % 2 == 1);

        let mut map = HashMap::new();

        let x_offset = (width / 2) as i64;
        let y_offset = (height / 2) as i64;

        for y in 0..height {
            for x in 0..width {
                let pos = Pos::new(x - x_offset, y - y_offset);
                map.insert(pos, raw_map[y as usize][x as usize]);
            }
        }

        Self {
            map,
            virus: (Pos::default(), Dir::Up),
            bursts: 0,
            infections: 0,
        }
    }
}

impl Cluster {
    fn do_burst(&mut self) {
        self.bursts += 1;

        if self.get(self.virus.0) == State::Infected {
            self.virus.1 = self.virus.1.turn_right();
            self.map.insert(self.virus.0, State::Clean);
        } else {
            self.virus.1 = self.virus.1.turn_left();
            self.map.insert(self.virus.0, State::Infected);
            self.infections += 1;
        }

        self.virus.0.offset(self.virus.1);
    }

    fn do_burst_2(&mut self) {
        self.bursts += 1;

        match self.get(self.virus.0) {
            State::Clean => {
                self.virus.1 = self.virus.1.turn_left();
                self.map.insert(self.virus.0, State::Weakened);
            }
            State::Weakened => {
                self.map.insert(self.virus.0, State::Infected);
                self.infections += 1;
            }
            State::Infected => {
                self.virus.1 = self.virus.1.turn_right();
                self.map.insert(self.virus.0, State::Flagged);
            }
            State::Flagged => {
                self.virus.1 = self.virus.1.reverse();
                self.map.insert(self.virus.0, State::Clean);
            }
        }

        self.virus.0.offset(self.virus.1);
    }

    fn get(&self, pos: Pos) -> State {
        match self.map.get(&pos) {
            Some(state) => *state,
            None => State::Clean,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Cluster;

    #[test]
    fn test_examples() {
        let map = "\
            ..#\n\
            #..\n\
            ...\
        ";
        let mut cluster = Cluster::from(map);
        for _ in 0..70 {
            cluster.do_burst();
        }

        assert_eq!(cluster.bursts, 70);
        assert_eq!(cluster.infections, 41);

        let mut cluster = Cluster::from(map);
        for _ in 0..10_000 {
            cluster.do_burst();
        }

        assert_eq!(cluster.bursts, 10_000);
        assert_eq!(cluster.infections, 5_587);
    }

    #[test]
    fn test_examples_part2() {
        let map = "\
            ..#\n\
            #..\n\
            ...\
        ";
        let mut cluster = Cluster::from(map);
        for _ in 0..100 {
            cluster.do_burst_2();
        }
        assert_eq!(cluster.bursts, 100);
        assert_eq!(cluster.infections, 26);

        let mut cluster = Cluster::from(map);
        for _ in 0..10_000_000 {
            cluster.do_burst_2();
        }

        assert_eq!(cluster.bursts, 10_000_000);
        assert_eq!(cluster.infections, 2_511_944);
    }

    #[test]
    fn test_input() {
        let map = std::fs::read_to_string("input/map.txt").unwrap();
        let mut cluster = Cluster::from(map.as_str());

        for _ in 0..10_000 {
            cluster.do_burst();
        }

        assert_eq!(cluster.bursts, 10_000);
        assert_eq!(cluster.infections, 5_176);
    }

    #[test]
    fn test_input_part2() {
        let map = std::fs::read_to_string("input/map.txt").unwrap();
        let mut cluster = Cluster::from(map.as_str());

        for _ in 0..10_000_000 {
            cluster.do_burst_2();
        }

        assert_eq!(cluster.bursts, 10_000_000);
        assert_eq!(cluster.infections, 2_512_017);
    }
}
