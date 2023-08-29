#![allow(dead_code)]

use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};

fn main() {
    println!("Advent of Code 2018 - day 07");
}

lazy_static! {
    static ref RE: Regex =
        Regex::new(r"^Step (\w) must be finished before step (\w) can begin.$").unwrap();
}

struct Instruction {
    prev: char,
    next: char,
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        let caps = RE.captures(value).unwrap();

        Self {
            prev: caps[1].chars().next().unwrap(),
            next: caps[2].chars().next().unwrap(),
        }
    }
}

fn parse_instructions(s: &str) -> HashMap<char, HashSet<char>> {
    s.lines()
        .map(|line| Instruction::from(line))
        .fold(HashMap::new(), |mut acc, i| {
            if let Some(next_steps) = acc.get_mut(&i.prev) {
                next_steps.insert(i.next);
            } else {
                acc.insert(i.prev, HashSet::from([i.next]));
            }
            acc
        })
}

fn extract_steps(instructions: &HashMap<char, HashSet<char>>) -> HashSet<char> {
    instructions
        .iter()
        .fold(HashSet::new(), |mut acc, (i, next_steps)| {
            acc.insert(*i);
            next_steps.iter().for_each(|i| {
                acc.insert(*i);
            });
            acc
        })
}

fn compute_start(
    steps: &HashSet<char>,
    instructions: &HashMap<char, HashSet<char>>,
) -> HashSet<char> {
    let follow_up_steps =
        instructions
            .values()
            .flatten()
            .map(|c| *c)
            .fold(HashSet::new(), |mut acc, c| {
                acc.insert(c);
                acc
            });
    steps
        .iter()
        .filter(|s| !follow_up_steps.contains(s))
        .map(|s| *s)
        .collect()
}

fn order_instructions(instructions: &HashMap<char, HashSet<char>>) -> String {
    let mut instructions = instructions.clone();
    let mut steps = extract_steps(&instructions);

    let mut steps_queue: Vec<char> = compute_start(&steps, &instructions).into_iter().collect();
    let mut order = String::with_capacity(steps.len());

    while !steps_queue.is_empty() {
        steps_queue.sort_by(|a, b| a.cmp(b).reverse());
        let cur_step = steps_queue.pop().unwrap();
        order.push(cur_step);
        instructions.remove(&cur_step);
        steps.remove(&cur_step);
        steps_queue = compute_start(&steps, &instructions).into_iter().collect();
    }

    order
}

fn time_execution(
    instructions: &HashMap<char, HashSet<char>>,
    workers: usize,
    base_step_duration: u64,
) -> u64 {
    let mut instructions = instructions.clone();
    let mut steps = extract_steps(&instructions);

    let mut t = 0;
    let mut workers = vec![(0, '.'); workers];

    loop {
        for w in workers.iter_mut().filter(|w| w.0 > 0) {
            w.0 -= 1;
            if w.0 == 0 {
                instructions.remove(&w.1);
                w.1 = '.';
            }
        }
        let mut workers_avail = workers.iter().filter(|w| w.0 == 0).count();
        if workers_avail == workers.len() && steps.is_empty() {
            break;
        }

        let mut steps_queue: Vec<char> = compute_start(&steps, &instructions).into_iter().collect();
        while workers_avail > 0 && !steps_queue.is_empty() {
            steps_queue.sort_by(|a, b| a.cmp(b).reverse());
            let cur_step = steps_queue.pop().unwrap();
            steps.remove(&cur_step);

            let mut w = workers.iter_mut().filter(|w| w.0 == 0).next().unwrap();
            w.0 = base_step_duration + (cur_step as u64) - ('A' as u64) + 1;
            w.1 = cur_step;
            workers_avail -= 1;
        }

        t += 1;
    }

    t
}

#[cfg(test)]
mod tests {
    use crate::{order_instructions, parse_instructions, time_execution};

    #[test]
    fn test_examples() {
        let instructions = "\
            Step C must be finished before step A can begin.\n\
            Step C must be finished before step F can begin.\n\
            Step A must be finished before step B can begin.\n\
            Step A must be finished before step D can begin.\n\
            Step B must be finished before step E can begin.\n\
            Step D must be finished before step E can begin.\n\
            Step F must be finished before step E can begin.\
        ";
        let instructions = parse_instructions(&instructions);

        let order = order_instructions(&instructions);
        assert_eq!(order, "CABDFE");

        let time = time_execution(&instructions, 2, 0);
        assert_eq!(time, 15);
    }

    #[test]
    fn test_input() {
        let instructions = std::fs::read_to_string("input/steps.txt").unwrap();
        let instructions = parse_instructions(&instructions);

        let order = order_instructions(&instructions);
        assert_eq!(order, "CHILFNMORYKGAQXUVBZPSJWDET");

        let time = time_execution(&instructions, 5, 60);
        assert_eq!(time, 891);
    }
}
