#![allow(dead_code)]

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, VecDeque},
    fmt::Display,
};

fn main() {
    println!("Advent of Code 2016 - day 24");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn dist(&self, other: &Pos) -> usize {
        let min_x = self.x.min(other.x);
        let max_x = self.x.max(other.x);
        let min_y = self.y.min(other.y);
        let max_y = self.y.max(other.y);
        (max_x - min_x) + (max_y - min_y)
    }

    fn neighbors(&self) -> Vec<Self> {
        let mut neighbors = Vec::with_capacity(4);

        if self.x > 0 {
            neighbors.push(Self {
                x: self.x - 1,
                y: self.y,
            });
        }
        if self.y > 0 {
            neighbors.push(Self {
                x: self.x,
                y: self.y - 1,
            });
        }
        neighbors.push(Self {
            x: self.x + 1,
            y: self.y,
        });
        neighbors.push(Self {
            x: self.x,
            y: self.y + 1,
        });

        neighbors
    }
}

#[derive(Debug, Clone, Copy)]
enum Cell {
    Wall,
    Open,
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Cell::Wall => '#',
                Cell::Open => ' ',
            }
        )
    }
}

#[derive(Debug)]
struct Maze {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
    pois: Vec<Pos>,
}

impl From<&str> for Maze {
    fn from(value: &str) -> Self {
        let mut cells = Vec::new();
        let mut pois = Vec::new();
        let mut height = 0;
        let mut width = 0;
        let mut i = 0;

        for line in value.lines() {
            height += 1;
            width = line.len();
            for c in line.chars() {
                let cell = match c {
                    '#' => Cell::Wall,
                    '.' => Cell::Open,
                    c if c.is_numeric() => {
                        pois.push((c.to_digit(10).unwrap(), i));
                        Cell::Open
                    }
                    _ => panic!("Illegal character!"),
                };
                cells.push(cell);
                i += 1;
            }
        }

        pois.sort_by(|(a, _), (b, _)| a.cmp(b));
        let pois = pois
            .iter()
            .map(|(_, cell_idx)| Pos::new(cell_idx % width, cell_idx / width))
            .collect();

        Self {
            width,
            height,
            cells,
            pois,
        }
    }
}

impl Display for Maze {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut pois = HashMap::new();
        self.pois.iter().enumerate().for_each(|(idx, p)| {
            pois.insert(self.pos_to_idx(*p), idx);
        });

        for y in 0..self.height {
            for x in 0..self.width {
                let idx = y * self.width + x;
                if let Some(num) = pois.get(&idx) {
                    write!(f, "{}", *num).unwrap();
                } else {
                    write!(f, "{}", self.cells[idx]).unwrap();
                }
            }
            writeln!(f, "").unwrap();
        }
        Ok(())
    }
}

impl Maze {
    fn idx_to_pos(&self, idx: usize) -> Pos {
        Pos {
            x: idx % self.width,
            y: idx / self.width,
        }
    }

    fn pos_to_idx(&self, pos: Pos) -> usize {
        pos.y * self.width + pos.x
    }

    fn possible_steps(&self, from: Pos) -> Vec<Pos> {
        if let Cell::Wall = self.cells[self.pos_to_idx(from)] {
            panic!("Walls don't allow any moves!");
        }

        let mut steps = Vec::with_capacity(4);

        for neighbor in from.neighbors() {
            if let Cell::Open = self.cells[self.pos_to_idx(neighbor)] {
                steps.push(neighbor);
            }
        }

        steps
    }

    fn shortest_distance_between(&self, a: Pos, b: Pos) -> usize {
        [a, b].iter().for_each(|pos| {
            if let Cell::Wall = self.cells[self.pos_to_idx(*pos)] {
                panic!("{pos:?} is a Wall!");
            }
        });

        let mut queue = BinaryHeap::new();
        let mut seen = HashMap::new();
        queue.push(Reverse(ShortestPathState {
            current_pos: a,
            steps: 0,
            dist: a.dist(&b),
        }));
        seen.insert(a, 0);
        while let Some(Reverse(state)) = queue.pop() {
            let steps = state.steps + 1;
            for next in self.possible_steps(state.current_pos) {
                if next == b {
                    return steps;
                }

                if let Some(seen_steps) = seen.get(&next) {
                    if steps >= *seen_steps {
                        continue;
                    }
                }
                queue.push(Reverse(ShortestPathState {
                    current_pos: next,
                    steps: steps,
                    dist: next.dist(&b),
                }));
                seen.insert(next, steps);
            }
        }

        panic!("No path between {a:?} and {a:?}!")
    }

    fn calculate_dist_between_all_pois(&self) -> HashMap<(Pos, Pos), usize> {
        let mut dists = HashMap::new();

        for i in 0..self.pois.len() {
            for j in i + 1..self.pois.len() {
                let a = self.pois[i];
                let b = self.pois[j];
                let dist = self.shortest_distance_between(a, b);
                dists.insert((a, b), dist);
                dists.insert((b, a), dist);
            }
        }

        dists
    }

    fn calculate_all_poi_routes_starting_at_0_go(
        &self,
        remaining: &mut VecDeque<Pos>,
        route: &mut Vec<Pos>,
        routes: &mut Vec<Vec<Pos>>,
    ) {
        if remaining.is_empty() {
            routes.push(route.clone());
            return;
        }

        for _ in 0..remaining.len() {
            let next = remaining.pop_front().unwrap();
            route.push(next);
            self.calculate_all_poi_routes_starting_at_0_go(remaining, route, routes);
            route.pop();
            remaining.push_back(next);
        }
    }

    fn calculate_all_poi_routes_starting_at_0(&self) -> Vec<Vec<Pos>> {
        let mut routes = Vec::new();

        let remaining: Vec<Pos> = self.pois[1..].into();
        let mut route = Vec::with_capacity(self.pois.len());
        route.push(self.pois[0]);

        self.calculate_all_poi_routes_starting_at_0_go(
            &mut remaining.into(),
            &mut route,
            &mut routes,
        );

        routes
    }

    fn calculate_route_len(&self, dists: &HashMap<(Pos, Pos), usize>, route: &Vec<Pos>) -> usize {
        if route.len() <= 1 {
            return 0;
        }

        let mut cost = 0;
        for i in 0..(route.len() - 1) {
            let part = (route[i], route[i + 1]);
            cost += dists.get(&part).unwrap();
        }
        cost
    }

    fn shortest_path_through_all_pois_starting_at_0(&self) -> usize {
        let dists = self.calculate_dist_between_all_pois();
        // with just 7 points of interest calculating all permutations isn't too bad.
        let routes = self.calculate_all_poi_routes_starting_at_0();

        routes
            .iter()
            .map(|route| self.calculate_route_len(&dists, route))
            .min()
            .unwrap()
    }

    fn shortest_loop_through_all_pois_starting_at_0(&self) -> usize {
        let dists = self.calculate_dist_between_all_pois();
        // with just 7 points of interest calculating all permutations isn't too bad.
        let mut routes = self.calculate_all_poi_routes_starting_at_0();
        for route in routes.iter_mut() {
            route.push(self.pois[0]);
        }

        routes
            .iter()
            .map(|route| self.calculate_route_len(&dists, route))
            .min()
            .unwrap()
    }
}

#[derive(Debug, Clone)]
struct ShortestPathState {
    current_pos: Pos,
    steps: usize,
    dist: usize,
}

impl PartialEq for ShortestPathState {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}

impl Eq for ShortestPathState {}

impl PartialOrd for ShortestPathState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ShortestPathState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_cost = self.steps + self.dist;
        let other_cost = other.steps + other.dist;
        self_cost.cmp(&other_cost)
    }
}

#[cfg(test)]
mod tests {
    use crate::Maze;

    #[test]
    fn test_example() {
        let map = "\
            ###########\n\
            #0.1.....2#\n\
            #.#######.#\n\
            #4.......3#\n\
            ###########\
        ";

        let maze = Maze::from(map);

        assert_eq!(
            maze.shortest_distance_between(maze.pois[0], maze.pois[4]),
            2
        );
        assert_eq!(
            maze.shortest_distance_between(maze.pois[4], maze.pois[1]),
            4
        );
        assert_eq!(
            maze.shortest_distance_between(maze.pois[1], maze.pois[2]),
            6
        );
        assert_eq!(
            maze.shortest_distance_between(maze.pois[2], maze.pois[2]),
            2
        );

        assert_eq!(maze.shortest_path_through_all_pois_starting_at_0(), 14);
    }

    #[test]
    fn test_input() {
        let map = std::fs::read_to_string("input/map.txt").unwrap();
        let maze = Maze::from(map.as_str());
        assert_eq!(maze.shortest_path_through_all_pois_starting_at_0(), 464);
        assert_eq!(maze.shortest_loop_through_all_pois_starting_at_0(), 652);
    }
}
