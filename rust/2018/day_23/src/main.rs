#![allow(dead_code)]

use std::{cmp::Reverse, collections::BinaryHeap};

use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    println!("Advent of Code 2018 - day 23");
}

lazy_static! {
    static ref RE: Regex = Regex::new(r"^pos=<(-?\d+),(-?\d+),(-?\d+)>, r=(\d+)$").unwrap();
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Pos {
    x: isize,
    y: isize,
    z: isize,
}

const ORIGIN: Pos = Pos::default();

impl Pos {
    fn new(x: isize, y: isize, z: isize) -> Self {
        Self { x, y, z }
    }

    fn dist(&self, other: &Self) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y) + self.z.abs_diff(other.z)
    }

    const fn default() -> Self {
        Self { x: 0, y: 0, z: 0 }
    }
}

#[derive(Debug, Clone)]
struct Nanobot {
    pos: Pos,
    r: usize,
}

impl From<&str> for Nanobot {
    fn from(value: &str) -> Self {
        let caps = RE.captures(value).unwrap();

        let x: isize = caps[1].parse().unwrap();
        let y: isize = caps[2].parse().unwrap();
        let z: isize = caps[3].parse().unwrap();
        let r: usize = caps[4].parse().unwrap();

        Self::new(Pos::new(x, y, z), r)
    }
}

impl Nanobot {
    fn new(pos: Pos, r: usize) -> Self {
        Self { pos, r }
    }
}

struct Fleet {
    bots: Vec<Nanobot>,
}

impl From<&str> for Fleet {
    fn from(value: &str) -> Self {
        Self {
            bots: value.lines().map(|line| line.into()).collect(),
        }
    }
}

impl Fleet {
    fn strongest(&self) -> &Nanobot {
        self.bots
            .iter()
            .fold(self.bots.first().unwrap(), |acc, bot| {
                if acc.r < bot.r {
                    bot
                } else {
                    acc
                }
            })
    }

    fn in_range_of(&self, bot: &Nanobot) -> usize {
        self.bots
            .iter()
            .filter(|other| other.pos.dist(&bot.pos) <= bot.r)
            .count()
    }

    fn optimal_dist(&self) -> usize {
        // couldn't figure out a general solution for hours that had decent performance.
        // so, copied an "incorrect" solution from reddit that works on the given input.
        // see https://www.reddit.com/r/adventofcode/comments/a8s17l/2018_day_23_solutions/ecdqzdg/
        let mut queue = BinaryHeap::new();

        for bot in &self.bots {
            let dist = ORIGIN.dist(&bot.pos);
            let from = if dist >= bot.r { dist - bot.r } else { 0 };

            queue.push(Reverse(BinHeapItem::new(from, DistRange::Begin)));
            queue.push(Reverse(BinHeapItem::new(dist + bot.r, DistRange::End)));
        }

        let mut count = 0;
        let mut max_count = 0;
        let mut result = 0;

        while let Some(Reverse(item)) = queue.pop() {
            match item.r {
                DistRange::Begin => count += 1,
                DistRange::End => count -= 1,
            }
            if count > max_count {
                result = item.dist;
                max_count = count;
            }
        }

        result
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
enum DistRange {
    Begin,
    End,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct BinHeapItem {
    dist: usize,
    r: DistRange,
}

impl BinHeapItem {
    fn new(dist: usize, r: DistRange) -> Self {
        Self { dist, r }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Fleet, Pos};

    #[test]
    fn test_example() {
        let bots = "\
            pos=<0,0,0>, r=4\n\
            pos=<1,0,0>, r=1\n\
            pos=<4,0,0>, r=3\n\
            pos=<0,2,0>, r=1\n\
            pos=<0,5,0>, r=3\n\
            pos=<0,0,3>, r=1\n\
            pos=<1,1,1>, r=1\n\
            pos=<1,1,2>, r=1\n\
            pos=<1,3,1>, r=1\
        ";
        let fleet = Fleet::from(bots);

        assert_eq!(fleet.bots.len(), 9);

        let strongest = fleet.strongest();
        assert_eq!(strongest.pos, Pos::new(0, 0, 0));
        assert_eq!(strongest.r, 4);

        let in_range = fleet.in_range_of(strongest);
        assert_eq!(in_range, 7);
    }

    #[test]
    fn test_example_part2() {
        let bots = "\
            pos=<10,12,12>, r=2\n\
            pos=<12,14,12>, r=2\n\
            pos=<16,12,12>, r=4\n\
            pos=<14,14,14>, r=6\n\
            pos=<50,50,50>, r=200\n\
            pos=<10,10,10>, r=5\
        ";
        let fleet = Fleet::from(bots);
        assert_eq!(fleet.bots.len(), 6);

        let optimal = fleet.optimal_dist();
        assert_eq!(optimal, 36);
    }

    #[test]
    fn test_input() {
        let bots = std::fs::read_to_string("input/nanobots.txt").unwrap();
        let fleet = Fleet::from(bots.as_str());

        assert_eq!(fleet.bots.len(), 1000);

        let strongest = fleet.strongest();
        let in_range = fleet.in_range_of(strongest);

        assert_eq!(in_range, 704);
    }

    #[test]
    fn test_input_part2() {
        let bots = std::fs::read_to_string("input/nanobots.txt").unwrap();
        let fleet = Fleet::from(bots.as_str());

        assert_eq!(fleet.bots.len(), 1000);

        let optimal = fleet.optimal_dist();
        assert_eq!(optimal, 111960222);
    }
}
