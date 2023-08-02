#![allow(dead_code)]

use std::fmt::Display;

fn main() {
    println!("Advent of Code 2016 - day 18");
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Safe,
    Trap,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Safe,
            '^' => Self::Trap,
            _ => panic!("Illegal tile character!"),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Safe => '.',
                Self::Trap => '^',
            }
        )
    }
}

#[derive(Debug, Clone)]
struct Row {
    tiles: Vec<Tile>,
}

impl From<&str> for Row {
    fn from(value: &str) -> Self {
        Self {
            tiles: value.chars().map(|c| Tile::from(c)).collect(),
        }
    }
}

impl Display for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for tile in &self.tiles {
            write!(f, "{}", tile).expect("Should not fail!");
        }

        Ok(())
    }
}

impl Row {
    fn compute_successor(&self) -> Self {
        let mut new_row = Vec::with_capacity(self.tiles.len());
        let max_idx = self.tiles.len() - 1;

        for i in 0..=max_idx {
            let left = if i > 0 { self.tiles[i - 1] } else { Tile::Safe };
            let right = if i < max_idx {
                self.tiles[i + 1]
            } else {
                Tile::Safe
            };
            let center = self.tiles[i];

            let tile = match (left, center, right) {
                (Tile::Trap, Tile::Trap, Tile::Safe) => Tile::Trap,
                (Tile::Safe, Tile::Trap, Tile::Trap) => Tile::Trap,
                (Tile::Trap, Tile::Safe, Tile::Safe) => Tile::Trap,
                (Tile::Safe, Tile::Safe, Tile::Trap) => Tile::Trap,
                _ => Tile::Safe,
            };
            new_row.push(tile);
        }

        Self { tiles: new_row }
    }

    fn count_tiles_like(&self, t: Tile) -> usize {
        self.tiles.iter().filter(|tile| **tile == t).count()
    }
}

struct Dungeon {
    rows: Vec<Row>,
}

impl Dungeon {
    fn new(initial_row: Row, row_count: usize) -> Self {
        let mut rows = Vec::with_capacity(row_count);

        let mut buf = initial_row;
        for _ in 1..row_count {
            let tmp = buf.compute_successor();
            rows.push(buf);
            buf = tmp;
        }
        rows.push(buf);

        Self { rows }
    }

    fn count_tiles_like(&self, t: Tile) -> usize {
        self.rows.iter().map(|row| row.count_tiles_like(t)).sum()
    }
}

impl Display for Dungeon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.rows {
            writeln!(f, "{}", row).expect("Should not fail!");
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{Dungeon, Row, Tile};

    #[test]
    fn test_example() {
        let expected = "\
            ..^^.\n\
            .^^^^\n\
            ^^..^\n\
        ";

        let initial_row = "..^^.";
        let initial_row = Row::from(initial_row);
        let dungeon = Dungeon::new(initial_row, 3);

        assert_eq!(dungeon.to_string(), expected);
        assert_eq!(dungeon.count_tiles_like(Tile::Safe), 6);

        let expected = "\
            .^^.^.^^^^\n\
            ^^^...^..^\n\
            ^.^^.^.^^.\n\
            ..^^...^^^\n\
            .^^^^.^^.^\n\
            ^^..^.^^..\n\
            ^^^^..^^^.\n\
            ^..^^^^.^^\n\
            .^^^..^.^^\n\
            ^^.^^^..^^\n\
        ";

        let initial_row = ".^^.^.^^^^";
        let initial_row = Row::from(initial_row);
        let dungeon = Dungeon::new(initial_row, 10);

        assert_eq!(dungeon.to_string(), expected);
        assert_eq!(dungeon.count_tiles_like(Tile::Safe), 38);
    }

    #[test]
    fn test_input() {
        let initial_row = std::fs::read_to_string("input/dungeon.txt").unwrap();
        let initial_row = Row::from(initial_row.as_str());

        let dungeon = Dungeon::new(initial_row, 40);
        assert_eq!(dungeon.count_tiles_like(Tile::Safe), 1939);
    }

    #[test]
    fn test_input_part2() {
        let initial_row = std::fs::read_to_string("input/dungeon.txt").unwrap();
        let initial_row = Row::from(initial_row.as_str());

        let dungeon = Dungeon::new(initial_row, 400000);
        assert_eq!(dungeon.count_tiles_like(Tile::Safe), 19999535);
    }
}
