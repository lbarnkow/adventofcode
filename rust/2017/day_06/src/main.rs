#![allow(dead_code)]

use std::collections::HashSet;

fn main() {
    println!("Advent of Code 2017 - day 06");
}

fn parse_memory_banks(s: &str) -> Vec<usize> {
    s.split("\t")
        .map(|bank| bank.parse::<usize>().unwrap())
        .collect()
}

fn detect_reallocation_cycle(banks: &mut Vec<usize>) -> usize {
    let mut seen = HashSet::new();
    seen.insert(banks.clone());

    let mut cycles = 0;

    loop {
        let (bank, blocks) = banks.iter().enumerate().fold((0, 0), |acc, (idx, blocks)| {
            if *blocks > acc.1 {
                (idx, *blocks)
            } else {
                acc
            }
        });

        banks[bank] = 0;
        let mut idx = bank;
        for _ in 0..blocks {
            idx += 1;
            if idx == banks.len() {
                idx = 0;
            }
            banks[idx] += 1;
        }

        cycles += 1;
        let banks = banks.clone();
        if seen.contains(&banks) {
            break;
        }
        seen.insert(banks);
    }

    cycles
}

#[cfg(test)]
mod tests {
    use crate::{detect_reallocation_cycle, parse_memory_banks};

    #[test]
    fn test_examples() {
        let memory = "0\t2\t7\t0";
        let mut memory = parse_memory_banks(memory);

        assert_eq!(memory, vec![0, 2, 7, 0]);
        assert_eq!(detect_reallocation_cycle(&mut memory), 5);
        assert_eq!(detect_reallocation_cycle(&mut memory), 4);
    }

    #[test]
    fn test_input() {
        let memory = std::fs::read_to_string("input/memory.txt").unwrap();
        let mut memory = parse_memory_banks(&memory);

        assert_eq!(detect_reallocation_cycle(&mut memory), 6681);
        assert_eq!(detect_reallocation_cycle(&mut memory), 2392);
    }
}
