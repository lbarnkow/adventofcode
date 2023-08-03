#![allow(dead_code)]

use std::collections::VecDeque;

fn main() {
    println!("Advent of Code 2016 - day 20");
}

fn parse_rule(rule: &str) -> (u32, u32) {
    let mut iter = rule.split('-').map(|ip| ip.parse::<u32>().unwrap());
    (iter.next().unwrap(), iter.next().unwrap())
}

fn lowest_valued_allowed_ip(rules: &str) -> u32 {
    let mut rules: Vec<(u32, u32)> = rules.lines().map(|rule| parse_rule(rule)).collect();
    rules.sort_by(|(a_from, _), (b_from, _)| a_from.cmp(b_from));

    let mut min_allowed = 0;

    for rule in rules {
        if rule.0 <= min_allowed && rule.1 >= min_allowed {
            min_allowed = rule.1 + 1;
        } else if rule.0 > min_allowed {
            break;
        }
    }

    min_allowed
}

fn allowed_ips(rules: &str) -> Vec<u32> {
    let mut rules: Vec<(u32, u32)> = rules.lines().map(|rule| parse_rule(rule)).collect();
    rules.sort_by(|(a_from, _), (b_from, _)| a_from.cmp(b_from));
    let mut rules = VecDeque::from(rules);

    let mut ip = 0;
    let mut allowed = Vec::new();
    while let Some(rule) = rules.pop_front() {
        while ip < rule.0 {
            allowed.push(ip);
            ip += 1;
        }
        if rule.1 == u32::MAX {
            return allowed;
        }
        ip = ip.max(rule.1 + 1);
    }

    allowed.append(&mut (ip..=u32::MAX).collect());
    allowed
}

#[cfg(test)]
mod tests {
    use crate::{allowed_ips, lowest_valued_allowed_ip};

    #[test]
    fn test_example() {
        let rules = "\
            5-8\n\
            0-2\n\
            4-7\
        ";

        assert_eq!(lowest_valued_allowed_ip(rules), 3);
    }

    #[test]
    fn test_input() {
        let rules = std::fs::read_to_string("input/rules.txt").unwrap();

        assert_eq!(lowest_valued_allowed_ip(&rules), 31053880);
        assert_eq!(allowed_ips(&rules).len(), 117);
    }
}
