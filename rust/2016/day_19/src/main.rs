#![allow(dead_code)]

use std::collections::VecDeque;

fn main() {
    println!("Advent of Code 2016 - day 19");
}

fn who_gets_all_presents(num: usize) -> usize {
    let mut elves = VecDeque::with_capacity(num);
    (1..=num).for_each(|i| elves.push_back(i));

    while elves.len() > 1 {
        let elf_a = elves.pop_front().unwrap();
        let _ = elves.pop_front().unwrap();
        elves.push_back(elf_a);
    }

    elves.pop_front().unwrap()
}

fn who_gets_all_presents_part2(num: usize) -> usize {
    let mut left = VecDeque::with_capacity(num / 2);
    let mut right = VecDeque::with_capacity((num / 2) + 1);

    (1..=(num / 2)).for_each(|i| left.push_back(i));
    (((num / 2) + 1)..=num).for_each(|i| right.push_back(i));

    while !left.is_empty() {
        right.pop_front().unwrap(); // remove elf from start of right half
        right.push_back(left.pop_front().unwrap()); // move elf from start of left half to end of right half
        if right.len() - left.len() > 1 {
            left.push_back(right.pop_front().unwrap()); // rebalance when necessary
        }
    }

    right.pop_front().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::{who_gets_all_presents, who_gets_all_presents_part2};

    #[test]
    fn test_example() {
        assert_eq!(who_gets_all_presents(5), 3);
        assert_eq!(who_gets_all_presents_part2(5), 2);
    }

    #[test]
    fn test_input() {
        assert_eq!(who_gets_all_presents(3012210), 1830117);
        assert_eq!(who_gets_all_presents_part2(3012210), 1417887);
    }
}
