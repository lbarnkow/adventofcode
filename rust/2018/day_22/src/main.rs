#![allow(dead_code)]

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    fmt::Display,
};

fn main() {
    println!("Advent of Code 2018 - day 22");
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn offset_mut(&mut self, x: isize, y: isize) {
        let x = isize::try_from(self.x).unwrap() + x;
        let y = isize::try_from(self.y).unwrap() + y;

        self.x = usize::try_from(x).unwrap();
        self.y = usize::try_from(y).unwrap();
    }

    fn offset(&self, x: isize, y: isize) -> Self {
        let mut p = *self;
        p.offset_mut(x, y);
        p
    }

    fn dist(&self, rhs: &Self) -> usize {
        self.x.abs_diff(rhs.x) + self.y.abs_diff(rhs.y)
    }

    fn neighbors(&self) -> [Option<Pos>; 4] {
        [
            if self.x > 0 {
                Some(self.offset(-1, 0))
            } else {
                None
            },
            if self.y > 0 {
                Some(self.offset(0, -1))
            } else {
                None
            },
            Some(self.offset(1, 0)),
            Some(self.offset(0, 1)),
        ]
    }
}

impl From<&str> for Pos {
    fn from(value: &str) -> Self {
        let mut split = value.split(',');

        let x = split.next().unwrap().parse().unwrap();
        let y = split.next().unwrap().parse().unwrap();

        Self { x, y }
    }
}

impl Pos {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

struct Scan {
    depth: usize,
    target: Pos,
}

impl From<&str> for Scan {
    fn from(value: &str) -> Self {
        let mut iter = value.lines();

        let depth = iter.next().unwrap();
        let target = iter.next().unwrap();
        assert_eq!(iter.next(), None);

        assert!(depth.starts_with("depth: "));
        let depth = depth[7..].parse().unwrap();

        assert!(target.starts_with("target: "));
        let target = (&target[8..]).into();

        Self { depth, target }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum RegionType {
    Rocky,
    Wet,
    Narrow,
}

impl Display for RegionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            RegionType::Rocky => ".",
            RegionType::Wet => "=",
            RegionType::Narrow => "|",
        };
        write!(f, "{s}")
    }
}

impl RegionType {
    fn from_erosion_level(el: usize) -> Self {
        match el % 3 {
            0 => Self::Rocky,
            1 => Self::Wet,
            2 => Self::Narrow,
            _ => panic!("Impossible modulo result!"),
        }
    }

    fn risk_level(&self) -> usize {
        match self {
            RegionType::Rocky => 0,
            RegionType::Wet => 1,
            RegionType::Narrow => 2,
        }
    }
}

struct CaveSystem {
    target: Pos,
    explored_width: usize,
    regions: Vec<RegionType>,
}

impl Display for CaveSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut sep = "";
        for y in 0..=self.target.y {
            write!(f, "{sep}").unwrap();
            for x in 0..=self.target.x {
                match (x, y) {
                    (0, 0) => write!(f, "M").unwrap(),
                    (x, y) if x == self.target.x && y == self.target.y => write!(f, "T").unwrap(),
                    (x, y) => write!(f, "{}", self.regions[y * self.explored_width + x]).unwrap(),
                }
            }
            sep = "\n";
        }
        Ok(())
    }
}

impl CaveSystem {
    fn new(scan: Scan) -> Self {
        let (width, height) = (scan.target.x + 1, scan.target.y + 1);
        let (width, height) = (width * 2 - 1, height * 2 - 1);
        let dim = width.max(height) + 20;
        let mut geo_idx = vec![usize::MAX; dim * dim];
        let mut erosion_levels = vec![usize::MAX; dim * dim];
        let mut regions = vec![RegionType::Rocky; dim * dim];

        let mut pos = Pos::default();
        let mut loop_y = 0;
        while pos.y < dim {
            let i = pos.y * dim + pos.x;
            geo_idx[i] = match (pos.x, pos.y) {
                (0, 0) => 0,
                (x, y) if x == scan.target.x && y == scan.target.y => 0,
                (x, 0) => x * 16807,
                (0, y) => y * 48271,
                (x, y) => {
                    let i1 = y * dim + (x - 1);
                    let i2 = (y - 1) * dim + x;
                    erosion_levels[i1] * erosion_levels[i2]
                }
            };

            erosion_levels[i] = (geo_idx[i] + scan.depth) % 20183;
            regions[i] = RegionType::from_erosion_level(erosion_levels[i]);

            pos = if pos.y == 0 {
                loop_y += 1;
                Pos::new(0, loop_y)
            } else {
                pos.offset(1, -1)
            };
        }

        let target = scan.target;
        let explored_width = dim;
        Self {
            target,
            explored_width,
            regions,
        }
    }

    fn risk_level(&self) -> usize {
        let mut rl = 0;

        for y in 0..=self.target.y {
            for x in 0..=self.target.x {
                rl += self.regions[y * self.explored_width + x].risk_level();
            }
        }

        rl
    }

    fn fastest_route_to_target(&self) -> usize {
        let mut seen = HashMap::new();
        let mut nodes = BinaryHeap::new();
        let node = Node::new(self.target);
        seen.insert(SeenNode::from(&node), node.min_eta);
        nodes.push(Reverse(node));

        let mut best_time = usize::MAX;

        while let Some(Reverse(node)) = nodes.pop() {
            if node.min_eta >= best_time {
                break;
            }

            if node.pos == self.target {
                let mut node = node;
                let penalty = if node.gear == Gear::Torch { 0 } else { 7 };
                node.time += penalty;
                if node.time < best_time {
                    best_time = node.time;
                }
                break;
            }

            let i = node.pos.y * self.explored_width + node.pos.x;
            let rt = self.regions[i];

            for next_pos in node.pos.neighbors().into_iter().flatten() {
                let mut node = node.clone();
                let next_i = next_pos.y * self.explored_width + next_pos.x;
                let next_rt = self.regions[next_i];

                // can move without switching gear? go!
                if node.gear.can_enter(next_rt) {
                    node.step(next_pos, self.target);
                    enqueue_node(&mut nodes, &mut seen, node, best_time);
                    continue;
                }

                // else find the right gear and go.
                for gear in GEAR {
                    if gear.can_enter(rt) && gear.can_enter(next_rt) {
                        node.switch_gear(gear);
                        node.step(next_pos, self.target);
                        enqueue_node(&mut nodes, &mut seen, node, best_time);
                        break;
                    }
                }
            }
        }

        best_time
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct SeenNode {
    pos: Pos,
    gear: Gear,
}

impl From<&Node> for SeenNode {
    fn from(value: &Node) -> Self {
        Self {
            pos: value.pos,
            gear: value.gear,
        }
    }
}

fn enqueue_node(
    nodes: &mut BinaryHeap<Reverse<Node>>,
    seen: &mut HashMap<SeenNode, usize>,
    node: Node,
    best_time: usize,
) {
    if node.min_eta > best_time {
        return;
    }

    let key = SeenNode::from(&node);

    if let Some(seen_min_eta) = seen.get(&key) {
        if *seen_min_eta > node.min_eta {
            seen.insert(key, node.min_eta);
            nodes.push(Reverse(node));
        }
    } else {
        seen.insert(key, node.min_eta);
        nodes.push(Reverse(node));
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Gear {
    Torch,
    Climbing,
    Neither,
}

static GEAR: [Gear; 3] = [Gear::Torch, Gear::Climbing, Gear::Neither];

impl Gear {
    fn can_enter(&self, rt: RegionType) -> bool {
        match (rt, self) {
            (RegionType::Rocky, Gear::Torch) => true,
            (RegionType::Rocky, Gear::Climbing) => true,
            (RegionType::Rocky, Gear::Neither) => false,
            (RegionType::Wet, Gear::Torch) => false,
            (RegionType::Wet, Gear::Climbing) => true,
            (RegionType::Wet, Gear::Neither) => true,
            (RegionType::Narrow, Gear::Torch) => true,
            (RegionType::Narrow, Gear::Climbing) => false,
            (RegionType::Narrow, Gear::Neither) => true,
        }
    }
}

#[derive(Debug, Clone)]
struct Node {
    pos: Pos,
    gear: Gear,
    time: usize,
    min_eta: usize,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.min_eta == other.min_eta
    }
}

impl Eq for Node {}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.min_eta.cmp(&other.min_eta)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Node {
    fn min_eta(pos: Pos, target: Pos, time: usize, gear: Gear) -> usize {
        let dist = pos.dist(&target);

        time + dist + if gear != Gear::Torch { 7 } else { 0 }
    }

    fn new(target: Pos) -> Self {
        let pos = Pos::default();
        let gear = Gear::Torch;
        let time = 0;

        Self {
            pos,
            gear,
            time,
            min_eta: Self::min_eta(pos, target, time, gear),
        }
    }

    fn step(&mut self, new_pos: Pos, target: Pos) {
        self.pos = new_pos;
        self.time += 1;
        self.min_eta = Self::min_eta(self.pos, target, self.time, self.gear);
    }

    fn switch_gear(&mut self, new_gear: Gear) {
        self.gear = new_gear;
        self.time += 7;
    }
}

#[cfg(test)]
mod tests {
    use crate::{CaveSystem, Pos, Scan};

    #[test]
    fn test_input() {
        let scan = "\
            depth: 510\n\
            target: 10,10\
        ";
        let scan = Scan::from(scan);

        assert_eq!(scan.depth, 510);
        assert_eq!(scan.target, Pos::new(10, 10));

        let cave = CaveSystem::new(scan);

        let expected = "\
            M=.|=.|.|=.\n\
            .|=|=|||..|\n\
            .==|....||=\n\
            =.|....|.==\n\
            =|..==...=.\n\
            =||.=.=||=|\n\
            |.=.===|||.\n\
            |..==||=.|=\n\
            .=..===..=|\n\
            .======|||=\n\
            .===|=|===T\
        ";

        assert_eq!(cave.to_string(), expected);

        assert_eq!(cave.risk_level(), 114);
        assert_eq!(cave.fastest_route_to_target(), 45);
    }

    #[test]
    fn test_input_2() {
        let scan = std::fs::read_to_string("input/scan.txt").unwrap();
        let scan = Scan::from(scan.as_str());

        assert_eq!(scan.depth, 11109);
        assert_eq!(scan.target, Pos::new(9, 731));

        let cave = CaveSystem::new(scan);
        assert_eq!(cave.risk_level(), 7299);
        assert_eq!(cave.fastest_route_to_target(), 1008);
    }
}
