#![allow(dead_code)]

use std::fmt::Display;

use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    println!("Advent of Code 2018 - day 17");
}

lazy_static! {
    static ref RE: Regex = Regex::new(r"^(\w)=(\d+), (\w)=(\d+)..(\d+)$").unwrap();
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Dir {
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, Default)]
struct Vec2d {
    x: usize,
    y: usize,
}

impl Vec2d {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn offset(&self, dir: Dir) -> Option<Self> {
        match dir {
            Dir::Down => Some(Self {
                x: self.x,
                y: self.y + 1,
            }),
            Dir::Right => Some(Self {
                x: self.x + 1,
                y: self.y,
            }),
            Dir::Left => {
                if self.x > 0 {
                    Some(Self {
                        x: self.x - 1,
                        y: self.y,
                    })
                } else {
                    None
                }
            }
        }
    }

    fn to(&self, other: &Self) -> Vec<Self> {
        if self.x == other.x {
            (self.y..=other.y).map(|y| Self::new(self.x, y)).collect()
        } else {
            (self.x..=other.x).map(|x| Self::new(x, self.y)).collect()
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Cell {
    Sand,
    Clay,
    Reachable,
    Water,
    Spring,
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Cell::Sand => '.',
            Cell::Clay => '#',
            Cell::Reachable => '|',
            Cell::Water => '~',
            Cell::Spring => '+',
        };

        write!(f, "{c}")
    }
}

struct Map {
    min_x: usize,
    min_y: usize,
    max_y: usize,
    width: usize,
    height: usize,
    cells: Vec<Cell>,
    spring: Vec2d,
}

impl From<&[(Vec2d, Vec2d)]> for Map {
    fn from(clay_veins: &[(Vec2d, Vec2d)]) -> Self {
        let (width, height) = clay_veins.iter().fold((0, 0), |(width, height), (a, b)| {
            (width.max(a.x.max(b.x)), height.max(a.y.max(b.y)))
        });
        let (mut width, height) = (width + 2, height + 1);

        if width < 502 {
            width = 502;
        }

        let mut min_x = usize::MAX;
        let mut min_y = usize::MAX;
        let mut max_y = usize::MIN;
        let mut cells = vec![Cell::Sand; width * height];

        for (from, to) in clay_veins {
            for v in from.to(to) {
                let idx = v.y * width + v.x;
                cells[idx] = Cell::Clay;

                min_x = min_x.min(from.x.min(to.x));
                min_y = min_y.min(from.y.min(to.y));
                max_y = max_y.max(from.y.max(to.y));
            }
        }

        cells[500] = Cell::Spring;

        min_x = min_x.saturating_sub(1);

        Self {
            min_x,
            min_y,
            max_y,
            width,
            height,
            cells,
            spring: Vec2d::new(500, 0),
        }
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut sep = "";
        for y in 0..self.height {
            write!(f, "{sep}").unwrap();
            for x in self.min_x..self.width {
                let idx = y * self.width + x;
                write!(f, "{}", self.cells[idx]).unwrap();
            }
            sep = "\n";
        }
        Ok(())
    }
}

impl Map {
    fn idx(&self, pos: &Vec2d) -> usize {
        pos.y * self.width + pos.x
    }

    fn find_drop(&mut self, pos: &Vec2d, dir: Dir) -> Option<Vec2d> {
        let mut drop = None;
        let mut test_pos = *pos;

        while let Some(side_pos) = test_pos.offset(dir) {
            test_pos = side_pos;
            let idx = self.idx(&test_pos);
            if self.cells[idx] == Cell::Clay {
                break;
            }
            self.cells[idx] = Cell::Reachable;
            let below_test = test_pos.offset(Dir::Down).unwrap();
            if [Cell::Sand, Cell::Reachable].contains(&self.cells[self.idx(&below_test)]) {
                drop = Some(test_pos);
                break;
            }
        }

        drop
    }

    fn find_drops(&mut self, pos: &Vec2d) -> (Option<Vec2d>, Option<Vec2d>) {
        let left = self.find_drop(pos, Dir::Left);
        let right = self.find_drop(pos, Dir::Right);

        (left, right)
    }

    fn cap_filled_cells(&self, n: usize, filled: usize) -> usize {
        if n >= filled {
            n - filled
        } else {
            n
        }
    }

    fn fill_level_dir(&mut self, pos: &Vec2d, dir: Dir) -> usize {
        let mut filled = 0;
        let mut pos = *pos;

        while let Some(next) = pos.offset(dir) {
            pos = next;
            let idx = self.idx(&next);
            if self.cells[idx] == Cell::Reachable {
                self.cells[idx] = Cell::Water;
                filled += 1;
            } else {
                break;
            }
        }

        filled
    }

    fn fill_level(&mut self, pos: &Vec2d) -> usize {
        let idx = self.idx(pos);
        self.cells[idx] = Cell::Water;
        self.fill_level_dir(pos, Dir::Left) + self.fill_level_dir(pos, Dir::Right) + 1
    }

    fn fill_go(&mut self, pos: &Vec2d, n: usize) -> usize {
        if n == 0 {
            return 0;
        }

        let idx = self.idx(pos);
        self.cells[idx] = Cell::Reachable;

        let mut filled = 0;
        let mut remaining = n;

        let below_pos = pos.offset(Dir::Down).unwrap();
        let below_idx = self.idx(&below_pos);
        if below_idx >= self.cells.len() {
            return filled;
        }

        if self.cells[below_idx] == Cell::Sand {
            filled += self.fill_go(&below_pos, remaining);
            remaining = self.cap_filled_cells(n, filled)
        }
        if remaining == 0 {
            return filled;
        }

        if [Cell::Clay, Cell::Water].contains(&self.cells[below_idx]) {
            let (left_drop, right_drop) = self.find_drops(pos);

            if left_drop.is_none() && right_drop.is_none() {
                filled += self.fill_level(pos);
            } else {
                for drop in [left_drop, right_drop].into_iter().flatten() {
                    filled += self.fill_go(&drop, remaining);
                    remaining = self.cap_filled_cells(n, filled);
                }
            }
        }

        filled
    }

    fn fill(&mut self, n: usize) -> usize {
        let spring = self.spring.offset(Dir::Down).unwrap();
        self.fill_go(&spring, n)
    }

    fn reachable(&self, types: &[Cell]) -> usize {
        let mut count = 0;
        for y in self.min_y..=self.max_y {
            for x in self.min_x..self.width {
                let idx = y * self.width + x;
                let cell = self.cells[idx];
                if types.contains(&cell) {
                    count += 1;
                }
            }
        }

        count
    }
}

fn parse_scan(scan: &str) -> Vec<(Vec2d, Vec2d)> {
    scan.lines()
        .map(|line| {
            let caps = RE.captures(line).unwrap();
            let axis1_label = &caps[1];
            let axis1_value = caps[2].parse::<usize>().unwrap();
            let axis2_label = &caps[3];
            let axis2_from = caps[4].parse::<usize>().unwrap();
            let axis2_to = caps[5].parse::<usize>().unwrap();

            if axis1_label == "x" {
                assert_eq!(axis2_label, "y");
                (
                    Vec2d::new(axis1_value, axis2_from),
                    Vec2d::new(axis1_value, axis2_to),
                )
            } else {
                assert_eq!(axis2_label, "x");
                (
                    Vec2d::new(axis2_from, axis1_value),
                    Vec2d::new(axis2_to, axis1_value),
                )
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::{parse_scan, Cell, Map};

    #[test]
    fn test_examples() {
        let scan = "\
            x=495, y=2..7\n\
            y=7, x=495..501\n\
            x=501, y=3..7\n\
            x=498, y=2..4\n\
            x=506, y=1..2\n\
            x=498, y=10..13\n\
            x=504, y=10..13\n\
            y=13, x=498..504\
        ";

        let scan = parse_scan(scan);
        let map = Map::from(scan.as_slice());

        let expected = std::fs::read_to_string("input/example1.txt").unwrap();
        assert_eq!(map.to_string(), expected);

        let mut map = Map::from(scan.as_slice());
        map.fill(5);
        let expected = std::fs::read_to_string("input/example2.txt").unwrap();
        assert_eq!(map.to_string(), expected);

        let mut map = Map::from(scan.as_slice());
        map.fill(10);
        let expected = std::fs::read_to_string("input/example3.txt").unwrap();
        assert_eq!(map.to_string(), expected);

        let mut map = Map::from(scan.as_slice());
        map.fill(14);
        let expected = std::fs::read_to_string("input/example4.txt").unwrap();
        assert_eq!(map.to_string(), expected);

        let mut map = Map::from(scan.as_slice());
        map.fill(19);
        let expected = std::fs::read_to_string("input/example5.txt").unwrap();
        assert_eq!(map.to_string(), expected);

        let mut map = Map::from(scan.as_slice());
        map.fill(29);
        let expected = std::fs::read_to_string("input/example6.txt").unwrap();
        assert_eq!(map.to_string(), expected);

        let mut map = Map::from(scan.as_slice());
        map.fill(usize::MAX);
        let expected = std::fs::read_to_string("input/example7.txt").unwrap();
        assert_eq!(map.to_string(), expected);
        assert_eq!(map.reachable(&[Cell::Reachable, Cell::Water]), 57);
        assert_eq!(map.reachable(&[Cell::Water]), 29);
    }

    #[test]
    fn test_input() {
        let scan = std::fs::read_to_string("input/scan.txt").unwrap();
        let scan = parse_scan(&scan);

        let mut map = Map::from(scan.as_slice());
        map.fill(usize::MAX);
        assert_eq!(map.reachable(&[Cell::Reachable, Cell::Water]), 35707);
        assert_eq!(map.reachable(&[Cell::Water]), 29293);
    }
}
