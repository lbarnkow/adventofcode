#![allow(dead_code)]

use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Display,
};

fn main() {
    println!("Advent of Code 2016 - day 13");
}

#[derive(Debug, Clone, Copy)]
enum Cell {
    Unknown,
    Wall,
    Open,
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Cell::Unknown => '?',
                Cell::Wall => '#',
                Cell::Open => '.',
            }
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: isize,
    y: isize,
}

impl Pos {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn offset(&self, x: isize, y: isize) -> Self {
        Self {
            x: self.x + x,
            y: self.y + y,
        }
    }
}

#[derive(Debug)]
struct Maze {
    fave: isize,
    start: Pos,
    cells: HashMap<Pos, Cell>,
}

impl Maze {
    fn new(fave: isize, start: Pos) -> Self {
        let mut maze = Self {
            fave,
            start,
            cells: HashMap::new(),
        };
        maze.reveal_cell(start);
        maze
    }

    fn neighbors(&self, pos: Pos) -> Vec<Pos> {
        [(1, 0), (-1, 0), (0, 1), (0, -1)]
            .iter()
            .map(|(x, y)| pos.offset(*x, *y))
            .filter(|pos| pos.x >= 0 && pos.y >= 0)
            .collect()
    }

    fn reveal_cell(&mut self, pos: Pos) {
        if self.cells.contains_key(&pos) {
            return;
        }

        let x = pos.x;
        let y = pos.y;

        let cell = (x * x) + (3 * x) + (2 * x * y) + y + (y * y) + self.fave;

        let ones: usize = format!("{cell:b}")
            .chars()
            .map(|c| match c {
                '0' => 0,
                '1' => 1,
                _ => panic!("Binary representation should only contain 0s and 1s!"),
            })
            .sum();

        let cell = if ones % 2 == 0 {
            Cell::Open
        } else {
            Cell::Wall
        };

        self.cells.insert(pos, cell);
    }

    fn get(&self, pos: Pos) -> Cell {
        if pos.x < 0 || pos.y < 0 {
            return Cell::Wall;
        }

        if let Some(cell) = self.cells.get(&pos) {
            return *cell;
        }

        Cell::Unknown
    }

    fn shortest_route_to(&mut self, goal: Pos) -> usize {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();

        queue.push_back((0, self.start));
        while let Some((steps, pos)) = queue.pop_front() {
            if visited.contains(&pos) {
                continue;
            }
            visited.insert(pos);

            for neighbor in self.neighbors(pos) {
                self.reveal_cell(neighbor);

                if neighbor == goal {
                    return steps + 1;
                }
                if let Cell::Open = self.get(neighbor) {
                    queue.push_back((steps + 1, neighbor));
                }
            }
        }

        panic!("Can't reach {goal:?}!");
    }

    fn reachable_positions(&mut self, max_steps: usize) -> usize {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();

        queue.push_back((0, self.start));
        while let Some((steps, pos)) = queue.pop_front() {
            if visited.contains(&pos) {
                continue;
            }
            visited.insert(pos);

            if steps == max_steps {
                continue;
            }

            for neighbor in self.neighbors(pos) {
                self.reveal_cell(neighbor);

                if let Cell::Open = self.get(neighbor) {
                    queue.push_back((steps + 1, neighbor));
                }
            }
        }

        visited.len()
    }
}

impl Display for Maze {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let max = self.cells.keys().fold(Pos { x: 0, y: 0 }, |acc, p| Pos {
            x: acc.x.max(p.x),
            y: acc.y.max(p.y),
        });

        writeln!(f, "  0123456789").expect("Should not fail!");
        for y in 0..=max.y {
            write!(f, "{} ", y).expect("Should not fail!");
            for x in 0..=max.x {
                write!(f, "{}", self.get(Pos { x, y })).expect("Should not fail!");
            }
            write!(f, "\n").expect("Should not fail!");
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{Maze, Pos};

    #[test]
    fn test_example() {
        let mut maze = Maze::new(10, Pos::new(1, 1));
        assert_eq!(maze.shortest_route_to(Pos::new(7, 4)), 11);
    }

    #[test]
    fn test_input() {
        let mut maze = Maze::new(1352, Pos::new(1, 1));
        assert_eq!(maze.shortest_route_to(Pos::new(31, 39)), 90);
    }

    #[test]
    fn test_input_part2() {
        let mut maze = Maze::new(1352, Pos::new(1, 1));
        assert_eq!(maze.reachable_positions(50), 135);
    }
}
