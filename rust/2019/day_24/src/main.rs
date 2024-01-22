#![allow(dead_code)]

use std::{
    collections::{HashMap, HashSet},
    fmt::{Debug, Display},
};

fn main() {
    println!("Advent of Code 2019 - day 24");
}

#[derive(Debug)]
struct TryFromError {
    msg: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Cell {
    Empty,
    Infested,
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Empty => '.',
                Self::Infested => '#',
            }
        )
    }
}

impl TryFrom<char> for Cell {
    type Error = TryFromError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Empty),
            '#' => Ok(Self::Infested),
            _ => Err(TryFromError {
                msg: format!("Illegal cell char: '{}'!", value),
            }),
        }
    }
}

const WIDTH: usize = 5;
const HEIGHT: usize = 5;

struct Eris {
    cells: [Cell; WIDTH * HEIGHT],
    history: HashSet<[Cell; WIDTH * HEIGHT]>,
}

impl Display for Eris {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, cell) in self.cells.iter().enumerate() {
            write!(f, "{}", cell)?;
            if (i + 1) % WIDTH == 0 && i / WIDTH < HEIGHT - 1 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

impl TryFrom<&str> for Eris {
    type Error = TryFromError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() != WIDTH * HEIGHT + (HEIGHT - 1) {
            return Err(TryFromError {
                msg: format!(
                    "Input map must be exactly {}x{} chars long (including the newline chars)!",
                    WIDTH, HEIGHT
                ),
            });
        }

        let mut cells = [Cell::Empty; WIDTH * HEIGHT];
        let mut i = 0;
        for line in value.lines() {
            for cell in line.chars() {
                cells[i] = cell.try_into()?;
                i += 1;
            }
        }

        let mut history = HashSet::new();
        history.insert(cells);

        Ok(Self { cells, history })
    }
}

impl Eris {
    fn count_neighboring_bugs(&self, i: usize) -> usize {
        let mut bugs = 0;
        if i % WIDTH != 0 && self.cells[i - 1] == Cell::Infested {
            bugs += 1;
        }
        if (i + 1) % WIDTH != 0 && self.cells[i + 1] == Cell::Infested {
            bugs += 1;
        }
        if i / WIDTH != 0 && self.cells[i - WIDTH] == Cell::Infested {
            bugs += 1;
        }
        if i / WIDTH != HEIGHT - 1 && self.cells[i + WIDTH] == Cell::Infested {
            bugs += 1;
        }
        bugs
    }

    fn step(&mut self) -> ErisStepResult {
        let mut next = self.cells;

        for (i, next_cell) in next.iter_mut().enumerate() {
            let neighboring_bugs = self.count_neighboring_bugs(i);
            *next_cell = match (self.cells[i], neighboring_bugs) {
                (Cell::Empty, bugs) if [1, 2].contains(&bugs) => Cell::Infested,
                (Cell::Infested, bugs) if bugs != 1 => Cell::Empty,
                (cell, _) => cell,
            };
        }
        self.cells = next;

        if self.history.contains(&self.cells) {
            return ErisStepResult::RepeatedConfiguration;
        }

        self.history.insert(self.cells);
        ErisStepResult::NewConfiguration
    }

    fn calc_biodiversity(&self) -> u128 {
        let mut rating = 0;
        let mut cell_value = 1;

        for cell in self.cells {
            if cell == Cell::Infested {
                rating += cell_value;
            }
            cell_value *= 2;
        }

        rating
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum ErisStepResult {
    RepeatedConfiguration,
    NewConfiguration,
}

struct ErisRec {
    levels: HashMap<isize, [Cell; WIDTH * HEIGHT]>,
    age: usize,
}

impl From<Eris> for ErisRec {
    fn from(value: Eris) -> Self {
        let mut levels = HashMap::new();

        let mut cells = value.cells;
        cells[12] = Cell::Empty;

        levels.insert(0, cells);

        let mut result = Self { levels, age: 0 };
        result.ensure_buffer_levels();
        result
    }
}

impl ErisRec {
    fn ensure_buffer_levels(&mut self) {
        let min = *self.levels.keys().min().unwrap();
        let max = *self.levels.keys().max().unwrap();

        if self.count_bugs_by_level(min) > 0 {
            self.levels.insert(min - 1, [Cell::Empty; WIDTH * HEIGHT]);
        }
        if self.count_bugs_by_level(max) > 0 {
            self.levels.insert(max + 1, [Cell::Empty; WIDTH * HEIGHT]);
        }
    }

    fn count_bugs_by_level_and_indices(&self, level: isize, indices: &[usize]) -> usize {
        self.levels.get(&level).map_or(0, |level| {
            indices
                .iter()
                .filter(|i| level[**i] == Cell::Infested)
                .count()
        })
    }

    fn count_bugs_by_level(&self, level: isize) -> usize {
        self.levels.get(&level).map_or(0, |level| {
            level.iter().filter(|c| **c == Cell::Infested).count()
        })
    }

    fn count_bugs_total(&self) -> usize {
        self.levels
            .keys()
            .map(|level| self.count_bugs_by_level(*level))
            .sum()
    }

    fn count_neighboring_bugs(&self, level: isize, index: usize) -> usize {
        let x = index % WIDTH;
        let y = index / WIDTH;

        let mut bugs = 0;

        // up
        if y == 0 {
            bugs += self.count_bugs_by_level_and_indices(level - 1, &[7]);
        } else if y == 3 && x == 2 {
            bugs += self.count_bugs_by_level_and_indices(level + 1, &[20, 21, 22, 23, 24]);
        } else {
            bugs += self.count_bugs_by_level_and_indices(level, &[index - WIDTH]);
        }

        // down
        if y == 4 {
            bugs += self.count_bugs_by_level_and_indices(level - 1, &[17]);
        } else if y == 1 && x == 2 {
            bugs += self.count_bugs_by_level_and_indices(level + 1, &[0, 1, 2, 3, 4]);
        } else {
            bugs += self.count_bugs_by_level_and_indices(level, &[index + WIDTH]);
        }

        // left
        if x == 0 {
            bugs += self.count_bugs_by_level_and_indices(level - 1, &[11]);
        } else if x == 3 && y == 2 {
            bugs += self.count_bugs_by_level_and_indices(level + 1, &[4, 9, 14, 19, 24]);
        } else {
            bugs += self.count_bugs_by_level_and_indices(level, &[index - 1]);
        }

        // right
        if x == 4 {
            bugs += self.count_bugs_by_level_and_indices(level - 1, &[13]);
        } else if x == 1 && y == 2 {
            bugs += self.count_bugs_by_level_and_indices(level + 1, &[0, 5, 10, 15, 20]);
        } else {
            bugs += self.count_bugs_by_level_and_indices(level, &[index + 1]);
        }

        bugs
    }

    fn step(&mut self) {
        let mut next = self.levels.clone();

        let min = *self.levels.keys().min().unwrap();
        let max = *self.levels.keys().max().unwrap();
        for level in min..=max {
            let before = self.levels.get(&level).unwrap();

            for (index, cell) in next.get_mut(&level).unwrap().iter_mut().enumerate() {
                if index == 12 {
                    continue; // skip center cell
                }
                let neighboring_bugs = self.count_neighboring_bugs(level, index);
                *cell = match (before[index], neighboring_bugs) {
                    (Cell::Empty, bugs) if [1, 2].contains(&bugs) => Cell::Infested,
                    (Cell::Infested, bugs) if bugs != 1 => Cell::Empty,
                    (cell, _) => cell,
                };
            }
        }

        self.levels = next;
        self.ensure_buffer_levels();
    }
}

impl Debug for ErisRec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // exclude outer buffer levels
        let min = *self.levels.keys().min().unwrap() + 1;
        let max = *self.levels.keys().max().unwrap() - 1;

        let mut level_sep = "";

        for level in min..=max {
            write!(f, "{level_sep}")?;
            writeln!(f, "Depth {level}:")?;

            let level = self.levels.get(&level).unwrap();
            let mut line_sep = "";
            for y in 0..HEIGHT {
                write!(f, "{line_sep}")?;
                for x in 0..WIDTH {
                    let index = y * WIDTH + x;
                    if x == 2 && y == 2 {
                        write!(f, "?")?;
                    } else {
                        write!(f, "{}", level[index])?;
                    }
                }
                line_sep = "\n";
            }

            level_sep = "\n\n";
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{Eris, ErisRec, ErisStepResult, TryFromError};

    #[test]
    fn test_examples() -> Result<(), TryFromError> {
        let map = "\
            ....#\n\
            #..#.\n\
            #..##\n\
            ..#..\n\
            #....\
        ";
        let mut eris = Eris::try_from(map)?;
        assert_eq!(eris.to_string(), map);

        eris.step();
        let expected = "\
            #..#.\n\
            ####.\n\
            ###.#\n\
            ##.##\n\
            .##..\
        ";
        assert_eq!(eris.to_string(), expected);

        eris.step();
        let expected = "\
            #####\n\
            ....#\n\
            ....#\n\
            ...#.\n\
            #.###\
        ";
        assert_eq!(eris.to_string(), expected);

        eris.step();
        let expected = "\
            #....\n\
            ####.\n\
            ...##\n\
            #.##.\n\
            .##.#\
        ";
        assert_eq!(eris.to_string(), expected);

        eris.step();
        let expected = "\
            ####.\n\
            ....#\n\
            ##..#\n\
            .....\n\
            ##...\
        ";
        assert_eq!(eris.to_string(), expected);

        while eris.step() != ErisStepResult::RepeatedConfiguration {}

        let expected = "\
            .....\n\
            .....\n\
            .....\n\
            #....\n\
            .#...\
        ";
        assert_eq!(eris.to_string(), expected);

        assert_eq!(eris.calc_biodiversity(), 2129920);

        Ok(())
    }

    #[test]
    fn test_examples_part2() -> Result<(), TryFromError> {
        let map = "\
            ....#\n\
            #..#.\n\
            #..##\n\
            ..#..\n\
            #....\
        ";
        let eris = Eris::try_from(map)?;
        let mut eris = ErisRec::from(eris);

        assert_eq!(eris.levels.len(), 3);
        assert_eq!(eris.count_bugs_by_level(-1), 0);
        assert_eq!(eris.count_bugs_by_level(0), 8);
        assert_eq!(eris.count_bugs_by_level(1), 0);
        assert_eq!(eris.count_bugs_total(), 8);

        for _ in 0..10 {
            eris.step();
        }

        let expected = "\
            Depth -5:\n\
            ..#..\n\
            .#.#.\n\
            ..?.#\n\
            .#.#.\n\
            ..#..\n\
            \n\
            Depth -4:\n\
            ...#.\n\
            ...##\n\
            ..?..\n\
            ...##\n\
            ...#.\n\
            \n\
            Depth -3:\n\
            #.#..\n\
            .#...\n\
            ..?..\n\
            .#...\n\
            #.#..\n\
            \n\
            Depth -2:\n\
            .#.##\n\
            ....#\n\
            ..?.#\n\
            ...##\n\
            .###.\n\
            \n\
            Depth -1:\n\
            #..##\n\
            ...##\n\
            ..?..\n\
            ...#.\n\
            .####\n\
            \n\
            Depth 0:\n\
            .#...\n\
            .#.##\n\
            .#?..\n\
            .....\n\
            .....\n\
            \n\
            Depth 1:\n\
            .##..\n\
            #..##\n\
            ..?.#\n\
            ##.##\n\
            #####\n\
            \n\
            Depth 2:\n\
            ###..\n\
            ##.#.\n\
            #.?..\n\
            .#.##\n\
            #.#..\n\
            \n\
            Depth 3:\n\
            ..###\n\
            .....\n\
            #.?..\n\
            #....\n\
            #...#\n\
            \n\
            Depth 4:\n\
            .###.\n\
            #..#.\n\
            #.?..\n\
            ##.#.\n\
            .....\n\
            \n\
            Depth 5:\n\
            ####.\n\
            #..#.\n\
            #.?#.\n\
            ####.\n\
            .....\
        ";
        assert_eq!(format!("{eris:?}"), expected);
        assert_eq!(eris.count_bugs_total(), 99);

        Ok(())
    }

    #[test]
    fn test_input() -> Result<(), TryFromError> {
        let map = std::fs::read_to_string("input/eris.txt").unwrap();
        let mut eris = Eris::try_from(map.as_str())?;

        while eris.step() != ErisStepResult::RepeatedConfiguration {}

        assert_eq!(eris.calc_biodiversity(), 19923473);

        Ok(())
    }

    #[test]
    fn test_input_part2() -> Result<(), TryFromError> {
        let map = std::fs::read_to_string("input/eris.txt").unwrap();
        let eris = Eris::try_from(map.as_str())?;
        let mut eris = ErisRec::from(eris);

        for _ in 0..200 {
            eris.step();
        }

        assert_eq!(eris.count_bugs_total(), 1902);

        Ok(())
    }
}
