#![allow(dead_code)]

use std::fmt::Display;

fn main() {
    println!("Advent of Code 2020 - day 03");
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

#[derive(Debug, Clone, Copy)]
enum Cell {
    Open,
    Tree,
}

impl TryFrom<char> for Cell {
    type Error = TryFromError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Open),
            '#' => Ok(Self::Tree),
            _ => Err(format!("Illegal cell character: '{}'!", value).into()),
        }
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let chr = match self {
            Self::Open => '.',
            Self::Tree => '#',
        };
        write!(f, "{chr}")
    }
}

struct Map {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
}

impl TryFrom<&str> for Map {
    type Error = TryFromError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut width = usize::MAX;
        let mut height = 0;
        let mut cells = Vec::with_capacity(value.len());

        for line in value.lines() {
            if width != usize::MAX && width != line.len() {
                return Err(format!(
                    "Input map has uneven line lengths! Expected: {}, encountered: {}",
                    width,
                    line.len()
                )
                .into());
            }
            width = line.len();
            height += 1;

            for chr in line.chars() {
                cells.push(chr.try_into()?);
            }
        }

        Ok(Self {
            width,
            height,
            cells,
        })
    }
}

impl Map {
    fn get(&self, x: usize, y: usize) -> Option<Cell> {
        if y >= self.height {
            return None;
        }

        let x = x % self.width;
        Some(self.cells[y * self.width + x])
    }

    fn count_trees_on_route(&self, step_x: usize, step_y: usize) -> usize {
        let mut count = 0;
        let mut x = 0;
        let mut y = 0;

        while y < self.height {
            y += step_y;
            x += step_x;
            if matches!(self.get(x, y), Some(Cell::Tree)) {
                count += 1;
            }
        }

        count
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            if y > 0 {
                writeln!(f).expect("Should not fail!");
            }
            for x in 0..self.width {
                write!(f, "{}", self.cells[y * self.width + x]).expect("Should not fail!");
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::Map;

    #[test]
    fn test_examples() {
        let raw = "\
            ..##.......\n\
            #...#...#..\n\
            .#....#..#.\n\
            ..#.#...#.#\n\
            .#...##..#.\n\
            ..#.##.....\n\
            .#.#.#....#\n\
            .#........#\n\
            #.##...#...\n\
            #...##....#\n\
            .#..#...#.#\
        ";

        let map = Map::try_from(raw).expect("Failed to parse raw map!");
        assert_eq!(raw, format!("{map}"));

        assert_eq!(map.count_trees_on_route(0, 1), 3);
        assert_eq!(map.count_trees_on_route(1, 1), 2);
        assert_eq!(map.count_trees_on_route(2, 1), 1);
        assert_eq!(map.count_trees_on_route(3, 1), 7);

        let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
        let product = slopes
            .into_iter()
            .map(|(step_x, step_y)| map.count_trees_on_route(step_x, step_y))
            .fold(1, |acc, trees| acc * trees);
        assert_eq!(product, 336);
    }

    #[test]
    fn test_input() {
        let raw = std::fs::read_to_string("input/map.txt").unwrap();

        let map = Map::try_from(raw.as_str()).expect("Failed to parse raw map!");
        assert_eq!(raw, format!("{map}"));

        assert_eq!(map.count_trees_on_route(3, 1), 191);

        let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
        let product = slopes
            .into_iter()
            .map(|(step_x, step_y)| map.count_trees_on_route(step_x, step_y))
            .fold(1, |acc, trees| acc * trees);
        assert_eq!(product, 1478615040);
    }
}
