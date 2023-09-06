#![allow(dead_code)]

use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::{Debug, Display},
};

fn main() {
    println!("Advent of Code 2018 - day 15");
}

static BASE_HP: i64 = 200;
static BASE_DMG: i64 = 3;

#[derive(Debug, Clone, Copy)]
enum Cell {
    Empty,
    Wall,
    Elf(i64),
    Goblin(i64),
}

impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Elf(_), Self::Elf(_)) => true,
            (Self::Goblin(_), Self::Goblin(_)) => true,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Empty,
            '#' => Self::Wall,
            'E' => Self::Elf(BASE_HP),
            'G' => Self::Goblin(BASE_HP),
            _ => panic!("Illegal map symbol: '{value}'!"),
        }
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Cell::Empty => '.',
            Cell::Wall => '#',
            Cell::Elf(_) => 'E',
            Cell::Goblin(_) => 'G',
        };
        write!(f, "{}", c)
    }
}

impl Cell {
    fn is_char(&self) -> bool {
        match self {
            Cell::Elf(_) | Cell::Goblin(_) => true,
            _ => false,
        }
    }

    fn hp(&self) -> i64 {
        match self {
            Self::Elf(hp) | Self::Goblin(hp) => *hp,
            _ => 0,
        }
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
        let mut cells = Vec::new();
        for line in value.lines() {
            height += 1;
            width = 0;
            for c in line.chars() {
                width += 1;
                cells.push(c.into());
            }
        }

        assert_eq!(cells.len(), width * height);

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
            write!(f, "{}", sep).unwrap();
            for x in 0..self.width {
                let cell = &self.cells[self.idx(x, y)];
                write!(f, "{}", cell).unwrap();
            }
            sep = "\n";
        }

        Ok(())
    }
}

impl Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut sep = "";
        for (y, line) in self.to_string().lines().enumerate() {
            write!(f, "{}{} ---", sep, line).unwrap();
            self.cells
                .iter()
                .enumerate()
                .filter(|(idx, cell)| (idx / self.width) == y && cell.is_char())
                .map(|(_, cell)| cell.hp())
                .for_each(|hp| write!(f, " {}", hp).unwrap());
            sep = "\n";
        }

        Ok(())
    }
}

impl Map {
    fn idx(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    fn is_adjacent(&self, idx_a: usize, idx_b: usize) -> bool {
        let dist = if idx_b > idx_a {
            idx_b - idx_a
        } else {
            idx_a - idx_b
        };
        dist == 1 || dist == self.width
    }

    fn hp_left(&self) -> u64 {
        self.cells
            .iter()
            .map(|c| c.hp())
            .sum::<i64>()
            .try_into()
            .unwrap()
    }

    fn neighbors(&self, cell_idx: usize) -> [Option<usize>; 4] {
        [
            if cell_idx / self.width > 0 {
                Some(cell_idx - self.width)
            } else {
                None
            },
            if cell_idx > 0 {
                Some(cell_idx - 1)
            } else {
                None
            },
            if cell_idx < self.cells.len() - 1 {
                Some(cell_idx + 1)
            } else {
                None
            },
            if cell_idx / self.width < self.height {
                Some(cell_idx + self.width)
            } else {
                None
            },
        ]
    }

    fn empty_neighbors(&self, cell_idx: usize) -> [Option<usize>; 4] {
        let mut indices = self.neighbors(cell_idx);

        for i in 0..indices.len() {
            if let Some(cell_idx) = indices[i] {
                if self.cells[cell_idx] != Cell::Empty {
                    indices[i] = None;
                }
            }
        }

        indices
    }

    fn shortest_path_rev(&self, start_idx: usize, target_idx: usize) -> Vec<usize> {
        let mut queue = VecDeque::new();
        let mut seen = HashMap::new();

        queue.push_back(start_idx);
        seen.insert(start_idx, start_idx);

        while let Some(current) = queue.pop_front() {
            if current == target_idx {
                let mut idx = target_idx;
                let mut path = Vec::new();
                while idx != start_idx {
                    path.push(idx);
                    idx = seen[&idx];
                }
                return path;
            }
            let neighbors = self.neighbors(current);
            for neighbor in neighbors {
                if let Some(neighbor) = neighbor {
                    if !seen.contains_key(&neighbor) && self.cells[neighbor] == Cell::Empty {
                        queue.push_back(neighbor);
                        seen.insert(neighbor, current);
                    }
                }
            }
        }
        Vec::with_capacity(0)
    }
}

#[derive(Debug)]
struct BattleResult {
    map: String,
    full_rounds: usize,
    hp_left: u64,
}

fn find_enemies(map: &Map, actor: &Cell) -> Vec<usize> {
    map.cells
        .iter()
        .enumerate()
        .filter(|(_, c)| c.is_char() && *c != actor)
        .map(|(idx, _)| idx)
        .collect()
}

fn find_adjacent_enemies(map: &Map, enemies: &[usize], actor_idx: usize) -> Vec<usize> {
    enemies
        .iter()
        .filter(|e_idx| map.is_adjacent(actor_idx, **e_idx))
        .map(|idx| *idx)
        .collect()
}

fn try_attack(map: &mut Map, enemies: &[usize], actor_idx: usize, elf_atk: i64) -> Option<usize> {
    let mut adjacent_enemies = find_adjacent_enemies(&map, &enemies, actor_idx);
    if !adjacent_enemies.is_empty() {
        adjacent_enemies.sort_by(|a, b| map.cells[*a].hp().cmp(&map.cells[*b].hp()));
        let idx = adjacent_enemies[0];
        let enemy_cell = map.cells[idx];
        let mut updated_cell = match enemy_cell {
            Cell::Elf(hp) => Cell::Elf(hp - BASE_DMG),
            Cell::Goblin(hp) => Cell::Goblin(hp - elf_atk),
            _ => panic!("Should never happen!"),
        };
        if updated_cell.hp() <= 0 {
            updated_cell = Cell::Empty;
        }
        map.cells[idx] = updated_cell;
        Some(idx)
    } else {
        None
    }
}

fn find_next_move(map: &Map, enemies: &[usize], cell_idx: usize) -> Option<usize> {
    let empty_targets: HashSet<usize> = enemies
        .iter()
        .map(|e_idx| map.empty_neighbors(*e_idx))
        .flatten()
        .filter(|empty_neighbor| empty_neighbor.is_some())
        .map(|empty_neighbor| empty_neighbor.unwrap())
        .collect();
    let empty_targets: Vec<usize> = empty_targets.into_iter().collect();

    let mut paths: Vec<(usize, Vec<usize>)> = empty_targets
        .into_iter()
        .map(|t_idx| (t_idx, map.shortest_path_rev(cell_idx, t_idx)))
        .filter(|(_, path)| !path.is_empty())
        .collect();
    if paths.len() == 0 {
        return None;
    }

    paths.sort_by(|(_, a_path), (_, b_path)| a_path.len().cmp(&b_path.len()));
    let fewest_steps = paths[0].1.len();
    let mut paths: Vec<(usize, Vec<usize>)> = paths
        .into_iter()
        .filter(|(_, path)| path.len() == fewest_steps)
        .collect();
    paths.sort_by(|(a_idx, _), (b_idx, _)| a_idx.cmp(b_idx));
    Some(paths[0].1[fewest_steps - 1])
}

fn simulate_battle(map: &str, elf_atk: i64) -> BattleResult {
    let mut map = Map::from(map);

    let mut battle_over = false;
    let mut full_rounds = 0;

    let mut killed_indices = Vec::new();

    loop {
        killed_indices.clear();

        let actors: Vec<usize> = map
            .cells
            .iter()
            .enumerate()
            .filter(|(_, c)| c.is_char())
            .map(|(idx, _)| idx)
            .collect();
        for actor_idx in actors {
            if killed_indices.contains(&actor_idx) {
                // prevent killed cells that have been re-occupied from
                // moving again!
                continue;
            }

            let actor = map.cells[actor_idx];
            if !actor.is_char() {
                continue;
            }

            let enemies = find_enemies(&map, &actor);
            if enemies.is_empty() {
                battle_over = true;
                break;
            }

            if let Some(target) = try_attack(&mut map, &enemies, actor_idx, elf_atk) {
                if map.cells[target] == Cell::Empty {
                    killed_indices.push(target);
                }
                continue; // a successful attack ends the turn for this actor
            }

            if let Some(new_idx) = find_next_move(&map, &enemies, actor_idx) {
                map.cells[actor_idx] = Cell::Empty;
                map.cells[new_idx] = actor;
                if let Some(target) = try_attack(&mut map, &enemies, new_idx, elf_atk) {
                    if map.cells[target] == Cell::Empty {
                        killed_indices.push(target);
                    }
                }
            }
        }

        if battle_over {
            break;
        }

        full_rounds += 1;
    }

    BattleResult {
        map: map.to_string(),
        full_rounds,
        hp_left: map.hp_left(),
    }
}

#[cfg(test)]
mod tests {
    use crate::{simulate_battle, BASE_DMG};

    #[derive(Debug)]
    struct TestData {
        start_map: &'static str,
        end_map: &'static str,
        full_rounds: [usize; 2],
        hp_left: [u64; 2],
        outcome: [u64; 2],
        elf_atk: i64,
    }

    static TEST_DATA: [TestData; 6] = [
        TestData {
            start_map: "\
                #######\n\
                #.G...#\n\
                #...EG#\n\
                #.#.#G#\n\
                #..G#E#\n\
                #.....#\n\
                #######\
            ",
            end_map: "\
                #######\n\
                #G....#\n\
                #.G...#\n\
                #.#.#G#\n\
                #...#.#\n\
                #....G#\n\
                #######\
            ",
            full_rounds: [47, 29],
            hp_left: [590, 172],
            outcome: [27730, 4988],
            elf_atk: 15,
        },
        TestData {
            start_map: "\
                #######\n\
                #G..#E#\n\
                #E#E.E#\n\
                #G.##.#\n\
                #...#E#\n\
                #...E.#\n\
                #######\
            ",
            end_map: "\
                #######\n\
                #...#E#\n\
                #E#...#\n\
                #.E##.#\n\
                #E..#E#\n\
                #.....#\n\
                #######\
            ",
            full_rounds: [37, 10],
            hp_left: [982, 1146],
            outcome: [36334, 11460],
            elf_atk: 15,
        },
        TestData {
            start_map: "\
                #######\n\
                #E..EG#\n\
                #.#G.E#\n\
                #E.##E#\n\
                #G..#.#\n\
                #..E#.#\n\
                #######\
            ",
            end_map: "\
                #######\n\
                #.E.E.#\n\
                #.#E..#\n\
                #E.##.#\n\
                #.E.#.#\n\
                #...#.#\n\
                #######\
            ",
            full_rounds: [46, 33],
            hp_left: [859, 948],
            outcome: [39514, 31284],
            elf_atk: 4,
        },
        TestData {
            start_map: "\
                #######\n\
                #E.G#.#\n\
                #.#G..#\n\
                #G.#.G#\n\
                #G..#.#\n\
                #...E.#\n\
                #######\
            ",
            end_map: "\
                #######\n\
                #G.G#.#\n\
                #.#G..#\n\
                #..#..#\n\
                #...#G#\n\
                #...G.#\n\
                #######\
            ",
            full_rounds: [35, 37],
            hp_left: [793, 94],
            outcome: [27755, 3478],
            elf_atk: 15,
        },
        TestData {
            start_map: "\
                #######\n\
                #.E...#\n\
                #.#..G#\n\
                #.###.#\n\
                #E#G#G#\n\
                #...#G#\n\
                #######\
            ",
            end_map: "\
                #######\n\
                #.....#\n\
                #.#G..#\n\
                #.###.#\n\
                #.#.#.#\n\
                #G.G#G#\n\
                #######\
            ",
            full_rounds: [54, 39],
            hp_left: [536, 166],
            outcome: [28944, 6474],
            elf_atk: 12,
        },
        TestData {
            start_map: "\
                #########\n\
                #G......#\n\
                #.E.#...#\n\
                #..##..G#\n\
                #...##..#\n\
                #...#...#\n\
                #.G...G.#\n\
                #.....G.#\n\
                #########\
            ",
            end_map: "\
                #########\n\
                #.G.....#\n\
                #G.G#...#\n\
                #.G##...#\n\
                #...##..#\n\
                #.G.#...#\n\
                #.......#\n\
                #.......#\n\
                #########\
            ",
            full_rounds: [20, 30],
            hp_left: [937, 38],
            outcome: [18740, 1140],
            elf_atk: 34,
        },
    ];

    #[test]
    fn test_example() {
        for data in &TEST_DATA {
            let result = simulate_battle(data.start_map, BASE_DMG);

            assert_eq!(result.map, data.end_map);
            assert_eq!(result.full_rounds, data.full_rounds[0]);
            assert_eq!(result.hp_left, data.hp_left[0]);
            assert_eq!(result.full_rounds as u64 * result.hp_left, data.outcome[0]);
        }
    }

    #[test]
    fn test_example_part2() {
        for data in &TEST_DATA {
            let result = simulate_battle(data.start_map, data.elf_atk);

            assert_eq!(result.full_rounds, data.full_rounds[1]);
            assert_eq!(result.hp_left, data.hp_left[1]);
            assert_eq!(result.full_rounds as u64 * result.hp_left, data.outcome[1]);
        }
    }

    #[test]
    fn test_input() {
        let map = std::fs::read_to_string("input/map.txt").unwrap();
        let result = simulate_battle(&map, BASE_DMG);

        assert_eq!(result.full_rounds, 81);
        assert_eq!(result.hp_left, 2770);
        assert_eq!(result.full_rounds as u64 * result.hp_left, 224370);
    }

    #[test]
    fn test_input_part2() {
        let map = std::fs::read_to_string("input/map.txt").unwrap();
        let starting_elves = map.chars().filter(|c| *c == 'E').count();

        let result = simulate_battle(&map, 23);
        let finishing_elves = result.map.chars().filter(|c| *c == 'E').count();

        assert_eq!(starting_elves, finishing_elves);
        assert_eq!(result.full_rounds, 31);
        assert_eq!(result.hp_left, 1469);
        assert_eq!(result.full_rounds as u64 * result.hp_left, 45539);
    }
}
