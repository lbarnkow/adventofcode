#![allow(dead_code)]

use std::collections::HashSet;

fn main() {
    println!("Advent of Code 2018 - day 01");
}

fn parse_changes(s: &str) -> Vec<i64> {
    s.lines().map(|line| line.parse().unwrap()).collect()
}

fn calibrate_frequencies(start: i64, changes: &[i64]) -> i64 {
    changes.iter().fold(start, |acc, c| acc + *c)
}

fn calibrate_frequencies_2(start: i64, changes: &[i64]) -> i64 {
    let mut freq = start;

    let mut seen = HashSet::with_capacity(changes.len());
    seen.insert(freq);

    loop {
        for change in changes {
            freq += *change;

            if seen.contains(&freq) {
                return freq;
            }
            seen.insert(freq);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{calibrate_frequencies, calibrate_frequencies_2, parse_changes};

    #[test]
    fn test_examples() {
        let changes = "\
            +1\n\
            -2\n\
            +3\n\
            +1\
        ";
        let changes = parse_changes(changes);
        assert_eq!(calibrate_frequencies(0, &changes), 3);

        let changes = "\
            +1\n\
            +1\n\
            +1\
        ";
        let changes = parse_changes(changes);
        assert_eq!(calibrate_frequencies(0, &changes), 3);

        let changes = "\
            +1\n\
            +1\n\
            -2\
        ";
        let changes = parse_changes(changes);
        assert_eq!(calibrate_frequencies(0, &changes), 0);

        let changes = "\
            -1\n\
            -2\n\
            -3\
        ";
        let changes = parse_changes(changes);
        assert_eq!(calibrate_frequencies(0, &changes), -6);
    }

    #[test]
    fn test_examples_part2() {
        let changes = "\
            +1\n\
            -2\n\
            +3\n\
            +1\
        ";
        let changes = parse_changes(changes);
        assert_eq!(calibrate_frequencies(0, &changes), 3);
        assert_eq!(calibrate_frequencies_2(0, &changes), 2);

        let changes = "\
            +1\n\
            -1\
        ";
        let changes = parse_changes(changes);
        assert_eq!(calibrate_frequencies(0, &changes), 0);
        assert_eq!(calibrate_frequencies_2(0, &changes), 0);

        let changes = "\
            +3\n\
            +3\n\
            +4\n\
            -2\n\
            -4\
        ";
        let changes = parse_changes(changes);
        assert_eq!(calibrate_frequencies(0, &changes), 4);
        assert_eq!(calibrate_frequencies_2(0, &changes), 10);

        let changes = "\
            -6\n\
            +3\n\
            +8\n\
            +5\n\
            -6\
        ";
        let changes = parse_changes(changes);
        assert_eq!(calibrate_frequencies(0, &changes), 4);
        assert_eq!(calibrate_frequencies_2(0, &changes), 5);

        let changes = "\
            +7\n\
            +7\n\
            -2\n\
            -7\n\
            -4\
        ";
        let changes = parse_changes(changes);
        assert_eq!(calibrate_frequencies(0, &changes), 1);
        assert_eq!(calibrate_frequencies_2(0, &changes), 14);
    }

    #[test]
    fn test_input() {
        let changes = std::fs::read_to_string("input/changes.txt").unwrap();
        let changes = parse_changes(changes.as_str());
        assert_eq!(calibrate_frequencies(0, &changes), 437);
        assert_eq!(calibrate_frequencies_2(0, &changes), 655);
    }
}
