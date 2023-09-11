#![allow(dead_code)]

use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Display,
};

fn main() {
    println!("Advent of Code 2018 - day 20");
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Dir {
    North,
    East,
    South,
    West,
}
static DIRS: [Dir; 4] = [Dir::North, Dir::East, Dir::South, Dir::West];

impl From<char> for Dir {
    fn from(value: char) -> Self {
        match value {
            'N' => Self::North,
            'E' => Self::East,
            'S' => Self::South,
            'W' => Self::West,
            _ => panic!("Illegal direction: {value}!"),
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
struct Pos {
    x: i64,
    y: i64,
}

impl Pos {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    fn offset(&self, dir: Dir) -> Self {
        match dir {
            Dir::North => Self {
                x: self.x,
                y: self.y - 1,
            },
            Dir::East => Self {
                x: self.x + 1,
                y: self.y,
            },
            Dir::South => Self {
                x: self.x,
                y: self.y + 1,
            },
            Dir::West => Self {
                x: self.x - 1,
                y: self.y,
            },
        }
    }

    fn offset_mut(&mut self, dir: Dir) {
        let offset = self.offset(dir);
        self.x = offset.x;
        self.y = offset.y;
    }
}

struct Map {
    rooms: HashSet<Pos>,
    doors: HashSet<(Pos, Pos)>,
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let mut iter = value.chars();

        let mut rooms = HashSet::new();
        let mut doors = HashSet::new();
        rooms.insert(Pos::default());

        let mut aps: Vec<Vec<Vec<Pos>>> = vec![vec![vec![Pos::default()]]];
        let mut active_level = 0;
        let mut active_branches: Vec<usize> = vec![0];

        assert_eq!(iter.next().unwrap(), '^');
        while let Some(ch) = iter.next() {
            match ch {
                ch if ch.is_ascii_alphabetic() => aps[active_level]
                    [*active_branches.last().unwrap()]
                .iter_mut()
                .for_each(|p| {
                    let before = *p;
                    p.offset_mut(ch.into());
                    rooms.insert(*p);
                    doors.insert((before, *p));
                    doors.insert((*p, before));
                }),
                '(' => {
                    active_level += 1;
                    let active_branch = active_branches.last().unwrap();
                    let prefixes = aps[active_level - 1][*active_branch].clone();
                    active_branches.push(0);
                    aps.push(vec![prefixes]);
                }
                '|' => {
                    let active_branch_prev_level = active_branches[active_branches.len() - 2];
                    let active_branch = active_branches.last_mut().unwrap();
                    *active_branch += 1;
                    let prefixes = aps[active_level - 1][active_branch_prev_level].clone();
                    aps[active_level].push(prefixes);
                }
                ')' => {
                    active_branches.pop();
                    let branches = aps[active_level]
                        .iter()
                        .flatten()
                        .map(|p| *p)
                        .collect::<HashSet<Pos>>();
                    aps.pop();
                    active_level -= 1;
                    let active_branch = active_branches.last_mut().unwrap();
                    aps[active_level][*active_branch].clear();
                    branches
                        .into_iter()
                        .for_each(|p| aps[active_level][*active_branch].push(p));
                }
                '$' => break,
                _ => panic!("Illegal character: '{ch}'!"),
            }
        }
        assert_eq!(active_branches.len(), 1);
        assert_eq!(active_branches[0], 0);
        assert_eq!(aps.len(), 1);
        assert_eq!(iter.next(), None);

        Self { rooms, doors }
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (min_x, min_y) = self.rooms.iter().fold((i64::MAX, i64::MAX), |acc, p| {
            (acc.0.min(p.x), acc.1.min(p.y))
        });
        let (max_x, max_y) = self.rooms.iter().fold((i64::MIN, i64::MIN), |acc, p| {
            (acc.0.max(p.x), acc.1.max(p.y))
        });

        for y in min_y..=max_y {
            write!(f, "#").unwrap();
            for x in min_x..=max_x {
                let p = Pos::new(x, y);
                let d = if self.doors.contains(&(p, p.offset(Dir::North))) {
                    '-'
                } else {
                    '#'
                };
                write!(f, "{d}#",).unwrap();
            }
            writeln!(f).unwrap();
            write!(f, "#").unwrap();
            for x in min_x..=max_x {
                let p = Pos::new(x, y);
                let d = if self.doors.contains(&(p, p.offset(Dir::East))) {
                    '|'
                } else {
                    '#'
                };
                let r = if x == 0 && y == 0 {
                    'X'
                } else if self.rooms.contains(&p) {
                    '.'
                } else {
                    '#'
                };
                write!(f, "{r}{d}",).unwrap();
            }
            writeln!(f).unwrap();
        }

        write!(f, "#").unwrap();
        for _ in min_x..=max_x {
            write!(f, "##",).unwrap();
        }

        Ok(())
    }
}

impl Map {
    fn min_dists(&self) -> HashMap<Pos, usize> {
        let mut queue = VecDeque::with_capacity(self.rooms.len());
        let mut seen: HashMap<Pos, usize> = HashMap::with_capacity(self.rooms.len());

        seen.insert(Pos::default(), 0);
        queue.push_back((Pos::default(), 0));

        while let Some((pos, steps)) = queue.pop_front() {
            DIRS.iter().for_each(|dir| {
                let next = pos.offset(*dir);
                if !seen.contains_key(&next) && self.doors.contains(&(pos, next)) {
                    seen.insert(next, steps + 1);
                    queue.push_back((next, steps + 1));
                }
            })
        }

        seen
    }

    fn dist_to_farthest_room(&self) -> usize {
        let dists = self.min_dists();
        dists.into_iter().fold(0, |acc, (_, dist)| dist.max(acc))
    }

    fn num_rooms_with_min_dist(&self, min_dist: usize) -> usize {
        let dists = self.min_dists();
        dists
            .into_iter()
            .filter(|(_, dist)| *dist >= min_dist)
            .count()
    }
}

#[cfg(test)]
mod tests {
    use crate::Map;

    #[test]
    fn test_examples() {
        let regex = "^WNE$";
        let map = Map::from(regex);
        let expected = "\
            #####\n\
            #.|.#\n\
            #-###\n\
            #.|X#\n\
            #####\
        ";
        assert_eq!(&map.to_string(), expected);
        assert_eq!(map.dist_to_farthest_room(), 3);

        let regex = "^N(E|W)N$";
        let map = Map::from(regex);
        let expected = "\
            #######\n\
            #.###.#\n\
            #-###-#\n\
            #.|.|.#\n\
            ###-###\n\
            ###X###\n\
            #######\
        ";
        assert_eq!(&map.to_string(), expected);
        assert_eq!(map.dist_to_farthest_room(), 3);

        let regex = "^ENWWW(NEEE|SSE(EE|N))$";
        let map = Map::from(regex);
        let expected = "\
            #########\n\
            #.|.|.|.#\n\
            #-#######\n\
            #.|.|.|.#\n\
            #-#####-#\n\
            #.#.#X|.#\n\
            #-#-#####\n\
            #.|.|.|.#\n\
            #########\
        ";
        assert_eq!(&map.to_string(), expected);
        assert_eq!(map.dist_to_farthest_room(), 10);

        let regex = "^(NEWS|WNSE|)$";
        let map = Map::from(regex);
        let expected = "\
            #######\n\
            #.#.|.#\n\
            #-#-###\n\
            #.|X###\n\
            #######\
        ";
        assert_eq!(&map.to_string(), expected);
        assert_eq!(map.dist_to_farthest_room(), 2);

        let regex = "^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$";
        let map = Map::from(regex);
        let expected = "\
            ###########\n\
            #.|.#.|.#.#\n\
            #-###-#-#-#\n\
            #.|.|.#.#.#\n\
            #-#####-#-#\n\
            #.#.#X|.#.#\n\
            #-#-#####-#\n\
            #.#.|.|.|.#\n\
            #-###-###-#\n\
            #.|.|.#.|.#\n\
            ###########\
        ";
        assert_eq!(&map.to_string(), expected);
        assert_eq!(map.dist_to_farthest_room(), 18);

        let regex = "^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$";
        let map = Map::from(regex);
        let expected = "\
            #############\n\
            #.|.|.|.|.|.#\n\
            #-#####-###-#\n\
            #.#.|.#.#.#.#\n\
            #-#-###-#-#-#\n\
            #.#.#.|.#.|.#\n\
            #-#-#-#####-#\n\
            #.#.#.#X|.#.#\n\
            #-#-#-###-#-#\n\
            #.|.#.|.#.#.#\n\
            ###-#-###-#-#\n\
            #.|.#.|.|.#.#\n\
            #############\
        ";
        assert_eq!(&map.to_string(), expected);
        assert_eq!(map.dist_to_farthest_room(), 23);

        let regex = "^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$";
        let map = Map::from(regex);
        let expected = "\
            ###############\n\
            #.|.|.|.#.|.|.#\n\
            #-###-###-#-#-#\n\
            #.|.#.|.|.#.#.#\n\
            #-#########-#-#\n\
            #.#.|.|.|.|.#.#\n\
            #-#-#########-#\n\
            #.#.#.|X#.|.#.#\n\
            ###-#-###-#-#-#\n\
            #.|.#.#.|.#.|.#\n\
            #-###-#####-###\n\
            #.|.#.|.|.#.#.#\n\
            #-#-#####-#-#-#\n\
            #.#.|.|.|.#.|.#\n\
            ###############\
        ";
        assert_eq!(&map.to_string(), expected);
        assert_eq!(map.dist_to_farthest_room(), 31);
    }

    #[test]
    fn test_input() {
        let regex = std::fs::read_to_string("input/routes.txt").unwrap();
        let map = Map::from(regex.as_str());

        assert_eq!(map.rooms.len(), 10_000);
        assert_eq!(map.dist_to_farthest_room(), 3512);
        assert_eq!(map.num_rooms_with_min_dist(1000), 8660);
    }
}
