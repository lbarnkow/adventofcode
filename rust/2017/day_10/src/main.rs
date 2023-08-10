#![allow(dead_code)]

use std::collections::VecDeque;

fn main() {
    println!("Advent of Code 2017 - day 10");
}

static SUFFIX: [usize; 5] = [17, 31, 73, 47, 23];

fn reverse(list: &mut VecDeque<usize>, n: usize) {
    if n < 2 {
        return;
    }

    for i in 0..n / 2 {
        let tmp = list[i];
        list[i] = list[n - 1 - i];
        list[n - 1 - i] = tmp;
    }
}

fn parse_lengths(s: &str) -> Vec<usize> {
    s.split(",").map(|e| e.parse::<usize>().unwrap()).collect()
}

fn parse_lengths_ascii(s: &str) -> Vec<usize> {
    s.chars()
        .map(|c| c as u32)
        .map(|unicode| u8::try_from(unicode).unwrap())
        .map(|ascii| ascii as usize)
        .chain(SUFFIX)
        .collect()
}

fn sparse_knot_hash(
    (lower_bound, upper_bound): (usize, usize),
    lengths: &[usize],
    rounds: usize,
) -> Vec<usize> {
    let mut knot: VecDeque<usize> = (lower_bound..=upper_bound).collect();
    let mut shifts = 0;
    let mut skip_size = 0;

    for _ in 0..rounds {
        for length in lengths {
            reverse(&mut knot, *length);

            let rotate = (length + skip_size) % knot.len();
            knot.rotate_left(rotate);
            shifts += rotate;
            skip_size += 1;
        }
    }

    shifts = shifts % knot.len();
    knot.rotate_right(shifts);

    knot.into()
}

fn knot_hash(input: &str) -> String {
    let lengths = parse_lengths_ascii(input);
    let sparse_hash = sparse_knot_hash((0, 255), &lengths, 64);
    let mut dense_hash = Vec::with_capacity(16);

    for i in 0..16 {
        let i = i * 16;
        let block = (&sparse_hash[i + 1..i + 16])
            .iter()
            .fold(sparse_hash[i], |acc, e| acc ^ *e);
        dense_hash.push(block);
    }

    dense_hash
        .iter()
        .map(|block| format!("{:0>2x}", block))
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::{knot_hash, parse_lengths, sparse_knot_hash};

    #[test]
    fn test_examples() {
        let lengths = parse_lengths("5,0,1");
        let hash = sparse_knot_hash((0, 5), &lengths, 1);
        assert_eq!(hash, [4, 3, 2, 1, 0, 5]);
        assert_eq!(hash[0] * hash[1], 12);

        let lenghts = parse_lengths("3,4,1,5");
        let hash = sparse_knot_hash((0, 4), &lenghts, 1);
        assert_eq!(hash, [3, 4, 2, 1, 0]);
        assert_eq!(hash[0] * hash[1], 12);

        let hash = knot_hash("");
        assert_eq!(hash, "a2582a3a0e66e6e86e3812dcb672a272");
        let hash = knot_hash("AoC 2017");
        assert_eq!(hash, "33efeb34ea91902bb2f59c9920caa6cd");
        let hash = knot_hash("1,2,3");
        assert_eq!(hash, "3efbe78a8d82f29979031a4aa0b16a9d");
        let hash = knot_hash("1,2,4");
        assert_eq!(hash, "63960835bcdc130f0b66d7ff4f6a5a8e");
    }

    #[test]
    fn test_input() {
        let input = std::fs::read_to_string("input/lengths.txt").unwrap();
        let lengths = parse_lengths(&input);
        let hash = sparse_knot_hash((0, 255), &lengths, 1);
        assert_eq!(hash[0] * hash[1], 52070);

        let hash = knot_hash(&input);
        assert_eq!(hash, "7f94112db4e32e19cf6502073c66f9bb");
    }
}
