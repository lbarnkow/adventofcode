#![allow(dead_code)]
use std::cmp::Ordering;

fn main() {
    println!("Advent of Code 2015 - day 24");
}

fn parse_packages(s: &str) -> Vec<usize> {
    s.lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect()
}

fn find_smallest_groups_with_sum(packages: &[usize], target_sum: usize) -> Vec<Vec<usize>> {
    let mut result = Vec::new();

    let mut smallest_len = usize::MAX;

    let mut q = Vec::new();
    q.push((packages, Vec::new()));

    while let Some((packages, cur_group)) = q.pop() {
        let cur_sum: usize = cur_group.iter().sum();

        if cur_group.len() > smallest_len || cur_sum > target_sum {
            continue;
        }

        if packages.len() == 0 {
            if cur_sum == target_sum {
                if cur_group.len() < smallest_len {
                    result.clear();
                }
                smallest_len = cur_group.len();
                result.push(cur_group);
            }
            continue;
        }

        if cur_sum + packages[0] <= target_sum {
            let mut cur_group = cur_group.clone();
            cur_group.push(packages[0]);
            q.push((&packages[1..], cur_group));
        }

        q.push((&packages[1..], cur_group.clone()));
    }

    result
}

fn quantum_entanglement(p: &[usize]) -> usize {
    p.iter().fold(1, |acc, x| acc * (*x))
}

fn sort_by_quantum_entanglement(a: &Vec<usize>, b: &Vec<usize>) -> Ordering {
    quantum_entanglement(a).cmp(&quantum_entanglement(b))
}

fn find_ideal_qe_rec(packages: &[usize], group_sum: usize, n: usize) -> usize {
    if n == 1 {
        return usize::MAX;
    }

    let mut group_candidates = find_smallest_groups_with_sum(&packages, group_sum);
    group_candidates.sort_by(|a, b| sort_by_quantum_entanglement(a, b));

    for group in group_candidates {
        let packages: Vec<usize> = packages
            .iter()
            .map(|p| *p)
            .filter(|p| !group.contains(p))
            .collect();

        find_ideal_qe_rec(&packages, group_sum, n - 1);
        return quantum_entanglement(&group);
    }

    panic!("no viable configuration found!")
}

fn find_ideal_qe(packages: &Vec<usize>, n: usize) -> usize {
    let group_sum = packages.iter().map(|p| *p).sum::<usize>() / n;
    find_ideal_qe_rec(packages, group_sum, n)
}

#[cfg(test)]
mod tests {
    use crate::{find_ideal_qe, parse_packages};

    #[test]
    fn test_examples() {
        let packages = "\
            1\n\
            2\n\
            3\n\
            4\n\
            5\n\
            7\n\
            8\n\
            9\n\
            10\n\
            11\
        ";

        let mut packages = parse_packages(packages);
        packages.sort();
        packages.reverse();
        let sum: usize = packages.iter().sum();

        assert_eq!(packages.len(), 10);
        assert_eq!(sum, 60);

        let min_qe = find_ideal_qe(&packages, 3);
        assert_eq!(min_qe, 99);

        let min_qe = find_ideal_qe(&packages, 4);
        assert_eq!(min_qe, 44);
    }

    #[test]
    fn test_input() {
        let packages = std::fs::read_to_string("input/packages.txt").unwrap();
        let mut packages = parse_packages(&packages);
        packages.sort();
        packages.reverse();
        let sum: usize = packages.iter().sum();

        assert_eq!(packages.len(), 29);
        assert_eq!(sum, 1536);

        let min_qe = find_ideal_qe(&packages, 3);
        assert_eq!(min_qe, 10723906903);

        let min_qe = find_ideal_qe(&packages, 4);
        assert_eq!(min_qe, 74850409);
    }
}
