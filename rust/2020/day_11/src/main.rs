#![allow(dead_code)]

use std::fmt::Display;

fn main() {
    println!("Advent of Code 2020 - day 11");
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
enum Tile {
    Floor,
    EmptySeat,
    Occupied,
}

impl TryFrom<char> for Tile {
    type Error = TryFromError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Floor),
            'L' => Ok(Self::EmptySeat),
            '#' => Ok(Self::Occupied),
            _ => Err(format!("Illegal input char: '{value}'!").into()),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Self::Floor => '.',
            Self::EmptySeat => 'L',
            Self::Occupied => '#',
        };

        write!(f, "{c}")
    }
}

struct Seats {
    width: usize,
    height: usize,
    tiles: Vec<Tile>,
}

impl TryFrom<&str> for Seats {
    type Error = TryFromError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut width = usize::MAX;
        let mut height = 0;
        let mut tiles = Vec::new();

        for line in value.lines() {
            height += 1;
            if width == usize::MAX {
                width = line.len();
            }
            if width != line.len() {
                return Err(format!(
                    "Input line {height} has wrong length! Expected {width}, got {}!",
                    line.len()
                )
                .into());
            }
            for chr in line.chars() {
                tiles.push(chr.try_into()?);
            }
        }

        Ok(Self {
            width,
            height,
            tiles,
        })
    }
}

impl Display for Seats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            if y != 0 {
                writeln!(f).expect("IO should not fail!");
            }
            for x in 0..self.width {
                let idx = self.pos_to_idx((x, y));
                write!(f, "{}", self.tiles[idx]).expect("IO should not fail!");
            }
        }
        Ok(())
    }
}

impl Seats {
    const fn pos_to_idx(&self, (x, y): (usize, usize)) -> usize {
        y * self.width + x
    }

    fn num_occupied(&self) -> usize {
        self.tiles.iter().filter(|t| **t == Tile::Occupied).count()
    }

    fn num_occupied_neighbors_in_sight(&self, (x, y): (usize, usize), part: Part) -> usize {
        let (width, height) = (self.width as isize, self.height as isize);

        let mut neighbors = 0;
        for (x_off, y_off) in [
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ] {
            let (mut x, mut y) = (x as isize, y as isize);
            loop {
                (x, y) = (x + x_off, y + y_off);
                if x < 0 || y < 0 || x >= width || y >= height {
                    break;
                }
                let idx = self.pos_to_idx((x as usize, y as usize));
                match self.tiles[idx] {
                    Tile::Floor => (),
                    Tile::EmptySeat => break,
                    Tile::Occupied => {
                        neighbors += 1;
                        break;
                    }
                }
                if part == Part::One {
                    break;
                }
            }
        }

        neighbors
    }

    fn step(&mut self, part: Part, tolerance: usize) -> bool {
        let mut changed = false;
        let mut new_tiles = self.tiles.clone();

        for y in 0..self.height {
            for x in 0..self.width {
                let idx = self.pos_to_idx((x, y));
                let neighbors = self.num_occupied_neighbors_in_sight((x, y), part);
                new_tiles[idx] = match (self.tiles[idx], neighbors) {
                    (Tile::EmptySeat, 0) => Tile::Occupied,
                    (Tile::Occupied, neighbors) if neighbors >= tolerance => Tile::EmptySeat,
                    (t, _) => t,
                };
                if new_tiles[idx] != self.tiles[idx] {
                    changed = true;
                }
            }
        }

        self.tiles = new_tiles;
        changed
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Part {
    One,
    Two,
}

#[cfg(test)]
mod tests {
    use crate::{Part, Seats, TryFromError};

    #[test]
    fn test_examples() -> Result<(), TryFromError> {
        let raw = "\
            L.LL.LL.LL\n\
            LLLLLLL.LL\n\
            L.L.L..L..\n\
            LLLL.LL.LL\n\
            L.LL.LL.LL\n\
            L.LLLLL.LL\n\
            ..L.L.....\n\
            LLLLLLLLLL\n\
            L.LLLLLL.L\n\
            L.LLLLL.LL\
        ";
        let mut seats = Seats::try_from(raw)?;

        assert_eq!(seats.to_string(), raw);
        assert_eq!(seats.num_occupied(), 0);

        let changed = seats.step(Part::One, 4);
        assert_eq!(changed, true);
        let raw = "\
            #.##.##.##\n\
            #######.##\n\
            #.#.#..#..\n\
            ####.##.##\n\
            #.##.##.##\n\
            #.#####.##\n\
            ..#.#.....\n\
            ##########\n\
            #.######.#\n\
            #.#####.##\
        ";
        assert_eq!(seats.to_string(), raw);

        let changed = seats.step(Part::One, 4);
        assert_eq!(changed, true);
        let raw = "\
            #.LL.L#.##\n\
            #LLLLLL.L#\n\
            L.L.L..L..\n\
            #LLL.LL.L#\n\
            #.LL.LL.LL\n\
            #.LLLL#.##\n\
            ..L.L.....\n\
            #LLLLLLLL#\n\
            #.LLLLLL.L\n\
            #.#LLLL.##\
        ";
        assert_eq!(seats.to_string(), raw);

        let changed = seats.step(Part::One, 4);
        assert_eq!(changed, true);
        let raw = "\
            #.##.L#.##\n\
            #L###LL.L#\n\
            L.#.#..#..\n\
            #L##.##.L#\n\
            #.##.LL.LL\n\
            #.###L#.##\n\
            ..#.#.....\n\
            #L######L#\n\
            #.LL###L.L\n\
            #.#L###.##\
        ";
        assert_eq!(seats.to_string(), raw);

        let changed = seats.step(Part::One, 4);
        assert_eq!(changed, true);
        let raw = "\
            #.#L.L#.##\n\
            #LLL#LL.L#\n\
            L.L.L..#..\n\
            #LLL.##.L#\n\
            #.LL.LL.LL\n\
            #.LL#L#.##\n\
            ..L.L.....\n\
            #L#LLLL#L#\n\
            #.LLLLLL.L\n\
            #.#L#L#.##\
        ";
        assert_eq!(seats.to_string(), raw);

        let changed = seats.step(Part::One, 4);
        assert_eq!(changed, true);
        let raw = "\
            #.#L.L#.##\n\
            #LLL#LL.L#\n\
            L.#.L..#..\n\
            #L##.##.L#\n\
            #.#L.LL.LL\n\
            #.#L#L#.##\n\
            ..L.L.....\n\
            #L#L##L#L#\n\
            #.LLLLLL.L\n\
            #.#L#L#.##\
        ";
        assert_eq!(seats.to_string(), raw);

        let changed = seats.step(Part::One, 4);
        assert_eq!(changed, false);
        assert_eq!(seats.to_string(), raw);
        assert_eq!(seats.num_occupied(), 37);

        Ok(())
    }

    #[test]
    fn test_examples_part2() -> Result<(), TryFromError> {
        let raw = "\
            L.LL.LL.LL\n\
            LLLLLLL.LL\n\
            L.L.L..L..\n\
            LLLL.LL.LL\n\
            L.LL.LL.LL\n\
            L.LLLLL.LL\n\
            ..L.L.....\n\
            LLLLLLLLLL\n\
            L.LLLLLL.L\n\
            L.LLLLL.LL\
        ";
        let mut seats = Seats::try_from(raw)?;

        assert_eq!(seats.to_string(), raw);
        assert_eq!(seats.num_occupied(), 0);

        let mut changed = false;
        for _ in 0..10_000 {
            changed = seats.step(Part::Two, 5);
            if !changed {
                break;
            }
        }

        assert_eq!(changed, false);
        assert_eq!(seats.num_occupied(), 26);

        Ok(())
    }

    #[test]
    fn test_input() -> Result<(), TryFromError> {
        let raw = std::fs::read_to_string("input/seats.txt").unwrap();
        let mut seats = Seats::try_from(raw.as_str())?;

        assert_eq!(seats.to_string(), raw);

        let mut changed = false;
        for _ in 0..10_000 {
            changed = seats.step(Part::One, 4);
            if !changed {
                break;
            }
        }

        assert_eq!(changed, false);
        assert_eq!(seats.num_occupied(), 2222);

        Ok(())
    }

    #[test]
    fn test_input_part2() -> Result<(), TryFromError> {
        let raw = std::fs::read_to_string("input/seats.txt").unwrap();
        let mut seats = Seats::try_from(raw.as_str())?;

        assert_eq!(seats.to_string(), raw);

        let mut changed = false;
        for _ in 0..10_000 {
            changed = seats.step(Part::Two, 5);
            if !changed {
                break;
            }
        }

        assert_eq!(changed, false);
        assert_eq!(seats.num_occupied(), 2032);

        Ok(())
    }
}
