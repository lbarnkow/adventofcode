#![allow(dead_code)]

use std::{collections::HashMap, fmt::Display};

fn main() {
    println!("Advent of Code 2019 - day 10");
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Cell {
    Empty,
    Asteroid,
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Empty,
            '#' => Self::Asteroid,
            x => panic!("Cannot parse '{x}' into Cell!"),
        }
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let chr = match self {
            Cell::Empty => '.',
            Cell::Asteroid => '#',
        };
        write!(f, "{chr}")
    }
}

struct Map {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let mut width = 0;
        let mut height = 0;
        let mut cells = Vec::with_capacity(value.len());

        for line in value.lines() {
            width = 0;
            for chr in line.chars() {
                cells.push(chr.into());
                width += 1;
            }
            height += 1;
        }

        Self {
            width,
            height,
            cells,
        }
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut sep = "";
        for y in 0..self.height {
            write!(f, "{sep}").unwrap();
            for x in 0..self.width {
                let cell = self.cells[y * self.width + x];
                write!(f, "{cell}").unwrap();
            }
            sep = "\n";
        }
        Ok(())
    }
}

type AngleTargets = HashMap<usize, Vec<(usize, usize)>>;

impl Map {
    fn dist((x1, y1): (usize, usize), (x2, y2): (usize, usize)) -> usize {
        x1.abs_diff(x2) + y1.abs_diff(y2)
    }

    fn compute_single_visibility(
        &self,
        x: usize,
        y: usize,
    ) -> (CellVisibility, HashMap<usize, Vec<(usize, usize)>>) {
        if self.cells[y * self.width + x] == Cell::Empty {
            return (CellVisibility::Empty, HashMap::with_capacity(0));
        }

        let mut seen: HashMap<usize, Vec<(usize, usize)>> = HashMap::new();

        for other_y in 0..self.height {
            for other_x in 0..self.height {
                if other_x == x && other_y == y {
                    continue;
                }
                if self.cells[other_y * self.width + other_x] == Cell::Empty {
                    continue;
                }

                let x_f64 = (other_x as isize - x as isize) as f64;
                let y_f64 = (other_y as isize - y as isize) as f64;

                let angle = y_f64.atan2(x_f64);
                let mut angle = (angle * 180f64 / std::f64::consts::PI) + 90f64;
                if angle < 0f64 {
                    angle += 360f64;
                }
                let angle = (angle * 1000f64) as usize;
                if let Some(list) = seen.get_mut(&angle) {
                    list.push((other_x, other_y));
                    list.sort_by(|(x1, y1), (x2, y2)| {
                        Self::dist((x, y), (*x1, *y1)).cmp(&Self::dist((x, y), (*x2, *y2)))
                    });
                } else {
                    seen.insert(angle, vec![(other_x, other_y)]);
                }
            }
        }

        (CellVisibility::Asteroid(seen.len()), seen)
    }

    fn compute_visibility(&self) -> Vec<(CellVisibility, AngleTargets)> {
        let mut vis = Vec::with_capacity(self.cells.len());

        for y in 0..self.height {
            for x in 0..self.width {
                vis.push(self.compute_single_visibility(x, y));
            }
        }

        vis
    }

    fn select_asteroid_with_best_visibility_go(
        &self,
        vis: &[(CellVisibility, AngleTargets)],
    ) -> (usize, usize, usize) {
        let (mut best_x, mut best_y, mut best_count) = (0, 0, 0);
        for y in 0..self.height {
            for x in 0..self.width {
                let (CellVisibility::Asteroid(count), _) = vis[y * self.width + x] else {
                    continue;
                };
                if count > best_count {
                    (best_x, best_y, best_count) = (x, y, count);
                }
            }
        }
        (best_x, best_y, best_count)
    }

    fn select_asteroid_with_best_visibility(&self) -> (usize, usize, usize) {
        self.select_asteroid_with_best_visibility_go(&self.compute_visibility())
    }

    fn vaporize_asteroids(&self) -> Vec<(usize, usize)> {
        let mut vaporized = Vec::new();

        let mut vis = self.compute_visibility();
        let (laser_x, laser_y, _) = self.select_asteroid_with_best_visibility_go(&vis);

        let (_, targets) = &mut vis[laser_y * self.width + laser_x];
        while !targets.is_empty() {
            let mut keys: Vec<usize> = targets.keys().copied().collect();
            keys.sort();
            for angle in keys {
                let targets_for_angle = targets.get_mut(&angle).unwrap();
                vaporized.push(*targets_for_angle.first().unwrap());
                targets_for_angle.remove(0);
                if targets_for_angle.is_empty() {
                    targets.remove(&angle);
                }
            }
        }

        vaporized
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum CellVisibility {
    Empty,
    Asteroid(usize),
}

impl Display for CellVisibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let chr = match self {
            CellVisibility::Empty => '.',
            CellVisibility::Asteroid(count) => match count {
                0 => '0',
                1 => '1',
                2 => '2',
                3 => '3',
                4 => '4',
                5 => '5',
                6 => '6',
                7 => '7',
                8 => '8',
                _ => '9',
            },
        };
        write!(f, "{chr}")
    }
}

struct VisibilityMap {
    width: usize,
    height: usize,
    cells: Vec<CellVisibility>,
}

impl From<&Map> for VisibilityMap {
    fn from(value: &Map) -> Self {
        let cells = value
            .compute_visibility()
            .into_iter()
            .map(|(cv, _)| cv)
            .collect();
        Self {
            width: value.width,
            height: value.height,
            cells,
        }
    }
}

impl Display for VisibilityMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut sep = "";
        for y in 0..self.height {
            write!(f, "{sep}").unwrap();
            for x in 0..self.width {
                let cell = self.cells[y * self.width + x];
                write!(f, "{cell}").unwrap();
            }
            sep = "\n";
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{Map, VisibilityMap};

    #[test]
    fn test_examples() {
        let raw_map = "\
            .#..#\n\
            .....\n\
            #####\n\
            ....#\n\
            ...##\
        ";
        let map = Map::from(raw_map);
        assert_eq!(raw_map, map.to_string());

        let map = VisibilityMap::from(&map);
        let raw_map = "\
        .7..7\n\
        .....\n\
        67775\n\
        ....7\n\
        ...87\
        ";
        assert_eq!(raw_map, map.to_string());

        let raw_map = "\
            ......#.#.\n\
            #..#.#....\n\
            ..#######.\n\
            .#.#.###..\n\
            .#..#.....\n\
            ..#....#.#\n\
            #..#....#.\n\
            .##.#..###\n\
            ##...#..#.\n\
            .#....####\
        ";
        let map = Map::from(raw_map);
        assert_eq!(map.select_asteroid_with_best_visibility(), (5, 8, 33));

        let raw_map = "\
            #.#...#.#.\n\
            .###....#.\n\
            .#....#...\n\
            ##.#.#.#.#\n\
            ....#.#.#.\n\
            .##..###.#\n\
            ..#...##..\n\
            ..##....##\n\
            ......#...\n\
            .####.###.\
        ";
        let map = Map::from(raw_map);
        assert_eq!(map.select_asteroid_with_best_visibility(), (1, 2, 35));

        let raw_map = "\
            .#..#..###\n\
            ####.###.#\n\
            ....###.#.\n\
            ..###.##.#\n\
            ##.##.#.#.\n\
            ....###..#\n\
            ..#.#..#.#\n\
            #..#.#.###\n\
            .##...##.#\n\
            .....#.#..\
        ";
        let map = Map::from(raw_map);
        assert_eq!(map.select_asteroid_with_best_visibility(), (6, 3, 41));

        let raw_map = "\
            .#..##.###...#######\n\
            ##.############..##.\n\
            .#.######.########.#\n\
            .###.#######.####.#.\n\
            #####.##.#.##.###.##\n\
            ..#####..#.#########\n\
            ####################\n\
            #.####....###.#.#.##\n\
            ##.#################\n\
            #####.##.###..####..\n\
            ..######..##.#######\n\
            ####.##.####...##..#\n\
            .#####..#.######.###\n\
            ##...#.##########...\n\
            #.##########.#######\n\
            .####.#.###.###.#.##\n\
            ....##.##.###..#####\n\
            .#.#.###########.###\n\
            #.#.#.#####.####.###\n\
            ###.##.####.##.#..##\
        ";
        let map = Map::from(raw_map);
        assert_eq!(map.select_asteroid_with_best_visibility(), (11, 13, 210));

        let vaporized = map.vaporize_asteroids();
        let total_asteroids = raw_map.chars().filter(|c| *c == '#').count();
        assert_eq!(vaporized.len(), total_asteroids - 1);
        assert_eq!(vaporized[1 - 1], (11, 12));
        assert_eq!(vaporized[2 - 1], (12, 1));
        assert_eq!(vaporized[3 - 1], (12, 2));
        assert_eq!(vaporized[10 - 1], (12, 8));
        assert_eq!(vaporized[20 - 1], (16, 0));
        assert_eq!(vaporized[50 - 1], (16, 9));
        assert_eq!(vaporized[100 - 1], (10, 16));
        assert_eq!(vaporized[199 - 1], (9, 6));
        assert_eq!(vaporized[200 - 1], (8, 2));
        assert_eq!(vaporized[201 - 1], (10, 9));
        assert_eq!(vaporized[299 - 1], (11, 1));

        let (x, y) = vaporized[200 - 1];
        assert_eq!(x * 100 + y, 802);
    }

    #[test]
    fn test_input() {
        let raw_map = std::fs::read_to_string("input/map.txt").unwrap();
        let map = Map::from(raw_map.as_str());
        assert_eq!(map.select_asteroid_with_best_visibility(), (20, 21, 247));

        let vaporized = map.vaporize_asteroids();

        let (x, y) = vaporized[200 - 1];
        assert_eq!(x * 100 + y, 1919);
    }
}
