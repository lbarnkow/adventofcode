#![allow(dead_code)]

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet, VecDeque},
    fmt::Display,
    hash::Hash,
};

use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    println!("Advent of Code 2016 - day 22");
}

lazy_static! {
    static ref RE_DF: Regex =
        Regex::new(r"^/dev/grid/node-x(\d+)-y(\d+)\s+(\d+)T\s+(\d+)T\s+(\d+)T\s+(\d+)%$").unwrap();
}

struct DfInfo {
    node: Pos,
    size: usize,
    used: usize,
}

impl DfInfo {
    fn from(value: &str) -> Option<Self> {
        if let Some(caps) = RE_DF.captures(value) {
            let node = Pos::new(caps[1].parse().unwrap(), caps[2].parse().unwrap());
            let size = caps[3].parse().unwrap();
            let used = caps[4].parse().unwrap();
            let avail = caps[5].parse().unwrap();
            let use_p = caps[6].parse().unwrap();

            assert_eq!(size - used, avail);
            assert_eq!(used * 100 / size, use_p);

            Some(Self { node, size, used })
        } else {
            None
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
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

        (max_y - min_y) + (max_x - min_x)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Data {
    Generic(usize),
    Target(usize),
}

impl Data {
    fn is_generic(&self) -> bool {
        match self {
            Data::Generic(_) => true,
            _ => false,
        }
    }

    fn is_target(&self) -> bool {
        match self {
            Data::Target(_) => true,
            _ => false,
        }
    }

    fn size(&self) -> usize {
        match self {
            Data::Generic(s) => *s,
            Data::Target(s) => *s,
        }
    }
}

impl std::ops::Add<Data> for Data {
    type Output = Self;

    fn add(self, rhs: Data) -> Self::Output {
        match (self, rhs) {
            (Data::Generic(a), Data::Generic(b)) => Self::Generic(a + b),
            (Data::Generic(a), Data::Target(b)) => Self::Target(a + b),
            (Data::Target(a), Data::Generic(b)) => Self::Target(a + b),
            (Data::Target(a), Data::Target(b)) => Self::Target(a + b),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Node {
    size: usize,
    data: Data,
}

impl Node {
    fn new(size: usize, data: Data) -> Self {
        Self { size, data }
    }

    fn used(&self) -> usize {
        self.data.size()
    }

    fn avail(&self) -> usize {
        self.size - self.data.size()
    }

    fn is_empty(&self) -> bool {
        self.used() == 0
    }

    fn has_target(&self) -> bool {
        self.data.is_target()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum SimplifiedNode {
    Empty,
    Target,
    Small,
    Huge,
}

#[derive(Debug, Clone, Eq)]
struct Cluster {
    width: usize,
    height: usize,
    nodes: Vec<Node>,
    simplified: VecDeque<SimplifiedNode>,
}

impl PartialEq for Cluster {
    fn eq(&self, other: &Self) -> bool {
        self.width == other.width
            && self.height == other.height
            && self.simplified == other.simplified
    }
}

impl Hash for Cluster {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.width.hash(state);
        self.height.hash(state);
        self.simplified.hash(state);
    }
}

impl Cluster {
    fn new(df_output: &str, target: Pos) -> Self {
        let mut info: Vec<DfInfo> = df_output
            .lines()
            .filter_map(|line| DfInfo::from(line))
            .collect();

        info.sort_by(|a, b| {
            let cmp = a.node.y.cmp(&b.node.y);
            if cmp.is_eq() {
                a.node.x.cmp(&b.node.x)
            } else {
                cmp
            }
        });

        let max_grid_pos = info.last().unwrap().node;
        let (width, height) = (max_grid_pos.x + 1, max_grid_pos.y + 1);
        assert_eq!(info.len(), width * height);
        let mut nodes = Vec::with_capacity(width * height);

        info.iter().for_each(|df| {
            let data = if target == df.node {
                Data::Target(df.used)
            } else {
                Data::Generic(df.used)
            };

            nodes.push(Node::new(df.size, data));
        });

        let mut cluster = Self {
            width,
            height,
            nodes,
            simplified: VecDeque::with_capacity(width * height),
        };

        cluster.calc_simplified();
        cluster
    }

    fn calc_simplified(&mut self) {
        let max_size = self.nodes.iter().map(|n| n.size).max().unwrap();

        self.simplified.clear();
        self.nodes.iter().for_each(|n| {
            let simple_entry = if n.has_target() {
                SimplifiedNode::Target
            } else if n.is_empty() {
                SimplifiedNode::Empty
            } else if n.size < max_size {
                SimplifiedNode::Small
            } else {
                SimplifiedNode::Huge
            };
            self.simplified.push_back(simple_entry);
        });
    }

    fn pos_to_idx(&self, pos: Pos) -> usize {
        pos.y * self.width + pos.x
    }

    fn idx_to_pos(&self, idx: usize) -> Pos {
        Pos {
            x: idx % self.width,
            y: idx / self.width,
        }
    }

    fn viable_pairs(&self) -> Vec<(Pos, Pos)> {
        let mut pairs = Vec::new();
        for (a_idx, a) in self.nodes.iter().enumerate() {
            for (b_idx, b) in self.nodes.iter().enumerate() {
                if !a.is_empty() && a_idx != b_idx && a.used() <= b.avail() {
                    pairs.push((self.idx_to_pos(a_idx), self.idx_to_pos(b_idx)));
                }
            }
        }
        pairs
    }

    fn neighbors(&self, pos: Pos) -> Vec<Pos> {
        let mut neighbors = Vec::with_capacity(4);

        if pos.x > 0 {
            neighbors.push(Pos::new(pos.x - 1, pos.y));
        }
        if pos.y > 0 {
            neighbors.push(Pos::new(pos.x, pos.y - 1));
        }
        if pos.x < (self.width - 1) {
            neighbors.push(Pos::new(pos.x + 1, pos.y));
        }
        if pos.y < (self.height - 1) {
            neighbors.push(Pos::new(pos.x, pos.y + 1))
        }

        neighbors
    }

    fn try_move(&self, from_pos: Pos, to_pos: Pos) -> Option<Self> {
        let from_idx = self.pos_to_idx(from_pos);
        let to_idx = self.pos_to_idx(to_pos);
        let from = self.nodes[from_idx];
        let to = self.nodes[to_idx];

        if from.used() == 0 || to.avail() < from.used() {
            return None;
        }

        if to.has_target() {
            return None;
        }
        if from.has_target() && !to.is_empty() {
            return None;
        }

        let mut successor = self.clone();
        successor.nodes[to_idx].data =
            successor.nodes[from_idx].data + successor.nodes[to_idx].data;
        successor.nodes[from_idx].data = Data::Generic(0);
        successor.calc_simplified();
        Some(successor)
    }

    fn fewest_moves_to_access_target(&self, entry_point: Pos) -> usize {
        let mut heap = BinaryHeap::new();
        let mut seen = HashSet::new();

        heap.push(Reverse(BinHeapItem::new(0, self.clone(), entry_point)));
        seen.insert(self.clone());

        let entry_idx = self.pos_to_idx(entry_point);

        while let Some(item) = heap.pop() {
            let steps = item.0.steps;
            let state = item.0.state;

            for pos in (0..state.nodes.len()).map(|idx| self.idx_to_pos(idx)) {
                for neighbor in self.neighbors(pos) {
                    if let Some(next_state) = state.try_move(pos, neighbor) {
                        if next_state.nodes[entry_idx].has_target() {
                            return steps + 1;
                        }
                        if !seen.contains(&next_state) {
                            heap.push(Reverse(BinHeapItem::new(
                                steps + 1,
                                next_state.clone(),
                                entry_point,
                            )));
                            seen.insert(next_state);
                        }
                    }
                }
            }
        }

        panic!("No solution found!");
    }
}

struct BinHeapItem {
    steps: usize,
    state: Cluster,
    score: usize,
}

impl BinHeapItem {
    fn new(steps: usize, state: Cluster, entry_point: Pos) -> Self {
        let score = Self::score(&state, entry_point);

        Self {
            steps,
            state,
            score,
        }
    }

    fn score(state: &Cluster, entry_point: Pos) -> usize {
        let target_pos = state
            .nodes
            .iter()
            .enumerate()
            .filter(|(_, n)| n.has_target())
            .map(|(idx, _)| state.idx_to_pos(idx))
            .next()
            .unwrap();
        let target_dist = entry_point.dist(&target_pos);

        let mut min_empty_dist = usize::MAX;
        state
            .nodes
            .iter()
            .enumerate()
            .filter(|(_, n)| n.is_empty())
            .for_each(|(idx, _)| {
                let empty_pos = state.idx_to_pos(idx);
                min_empty_dist = min_empty_dist.min(target_pos.dist(&empty_pos))
            });

        min_empty_dist + target_dist
    }
}

impl PartialEq for BinHeapItem {
    fn eq(&self, other: &Self) -> bool {
        self.steps == other.steps && self.score == other.score
    }
}

impl Eq for BinHeapItem {}

impl PartialOrd for BinHeapItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BinHeapItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let cmp = self.score.cmp(&other.score);
        if cmp.is_eq() {
            self.steps.cmp(&other.steps)
        } else {
            cmp
        }
    }
}

impl Display for Cluster {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = y * self.width + x;
                match self.nodes[idx].data {
                    Data::Generic(s) => {
                        if s == 0 {
                            write!(f, " _ ").unwrap()
                        } else if s > 250 {
                            write!(f, " # ").unwrap()
                        } else {
                            write!(f, " . ").unwrap()
                        }
                    }
                    Data::Target(_) => write!(f, " G ").unwrap(),
                }
            }
            write!(f, "\n").unwrap();
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{Cluster, Pos};

    #[test]
    fn test_example() {
        let df_output = "\
            root@ebhq-gridcenter# df -h\n\
            Filesystem            Size  Used  Avail  Use%\n\
            /dev/grid/node-x0-y0   10T    8T     2T   80%\n\
            /dev/grid/node-x0-y1   11T    6T     5T   54%\n\
            /dev/grid/node-x0-y2   32T   28T     4T   87%\n\
            /dev/grid/node-x1-y0    9T    7T     2T   77%\n\
            /dev/grid/node-x1-y1    8T    0T     8T    0%\n\
            /dev/grid/node-x1-y2   11T    7T     4T   63%\n\
            /dev/grid/node-x2-y0   10T    6T     4T   60%\n\
            /dev/grid/node-x2-y1    9T    8T     1T   88%\n\
            /dev/grid/node-x2-y2    9T    6T     3T   66%\
        ";

        let c = Cluster::new(df_output, Pos::new(2, 0));
        assert_eq!(c.viable_pairs().len(), 7);

        assert_eq!(c.fewest_moves_to_access_target(Pos::new(0, 0)), 7);
    }

    #[test]
    fn test_input() {
        let df = std::fs::read_to_string("input/df.txt").unwrap();

        let c = Cluster::new(df.as_str(), Pos::new(34, 0));
        assert_eq!(c.viable_pairs().len(), 1003);

        assert_eq!(c.fewest_moves_to_access_target(Pos::new(0, 0)), 192);
    }
}
