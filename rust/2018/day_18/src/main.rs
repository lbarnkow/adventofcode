#![allow(dead_code)]

use std::{collections::HashMap, fmt::Display};

fn main() {
    println!("Advent of Code 2018 - day 18");
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Acre {
    OutOfBounds,
    Open,
    Trees,
    Lumberyard,
}

impl From<char> for Acre {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Open,
            '|' => Self::Trees,
            '#' => Self::Lumberyard,
            _ => panic!("Illegal acre character: '{value}'!"),
        }
    }
}

impl Display for Acre {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Acre::OutOfBounds => '_',
            Acre::Open => '.',
            Acre::Trees => '|',
            Acre::Lumberyard => '#',
        };
        write!(f, "{c}")
    }
}

#[derive(Debug, Clone)]
struct Map {
    width: usize,
    height: usize,
    buf: [Vec<Acre>; 2],
    active: usize,
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let width = value.lines().next().unwrap().len() + 2;
        let height = value.lines().count() + 2;

        let mut buf = [
            vec![Acre::OutOfBounds; width * height],
            vec![Acre::OutOfBounds; width * height],
        ];

        for (y, line) in value.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let idx = (y + 1) * width + (x + 1);
                buf[0][idx] = c.into();
            }
        }

        Self {
            width,
            height,
            buf,
            active: 0,
        }
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let buf = &self.buf[self.active];
        let mut sep = "";
        for y in 1..self.height - 1 {
            write!(f, "{sep}").unwrap();
            for x in 1..self.width - 1 {
                let idx = y * self.width + x;
                write!(f, "{}", buf[idx]).unwrap();
            }
            sep = "\n";
        }

        Ok(())
    }
}

impl Map {
    fn inactive(&self) -> usize {
        (self.active + 1) % 2
    }

    fn flip(&mut self) {
        self.active = self.inactive();
    }

    fn count_neighbors_by_type(&self, idx: usize, t: Acre) -> usize {
        let neighbor_idxs = [
            idx - self.width - 1,
            idx - self.width,
            idx - self.width + 1,
            idx - 1,
            idx + 1,
            idx + self.width - 1,
            idx + self.width,
            idx + self.width + 1,
        ];

        let buf = &self.buf[self.active];
        neighbor_idxs
            .into_iter()
            .map(|idx| buf[idx])
            .filter(|a| *a == t)
            .count()
    }

    fn transform(&self, idx: usize) -> Acre {
        let acre = self.buf[self.active][idx];

        match acre {
            Acre::Open => {
                let adjacent_trees = self.count_neighbors_by_type(idx, Acre::Trees);
                if adjacent_trees >= 3 {
                    Acre::Trees
                } else {
                    acre
                }
            }
            Acre::Trees => {
                let adjacent_lumberyards = self.count_neighbors_by_type(idx, Acre::Lumberyard);
                if adjacent_lumberyards >= 3 {
                    Acre::Lumberyard
                } else {
                    acre
                }
            }
            Acre::Lumberyard => {
                let adjacent_lumberyards = self.count_neighbors_by_type(idx, Acre::Lumberyard);
                let adjacent_trees = self.count_neighbors_by_type(idx, Acre::Trees);
                if adjacent_lumberyards >= 1 && adjacent_trees >= 1 {
                    acre
                } else {
                    Acre::Open
                }
            }
            Acre::OutOfBounds => panic!("Should never transform OutOfBounds!"),
        }
    }

    fn step(&mut self) {
        let inactive = self.inactive();
        for y in 1..self.height - 1 {
            for x in 1..self.width - 1 {
                let idx = y * self.width + x;
                let transformed = self.transform(idx);
                self.buf[inactive][idx] = transformed;
            }
        }
        self.flip();
    }

    fn step_n(&mut self, n: usize) {
        let mut remaining = n;
        let mut seen = HashMap::new();
        seen.insert(self.to_string(), 0);

        for step in 1..=n {
            remaining -= 1;
            self.step();
            let s = self.to_string();
            if let Some(prev_step) = seen.get(&s) {
                let cycle = step - prev_step;
                remaining %= cycle;
                break;
            } else {
                seen.insert(s, step);
            }
        }

        for _ in 0..remaining {
            self.step();
        }
    }

    fn width(&self) -> usize {
        self.width - 2
    }

    fn height(&self) -> usize {
        self.height - 2
    }

    fn count_acres_by_type(&self, t: Acre) -> usize {
        let buf = &self.buf[self.active];
        buf.iter().filter(|a| **a == t).count()
    }

    fn resource_value(&self) -> usize {
        let trees = self.count_acres_by_type(Acre::Trees);
        let lumberyards = self.count_acres_by_type(Acre::Lumberyard);
        trees * lumberyards
    }
}

#[cfg(test)]
mod tests {
    use crate::Map;

    fn load_examples() -> Vec<Map> {
        let scans = std::fs::read_to_string("input/example.txt").unwrap();
        let scans = scans.split("\n\n");

        scans.map(|scan| scan.into()).collect()
    }

    #[test]
    fn test_examples() {
        let maps = load_examples();

        let mut map = maps[0].clone();
        assert_eq!(map.width(), 10);
        assert_eq!(map.height(), 10);
        assert_eq!(map.to_string(), maps[0].to_string());

        for i in 1..=10 {
            map.step();
            assert_eq!(map.to_string(), maps[i].to_string());
        }

        assert_eq!(map.resource_value(), 1147);
    }

    #[test]
    fn test_input() {
        let map = std::fs::read_to_string("input/scan.txt").unwrap();
        let mut map = Map::from(map.as_str());

        for _ in 0..10 {
            map.step();
        }

        assert_eq!(map.resource_value(), 603098);
    }

    #[test]
    fn test_input_part2() {
        let map = std::fs::read_to_string("input/scan.txt").unwrap();
        let mut map = Map::from(map.as_str());

        map.step_n(1_000_000_000);

        assert_eq!(map.resource_value(), 210000);
    }
}
