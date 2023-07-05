#![allow(dead_code)]
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};

lazy_static! {
    static ref REGEX: Regex = Regex::new(r"^(\w+) to (\w+) = (\d+)$").unwrap();
}

fn main() {
    println!("Advent of Code 2015 - day 9");
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Pair {
    a: String,
    b: String,
}

fn parse_dist_lines(dist: &str) -> (Vec<String>, HashMap<Pair, u64>) {
    let mut locations = HashSet::new();
    let mut dist_table = HashMap::new();

    for line in dist.lines() {
        let caps = REGEX.captures(line).unwrap();
        locations.insert(caps[1].to_owned());
        locations.insert(caps[2].to_owned());
        dist_table.insert(
            Pair {
                a: caps[1].to_owned(),
                b: caps[2].to_owned(),
            },
            caps[3].parse::<u64>().unwrap(),
        );
        dist_table.insert(
            Pair {
                a: caps[2].to_owned(),
                b: caps[1].to_owned(),
            },
            caps[3].parse::<u64>().unwrap(),
        );
    }

    (locations.into_iter().collect(), dist_table)
}

fn find_route_distance<F>(
    start: String,
    mut locations: Vec<String>,
    dist_table: &HashMap<Pair, u64>,
    func: &F,
    default: u64,
) -> u64
where
    F: Fn(u64, u64) -> u64,
{
    if locations.len() == 1 {
        let pair = Pair {
            a: start,
            b: locations.pop().unwrap(),
        };
        *dist_table.get(&pair).unwrap()
    } else {
        let mut best = default;

        for idx in 0..locations.len() {
            let mut locations = locations.clone();
            let dest = locations.remove(idx);

            let pair = Pair {
                a: start.clone(),
                b: dest.clone(),
            };
            best = func(
                find_route_distance(dest, locations, dist_table, func, default)
                    + *dist_table.get(&pair).unwrap(),
                best,
            );
        }

        best
    }
}

fn calculate_route<F>(dist: &str, func: &F, default: u64) -> u64
where
    F: Fn(u64, u64) -> u64,
{
    let (locations, dist_table) = parse_dist_lines(dist);

    let mut best = default;
    for idx in 0..locations.len() {
        let mut locations = locations.clone();
        let start = locations.remove(idx);

        best = func(
            best,
            find_route_distance(start, locations, &dist_table, func, default),
        );
    }

    best
}

#[cfg(test)]
mod tests {
    use crate::calculate_route;

    #[test]
    fn test_examples() {
        let dist = "London to Dublin = 464\n\
            London to Belfast = 518\n\
            Dublin to Belfast = 141";

        assert_eq!(calculate_route(dist, &std::cmp::min, u64::MAX), 605);
        assert_eq!(calculate_route(dist, &std::cmp::max, u64::MIN), 982);
    }

    #[test]
    fn test_input() {
        let dist = std::fs::read_to_string("input/distances.txt").unwrap();
        assert_eq!(calculate_route(&dist, &std::cmp::min, u64::MAX), 207);
        assert_eq!(calculate_route(&dist, &std::cmp::max, u64::MIN), 804);
    }
}
