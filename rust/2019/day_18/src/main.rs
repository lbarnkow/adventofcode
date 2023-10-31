#![allow(dead_code)]

use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Display,
};

fn main() {
    println!("Advent of Code 2019 - day 18");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Tile {
    Wall,
    Key(char),
    Door(char),
    Open,
    Actor,
}

impl From<&char> for Tile {
    fn from(value: &char) -> Self {
        match value {
            '#' => Self::Wall,
            c if c.is_ascii_lowercase() => Self::Key(*c),
            c if c.is_ascii_uppercase() => Self::Door(c.to_ascii_lowercase()),
            '.' => Self::Open,
            '@' => Self::Actor,
            _ => panic!("Illegal tile character '{value}'!"),
        }
    }
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        (&value).into()
    }
}

impl From<&Tile> for char {
    fn from(value: &Tile) -> Self {
        match value {
            Tile::Wall => '#',
            Tile::Key(c) => *c,
            Tile::Door(c) => c.to_ascii_uppercase(),
            Tile::Open => '.',
            Tile::Actor => '@',
        }
    }
}

impl From<Tile> for char {
    fn from(value: Tile) -> Self {
        (&value).into()
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", char::from(self))
    }
}

const MAX_ACTORS: usize = 4;

struct Vault {
    tiles: Vec<Tile>,
    width: usize,
    height: usize,
}

impl From<&str> for Vault {
    fn from(value: &str) -> Self {
        let mut tiles = Vec::with_capacity(value.len());
        let mut height = 0;
        let mut width = 0;
        for line in value.lines() {
            height += 1;
            width = line.len();
            for chr in line.chars() {
                tiles.push(chr.into());
            }
        }

        Self {
            tiles,
            width,
            height,
        }
    }
}

impl Display for Vault {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut sep = "";
        for y in 0..self.height {
            write!(f, "{sep}").unwrap();
            for x in 0..self.width {
                let tile: char = self.tiles[y * self.width + x].into();
                write!(f, "{}", tile).unwrap();
            }
            sep = "\n";
        }
        Ok(())
    }
}

impl Vault {
    fn pos_to_idx(&self, pos: Pos) -> usize {
        pos.y * self.width + pos.x
    }

    fn idx_to_pos(&self, idx: usize) -> Pos {
        Pos {
            x: idx % self.width,
            y: idx / self.width,
        }
    }

    fn starting_positions(&self) -> Vec<Pos> {
        let mut list = Vec::with_capacity(MAX_ACTORS);
        for y in 0..self.height {
            for x in 0..self.width {
                if self.tiles[y * self.width + x] == Tile::Actor {
                    list.push(Pos::new(x, y))
                }
            }
        }

        if list.is_empty() || list.len() > MAX_ACTORS {
            panic!("Illegal number of actors in Vault!");
        }

        list
    }

    fn num_keys(&self) -> usize {
        self.tiles
            .iter()
            .filter(|t| matches!(**t, Tile::Key(_)))
            .count()
    }

    fn neighbors(&self, pos: Pos) -> [Option<Pos>; 4] {
        [
            if pos.x > 0 {
                Some(Pos::new(pos.x - 1, pos.y))
            } else {
                None
            },
            if pos.x + 1 < self.width {
                Some(Pos::new(pos.x + 1, pos.y))
            } else {
                None
            },
            if pos.y > 0 {
                Some(Pos::new(pos.x, pos.y - 1))
            } else {
                None
            },
            if pos.y + 1 < self.height {
                Some(Pos::new(pos.x, pos.y + 1))
            } else {
                None
            },
        ]
    }

    fn list_possible_moves(&self, pos: Pos, keys: &[Tile]) -> Vec<Move> {
        let mut moves = Vec::new();

        let mut q = VecDeque::new();
        let mut seen = HashSet::new();

        q.push_back((pos, 0));
        seen.insert(pos);

        while let Some((pos, steps)) = q.pop_front() {
            for neighbor in self.neighbors(pos).into_iter().flatten() {
                if seen.contains(&neighbor) {
                    continue;
                }
                match &self.tiles[self.pos_to_idx(neighbor)] {
                    Tile::Wall => continue,
                    Tile::Open | Tile::Actor => (),
                    key if keys.contains(key) => (), // acquired keys are like open spaces
                    Tile::Door(c) if keys.contains(&c.into()) => (), // opened doors are like open spaces
                    Tile::Door(_) => continue,                       // shut doors are like walls
                    Tile::Key(_) => {
                        // unacquired keys are a possible move
                        moves.push(Move {
                            end: neighbor,
                            steps: steps + 1,
                        });
                        continue;
                    }
                }
                seen.insert(neighbor);
                q.push_back((neighbor, steps + 1));
            }
        }

        moves
    }

    fn shortest_path(&self) -> usize {
        let num_keys = self.num_keys();
        let mut shortest_path = usize::MAX;

        let mut q = VecDeque::new();
        let mut seen = HashMap::new();
        let state = SearchState::new(self.starting_positions(), vec![]);
        q.push_back((state.clone(), 0));
        seen.insert(state, 0);

        while let Some((state, steps)) = q.pop_front() {
            if *seen.get(&state).unwrap_or(&usize::MAX) > steps {
                continue;
            }

            if state.keys.len() == num_keys {
                shortest_path = shortest_path.min(steps);
                continue;
            }

            for (pos_i, pos) in state.pos.iter().enumerate() {
                for p_move in self.list_possible_moves(*pos, &state.keys) {
                    let mut pos = state.pos.clone();
                    pos[pos_i] = p_move.end;
                    let mut keys = state.keys.clone();
                    keys.push(self.tiles[self.pos_to_idx(p_move.end)]);
                    let steps = steps + p_move.steps;
                    let state = SearchState::new(pos, keys);
                    if let Some(prev_steps) = seen.get_mut(&state) {
                        if *prev_steps <= steps {
                            continue;
                        }
                        *prev_steps = steps;
                    } else {
                        seen.insert(state.clone(), steps);
                    }
                    q.push_back((state, steps));
                }
            }
        }

        shortest_path
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default, Eq, Hash)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Move {
    end: Pos,
    steps: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct SearchState {
    pos: Vec<Pos>,
    keys: Vec<Tile>,
}

impl SearchState {
    fn new(pos: Vec<Pos>, mut keys: Vec<Tile>) -> Self {
        keys.sort();
        Self { pos, keys }
    }
}

#[cfg(test)]
mod tests {
    use crate::Vault;

    #[test]
    fn test_examples() {
        let map = "\
            #########\n\
            #b.A.@.a#\n\
            #########\
        ";
        let vault = Vault::from(map);
        assert_eq!(vault.shortest_path(), 8);

        let map = "\
        ########################\n\
        #f.D.E.e.C.b.A.@.a.B.c.#\n\
        ######################.#\n\
        #d.....................#\n\
        ########################\
        ";
        let vault = Vault::from(map);
        assert_eq!(vault.shortest_path(), 86);

        let map = "\
        ########################\n\
        #...............b.C.D.f#\n\
        #.######################\n\
        #.....@.a.B.c.d.A.e.F.g#\n\
        ########################\
        ";
        let vault = Vault::from(map);
        assert_eq!(vault.shortest_path(), 132);

        let map = "\
        #################\n\
        #i.G..c...e..H.p#\n\
        ########.########\n\
        #j.A..b...f..D.o#\n\
        ########@########\n\
        #k.E..a...g..B.n#\n\
        ########.########\n\
        #l.F..d...h..C.m#\n\
        #################\
        ";
        let vault = Vault::from(map);
        assert_eq!(vault.shortest_path(), 136);

        let map = "\
            ########################\n\
            #@..............ac.GI.b#\n\
            ###d#e#f################\n\
            ###A#B#C################\n\
            ###g#h#i################\n\
            ########################\
        ";
        let vault = Vault::from(map);
        assert_eq!(vault.shortest_path(), 81);
    }

    #[test]
    fn test_examples_part2() {
        let map = "\
            #######\n\
            #a.#Cd#\n\
            ##@#@##\n\
            #######\n\
            ##@#@##\n\
            #cB#Ab#\n\
            #######\
        ";
        let vault = Vault::from(map);
        assert_eq!(vault.shortest_path(), 8);

        let map = "\
            ###############\n\
            #d.ABC.#.....a#\n\
            ######@#@######\n\
            ###############\n\
            ######@#@######\n\
            #b.....#.....c#\n\
            ###############\
        ";
        let vault = Vault::from(map);
        assert_eq!(vault.shortest_path(), 24);

        let map = "\
            #############\n\
            #DcBa.#.GhKl#\n\
            #.###@#@#I###\n\
            #e#d#####j#k#\n\
            ###C#@#@###J#\n\
            #fEbA.#.FgHi#\n\
            #############\
        ";
        let vault = Vault::from(map);
        assert_eq!(vault.shortest_path(), 32);

        let map = "\
            #############\n\
            #g#f.D#..h#l#\n\
            #F###e#E###.#\n\
            #dCba@#@BcIJ#\n\
            #############\n\
            #nK.L@#@G...#\n\
            #M###N#H###.#\n\
            #o#m..#i#jk.#\n\
            #############\
        ";
        let vault = Vault::from(map);
        assert_eq!(vault.shortest_path(), 72);
    }

    #[test]
    fn test_input() {
        let map = std::fs::read_to_string("input/vault.txt").unwrap();
        let vault = Vault::from(map.as_str());
        assert_eq!(vault.shortest_path(), 4406);
    }

    #[test]
    fn test_input_part2() {
        let map = std::fs::read_to_string("input/vault2.txt").unwrap();
        let vault = Vault::from(map.as_str());
        assert_eq!(vault.shortest_path(), 1964);
    }
}
