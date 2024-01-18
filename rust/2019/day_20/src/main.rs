#![allow(dead_code)]

use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    println!("Advent of Code 2019 - day 20");
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum RawTile {
    NotTraversable,
    Wall,
    Passage,
    PortalPart(char),
}

impl From<char> for RawTile {
    fn from(value: char) -> Self {
        match value {
            ' ' => Self::NotTraversable,
            '#' => Self::Wall,
            '.' => Self::Passage,
            c if c.is_ascii_alphabetic() => Self::PortalPart(c),
            _ => panic!("Unrecognized tile character '{value}'!"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Passage,
    Wall,
    Portal([char; 2]),
}

impl Tile {
    fn from_raw_tiles(tiles: [Option<RawTile>; 2]) -> Self {
        match (&tiles[0], &tiles[1]) {
            (Some(RawTile::NotTraversable), None) => Self::Wall,
            (Some(RawTile::Wall), None) => Self::Wall,
            (Some(RawTile::Passage), None) => Self::Passage,
            (Some(RawTile::PortalPart(c1)), Some(RawTile::PortalPart(c2))) => {
                Self::Portal([*c1, *c2])
            }
            (a, b) => panic!("Illegal raw tile combination: {a:?} / {b:?}!"),
        }
    }
}

struct Maze {
    tiles: Vec<Tile>,
    width: usize,
    height: usize,
    portals: HashMap<[char; 2], (usize, usize)>,
}

impl From<&str> for Maze {
    fn from(value: &str) -> Self {
        let width = value.lines().map(str::len).max().unwrap() + 2;
        let height = value.lines().count() + 2;

        let mut raw = vec![RawTile::NotTraversable; width * height];
        for (y, line) in value.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                raw[(y + 1) * width + (x + 1)] = c.into();
            }
        }

        let mut tiles = vec![Tile::Wall; width * height];
        let mut portals = HashMap::new();
        for y in 0..height {
            for x in 0..width {
                let tile = raw[y * width + x];
                let tile = match tile {
                    RawTile::NotTraversable | RawTile::Wall => Tile::Wall,
                    RawTile::Passage => {
                        let above = (raw[(y - 2) * width + x], raw[(y - 1) * width + x]);
                        let below = (raw[(y + 1) * width + x], raw[(y + 2) * width + x]);
                        let left = (raw[y * width + (x - 2)], raw[y * width + (x - 1)]);
                        let right = (raw[y * width + (x + 1)], raw[y * width + (x + 2)]);
                        match (above, below, left, right) {
                            ((RawTile::PortalPart(c1), RawTile::PortalPart(c2)), _, _, _) => {
                                Tile::Portal([c1, c2])
                            }
                            (_, (RawTile::PortalPart(c1), RawTile::PortalPart(c2)), _, _) => {
                                Tile::Portal([c1, c2])
                            }
                            (_, _, (RawTile::PortalPart(c1), RawTile::PortalPart(c2)), _) => {
                                Tile::Portal([c1, c2])
                            }
                            (_, _, _, (RawTile::PortalPart(c1), RawTile::PortalPart(c2))) => {
                                Tile::Portal([c1, c2])
                            }
                            _ => Tile::Passage,
                        }
                    }
                    RawTile::PortalPart(_) => Tile::Wall,
                };
                tiles[y * width + x] = tile;

                if let Tile::Portal(c) = tile {
                    if let Some((_, c2)) = portals.get_mut(&c) {
                        *c2 = y * width + x;
                    } else {
                        portals.insert(c, (y * width + x, y * width + x));
                    }
                }
            }
        }

        Self {
            tiles,
            width,
            height,
            portals,
        }
    }
}

impl Maze {
    fn start_idx(&self) -> usize {
        self.portals.get(&['A', 'A']).unwrap().0
    }

    fn end_idx(&self) -> usize {
        self.portals.get(&['Z', 'Z']).unwrap().0
    }

    fn is_outer_edge(&self, idx: usize) -> bool {
        let x = idx % self.width;
        let y = idx / self.width;

        x == 3 || x == (self.width - 4) || y == 3 || y == (self.height - 4)
    }

    fn possible_moves(&self, from: usize) -> [Option<usize>; 4] {
        let mut result = [None; 4];
        let mut i = 0;

        if let Tile::Portal(c) = self.tiles[from] {
            if let Some((p1, p2)) = self.portals.get(&c) {
                if p1 != p2 {
                    result[i] = if *p1 == from { Some(*p2) } else { Some(*p1) };
                    i += 1;
                }
            }
        }

        for neighbor in [from - 1, from + 1, from - self.width, from + self.width] {
            match self.tiles[neighbor] {
                Tile::Passage | Tile::Portal(_) => {
                    result[i] = Some(neighbor);
                    i += 1;
                }
                _ => (),
            }
        }

        result
    }

    const BASE_DEPTH: usize = 1;

    fn shortest_path(&self) -> usize {
        let start = self.start_idx();
        let end = self.end_idx();

        let mut q = VecDeque::new();
        let mut seen = HashSet::new();

        q.push_back((start, 0));
        seen.insert(start);

        while let Some((idx, steps)) = q.pop_front() {
            for mv in self.possible_moves(idx).into_iter().flatten() {
                if mv == end {
                    return steps + 1;
                }
                if seen.contains(&mv) {
                    continue;
                }
                q.push_back((mv, steps + 1));
                seen.insert(mv);
            }
        }

        panic!("No path found to exit!");
    }

    fn shortest_path_recursive(&self) -> usize {
        let start = self.start_idx();
        let end = self.end_idx();

        let mut q = VecDeque::new();
        let mut seen = HashSet::new();

        q.push_back((start, Self::BASE_DEPTH, 0));
        seen.insert((start, Self::BASE_DEPTH));

        while let Some((idx, depth, steps)) = q.pop_front() {
            for mv in self.possible_moves(idx).into_iter().flatten() {
                if mv == end && depth == Self::BASE_DEPTH {
                    return steps + 1;
                }

                let mv_depth = match (self.tiles[idx], self.tiles[mv]) {
                    (Tile::Portal(_), Tile::Portal(_)) => {
                        if self.is_outer_edge(mv) {
                            depth + 1
                        } else {
                            depth - 1
                        }
                    }
                    _ => depth,
                };

                if mv_depth < Self::BASE_DEPTH {
                    continue; // treat outer passages as walls
                }

                if seen.contains(&(mv, mv_depth)) {
                    continue;
                }

                q.push_back((mv, mv_depth, steps + 1));
                seen.insert((mv, mv_depth));
            }
        }

        panic!("No path found to exit!");
    }
}

#[cfg(test)]
mod tests {
    use crate::Maze;

    #[test]
    fn test_examples() {
        let maze = std::fs::read_to_string("input/example_1.txt").unwrap();
        let maze = Maze::from(maze.as_str());

        assert_eq!(maze.shortest_path(), 23);

        let maze = std::fs::read_to_string("input/example_2.txt").unwrap();
        let maze = Maze::from(maze.as_str());

        assert_eq!(maze.shortest_path(), 58);
    }

    #[test]
    fn test_examples_part2() {
        let maze = std::fs::read_to_string("input/example_1.txt").unwrap();
        let maze = Maze::from(maze.as_str());

        assert_eq!(maze.shortest_path_recursive(), 26);

        let maze = std::fs::read_to_string("input/example_3.txt").unwrap();
        let maze = Maze::from(maze.as_str());

        assert_eq!(maze.shortest_path_recursive(), 396);
    }

    #[test]
    fn test_input() {
        let maze = std::fs::read_to_string("input/maze.txt").unwrap();
        let maze = Maze::from(maze.as_str());

        assert_eq!(maze.shortest_path(), 510);
    }

    #[test]
    fn test_input_part2() {
        let maze = std::fs::read_to_string("input/maze.txt").unwrap();
        let maze = Maze::from(maze.as_str());

        assert_eq!(maze.shortest_path_recursive(), 5652);
    }
}
