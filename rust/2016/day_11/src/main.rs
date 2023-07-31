#![allow(dead_code)]

use std::{
    collections::{HashSet, VecDeque},
    fmt::Display,
    hash::Hash,
};

use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    println!("Advent of Code 2016 - day 11");
}

lazy_static! {
    static ref RE_FLOOR: Regex = Regex::new(r"^The (\w+) floor contains (.+)\.$").unwrap();
    static ref RE_NOTHING_RELEVANT: Regex = Regex::new(r"^nothing relevant$").unwrap();
    static ref RE_COMPONENTS: Regex = Regex::new(r"^((?U).+)(?:,? and (.+))?$").unwrap();
    static ref RE_COMPONENT: Regex = Regex::new(r"^a ([^\s]+) (\w+)$").unwrap();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Part {
    Generator(char),
    Microchip(char),
}

impl From<&str> for Part {
    fn from(value: &str) -> Self {
        let caps = RE_COMPONENT
            .captures(value)
            .expect("part should match RE_COMPONENT!");
        let element = caps[1]
            .chars()
            .next()
            .expect("Capture shouldn't be empty!")
            .to_ascii_uppercase();

        match &caps[2] {
            "generator" => Self::Generator(element),
            "microchip" => Self::Microchip(element),
            _ => panic!("Unsupported part {}!", value),
        }
    }
}

impl Display for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (element, component) = match self {
            Part::Generator(c) => (c, 'G'),
            Part::Microchip(c) => (c, 'M'),
        };
        write!(f, "{element}{component}")
    }
}

#[derive(Debug, Clone, Default)]
struct Floor {
    parts: HashSet<Part>,
}

impl Floor {
    fn from(value: &str) -> (usize, Self) {
        let caps = RE_FLOOR
            .captures(value)
            .expect("floor should match RE_FLOOR!");
        let floor = Self::parse_floor_number(&caps[1]);
        let contents = &caps[2];

        if RE_NOTHING_RELEVANT.is_match(contents) {
            return (floor, Self::default());
        }

        let caps = RE_COMPONENTS
            .captures(contents)
            .expect("floor should match RE_COMPONENTS!");
        let mut parts = HashSet::new();
        for component in caps[1].split(", ") {
            parts.insert(Part::from(component));
        }
        if let Some(component) = caps.get(2) {
            parts.insert(Part::from(component.as_str()));
        }

        (floor, Self { parts })
    }

    fn parse_floor_number(floor: &str) -> usize {
        match floor {
            "first" => 1,
            "second" => 2,
            "third" => 3,
            "fourth" => 4,
            _ => panic!("illegal floor number {}", floor),
        }
    }

    fn potential_moves_helper(
        size: usize,
        mut options: VecDeque<Part>,
        mut working_set: Vec<Part>,
        perms: &mut Vec<Vec<Part>>,
    ) {
        if working_set.len() == size {
            perms.push(working_set);
            return;
        }

        if options.is_empty() {
            return;
        }

        let first = options.pop_front().unwrap();
        Self::potential_moves_helper(size, options.clone(), working_set.clone(), perms);
        working_set.push(first);
        Self::potential_moves_helper(size, options, working_set.clone(), perms);
    }

    fn potential_moves(&self, min: usize, max: usize) -> Vec<Vec<Part>> {
        let parts: VecDeque<Part> = self.parts.iter().map(|p| *p).collect();
        let mut result = Vec::new();

        for n in min..=max {
            Self::potential_moves_helper(n, parts.clone(), Vec::new(), &mut result);
        }

        result
    }

    fn count_matching_pairs(&self) -> usize {
        let mut pairs = 0;

        self.parts.iter().for_each(|p| match p {
            Part::Generator(c) => {
                if self.parts.contains(&Part::Microchip(*c)) {
                    pairs += 1
                }
            }
            Part::Microchip(_) => (),
        });

        pairs
    }
}

#[derive(Debug, Clone)]
struct Building {
    floors: Vec<Floor>,
    elevator: usize,
}

impl Eq for Building {}

impl PartialEq for Building {
    fn eq(&self, other: &Self) -> bool {
        if self.elevator != other.elevator {
            return false;
        }

        for (s_floor, o_floor) in self.floors.iter().zip(other.floors.iter()) {
            if s_floor.parts.len() != o_floor.parts.len()
                || s_floor.count_matching_pairs() != o_floor.count_matching_pairs()
            {
                return false;
            }
        }

        true
    }
}

impl Hash for Building {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.elevator.hash(state);
        for floor in &self.floors {
            floor.parts.len().hash(state);
            floor.count_matching_pairs().hash(state);
        }
    }
}

impl From<&str> for Building {
    fn from(value: &str) -> Self {
        let mut floors = value
            .lines()
            .map(|line| Floor::from(line))
            .collect::<Vec<(usize, Floor)>>();
        floors.sort_by(|(a_idx, _), (b_idx, _)| b_idx.cmp(&a_idx));
        let floors = floors
            .into_iter()
            .map(|(_, floor)| floor)
            .collect::<Vec<Floor>>();
        let elevator = floors.len() - 1;
        Self { floors, elevator }
    }
}

impl Display for Building {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut parts: Vec<&Part> = self.floors.iter().flat_map(|floor| &floor.parts).collect();
        parts.sort_by(|a, b| {
            let (a_elem, a_component) = match a {
                Part::Generator(c) => (c, 'G'),
                Part::Microchip(c) => (c, 'M'),
            };
            let (b_elem, b_component) = match b {
                Part::Generator(c) => (c, 'G'),
                Part::Microchip(c) => (c, 'M'),
            };
            let cmp = a_elem.cmp(b_elem);
            if cmp.is_eq() {
                a_component.cmp(&b_component)
            } else {
                cmp
            }
        });

        for (idx, floor) in self.floors.iter().enumerate() {
            write!(f, "F{} ", idx + 1)?;
            if self.elevator == idx {
                write!(f, "E  ")?;
            } else {
                write!(f, ".  ")?;
            }
            for part in &parts {
                if floor.parts.contains(part) {
                    write!(f, "{part} ")?;
                } else {
                    write!(f, ".  ")?;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl Building {
    fn is_legal(&self) -> bool {
        for floor in &self.floors {
            let generators: Vec<char> = floor
                .parts
                .iter()
                .filter_map(|part| match part {
                    Part::Generator(c) => Some(*c),
                    Part::Microchip(_) => None,
                })
                .collect();
            let microchips: Vec<char> = floor
                .parts
                .iter()
                .filter_map(|part| match part {
                    Part::Microchip(c) => Some(*c),
                    Part::Generator(_) => None,
                })
                .collect();

            if !generators.is_empty() {
                for element in microchips {
                    if !generators.contains(&element) {
                        return false;
                    }
                }
            }
        }

        true
    }

    fn is_finished(&self) -> bool {
        for idx in 1..self.floors.len() {
            if !self.floors[idx].parts.is_empty() {
                return false;
            }
        }

        true
    }
}

fn solve(start: Building) -> usize {
    let mut seen = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((0, start.clone()));
    seen.insert(start);

    while let Some((steps, state)) = queue.pop_front() {
        let current_floor = &state.floors[state.elevator];
        for pm in current_floor.potential_moves(1, 2) {
            for dir in [0, 1] {
                let mut next_state = state.clone();
                if dir == 0 && next_state.elevator < next_state.floors.len() - 1 {
                    next_state.elevator += 1;
                } else if dir == 1 && next_state.elevator > 0 {
                    next_state.elevator -= 1;
                } else {
                    continue;
                }
                for part in &pm {
                    next_state.floors[state.elevator].parts.remove(part);
                    next_state.floors[next_state.elevator].parts.insert(*part);
                }
                if next_state.is_finished() {
                    return steps + 1;
                }
                if next_state.is_legal() && !seen.contains(&next_state) {
                    queue.push_back((steps + 1, next_state.clone()));
                    seen.insert(next_state);
                }
            }
        }
    }

    todo!()
}

#[cfg(test)]
mod tests {
    use crate::{solve, Building};

    #[test]
    fn test_example() {
        let floors = "\
            The first floor contains a hydrogen-compatible microchip and a lithium-compatible microchip.\n\
            The second floor contains a hydrogen generator.\n\
            The third floor contains a lithium generator.\n\
            The fourth floor contains nothing relevant.\
        ";

        let building = Building::from(floors);
        assert_eq!(solve(building), 11);
    }

    #[test]
    fn test_input() {
        let floors = "\
            The first floor contains a strontium generator, a strontium-compatible microchip, a plutonium generator, and a plutonium-compatible microchip.\n\
            The second floor contains a thulium generator, a ruthenium generator, a ruthenium-compatible microchip, a curium generator, and a curium-compatible microchip.\n\
            The third floor contains a thulium-compatible microchip.\n\
            The fourth floor contains nothing relevant.\
        ";

        let building = Building::from(floors);
        assert_eq!(solve(building), 37);
    }

    #[test]
    fn test_input_part2() {
        let floors = "\
            The first floor contains a elerium generator, a elerium-compatible microchip, a dilithium generator, a dilithium-compatible microchip, a strontium generator, a strontium-compatible microchip, a plutonium generator, and a plutonium-compatible microchip.\n\
            The second floor contains a thulium generator, a ruthenium generator, a ruthenium-compatible microchip, a curium generator, and a curium-compatible microchip.\n\
            The third floor contains a thulium-compatible microchip.\n\
            The fourth floor contains nothing relevant.\
        ";

        let building = Building::from(floors);
        assert_eq!(solve(building), 61);
    }
}
