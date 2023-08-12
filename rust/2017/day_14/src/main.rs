#![allow(dead_code)]

use std::collections::{HashSet, VecDeque};

fn main() {
    println!("Advent of Code 2017 - day 14");
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

fn hash_to_bitmap(hash: &str) -> String {
    if hash.len() != 32 {
        panic!("Illegal hash length!");
    }

    hash.chars()
        .map(|c| match c {
            '0' => "0000",
            '1' => "0001",
            '2' => "0010",
            '3' => "0011",
            '4' => "0100",
            '5' => "0101",
            '6' => "0110",
            '7' => "0111",
            '8' => "1000",
            '9' => "1001",
            'a' => "1010",
            'b' => "1011",
            'c' => "1100",
            'd' => "1101",
            'e' => "1110",
            'f' => "1111",
            _ => panic!("Illegal hash char: {c}!"),
        })
        .collect()
}

fn defrag_map(input: &str) -> Vec<String> {
    (0..128)
        .map(|i| format!("{input}-{i}"))
        .map(|row_input| knot_hash(&row_input))
        .map(|hash| hash_to_bitmap(&hash))
        .map(|row_bits| {
            row_bits
                .chars()
                .map(|c| if c == '0' { '.' } else { '#' })
                .collect::<String>()
        })
        .collect()
}

fn num_used_squares(defrag_map: &Vec<String>) -> usize {
    defrag_map
        .iter()
        .flat_map(|row| row.chars().filter(|c| *c == '#'))
        .count()
}

fn neighbor_idxs(idx: usize) -> [Option<usize>; 4] {
    let (x, y) = (idx % 128, idx / 128);

    [
        if x > 0 { Some(y * 128 + (x - 1)) } else { None },
        if y > 0 { Some((y - 1) * 128 + x) } else { None },
        if x < (128 - 1) {
            Some(y * 128 + (x + 1))
        } else {
            None
        },
        if y < (128 - 1) {
            Some((y + 1) * 128 + x)
        } else {
            None
        },
    ]
}

fn clear_region(flat_map: &mut Vec<char>, start_idx: usize) {
    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();

    queue.push_back(start_idx);
    seen.insert(start_idx);

    while let Some(idx) = queue.pop_front() {
        flat_map[idx] = '.';

        for neighbor in neighbor_idxs(idx) {
            if let Some(n_idx) = neighbor {
                if flat_map[n_idx] == '#' && !seen.contains(&n_idx) {
                    queue.push_back(n_idx);
                    seen.insert(n_idx);
                }
            }
        }
    }
}

fn num_regions(defrag_map: &Vec<String>) -> usize {
    let mut flat_map: Vec<char> = defrag_map
        .iter()
        .flat_map(|row| row.chars().collect::<Vec<char>>())
        .collect();

    let mut regions = 0;

    for i in 0..(128 * 128) {
        if flat_map[i] == '#' {
            regions += 1;
            clear_region(&mut flat_map, i);
        }
    }

    regions
}

#[cfg(test)]
mod tests {
    use crate::{defrag_map, hash_to_bitmap, num_regions, num_used_squares};

    #[test]
    fn test_hash_to_bitmap() {
        let hash = "a0c20170000000000000000000000000";
        let expected = "10100000110000100000000101110000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000";
        assert_eq!(hash_to_bitmap(hash), expected);
    }

    #[test]
    fn test_examples() {
        let input = "flqrgnkx";
        let map = defrag_map(input);

        assert_eq!(map.len(), 128);
        assert!(map[0].starts_with("##.#.#.."));
        assert!(map[1].starts_with(".#.#.#.#"));
        assert!(map[2].starts_with("....#.#."));
        assert!(map[3].starts_with("#.#.##.#"));
        assert!(map[4].starts_with(".##.#..."));
        assert!(map[5].starts_with("##..#..#"));
        assert!(map[6].starts_with(".#...#.."));
        assert!(map[7].starts_with("##.#.##."));

        assert_eq!(num_used_squares(&map), 8108);

        assert_eq!(num_regions(&map), 1242);
    }

    #[test]
    fn test_input() {
        let input = "uugsqrei";
        let map = defrag_map(input);

        assert_eq!(num_used_squares(&map), 8194);
        assert_eq!(num_regions(&map), 1141);
    }
}
