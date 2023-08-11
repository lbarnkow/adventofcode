#![allow(dead_code)]

use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    println!("Advent of Code 2017 - day 12");
}

fn build_pipes_map(pipes: &str) -> HashMap<usize, HashSet<usize>> {
    let mut connections: HashMap<usize, HashSet<usize>> = HashMap::new();

    for line in pipes.lines() {
        let mut split = line.split(" <-> ");
        let lhs = split.next().unwrap().parse::<usize>().unwrap();
        let rhs: Vec<usize> = split
            .next()
            .unwrap()
            .split(", ")
            .map(|p| p.parse::<usize>().unwrap())
            .collect();
        let mut lhs_list = HashSet::new();

        for partner in rhs {
            if let Some(partner_list) = connections.get_mut(&partner) {
                partner_list.insert(lhs);
            } else {
                connections.insert(partner, [lhs].into_iter().collect::<HashSet<usize>>());
            }
            lhs_list.insert(partner);
        }
        connections.insert(lhs, lhs_list);
    }

    connections
}

fn compute_group(pipes: &HashMap<usize, HashSet<usize>>, group_leader: usize) -> HashSet<usize> {
    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();
    queue.push_back(group_leader);
    seen.insert(group_leader);

    while let Some(program) = queue.pop_front() {
        for partner in pipes.get(&program).unwrap() {
            if !seen.contains(partner) {
                queue.push_back(*partner);
                seen.insert(*partner);
            }
        }
    }

    seen
}

fn count_disjoint_groups(mut pipes: HashMap<usize, HashSet<usize>>) -> usize {
    let mut groups = 0;

    while let Some(group_leader) = pipes.keys().next() {
        let group = compute_group(&pipes, *group_leader);
        for member in group {
            pipes.remove(&member);
        }
        groups += 1;
    }

    groups
}

#[cfg(test)]
mod tests {
    use crate::{build_pipes_map, compute_group, count_disjoint_groups};

    #[test]
    fn test_examples() {
        let pipes = "\
            0 <-> 2\n\
            1 <-> 1\n\
            2 <-> 0, 3, 4\n\
            3 <-> 2, 4\n\
            4 <-> 2, 3, 6\n\
            5 <-> 6\n\
            6 <-> 4, 5\
        ";
        let pipes = build_pipes_map(pipes);

        assert_eq!(compute_group(&pipes, 0).len(), 6);
        assert_eq!(compute_group(&pipes, 1).len(), 1);
        assert_eq!(count_disjoint_groups(pipes), 2);
    }

    #[test]
    fn test_input() {
        let pipes = std::fs::read_to_string("input/pipes.txt").unwrap();
        let pipes = build_pipes_map(&pipes);

        assert_eq!(compute_group(&pipes, 0).len(), 130);
        assert_eq!(count_disjoint_groups(pipes), 189);
    }
}
