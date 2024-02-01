#![allow(dead_code)]

use std::collections::VecDeque;

fn main() {
    println!("Advent of Code 2020 - day 09");
}

fn find_corrupt_xmas_number(input: &str, preamble_len: usize) -> u128 {
    let mut lines = input.lines();

    let mut sum_buffer: VecDeque<Vec<u128>> = VecDeque::with_capacity(preamble_len - 1);
    for _ in 0..(preamble_len - 1) {
        sum_buffer.push_back(vec![0; preamble_len - 1])
    }

    let mut buffer: VecDeque<u128> = VecDeque::with_capacity(preamble_len);
    for i in 0..preamble_len {
        let number = lines.next().unwrap().parse().unwrap();
        buffer.push_back(number);
        for (j, buffer_item) in buffer.iter().enumerate().take(i) {
            sum_buffer[i - 1][j] = number + buffer_item;
        }
    }

    for line in lines {
        let number = line.parse().unwrap();

        let mut found = false;
        for (i, sum_buffer_item) in sum_buffer.iter().enumerate().take(preamble_len - 1) {
            for sum_buffer_item_subitem in sum_buffer_item.iter().take(i + 1) {
                if number == *sum_buffer_item_subitem {
                    found = true;
                    break;
                }
            }
            if found {
                break;
            }
        }

        if !found {
            return number;
        }

        buffer.rotate_left(1);
        sum_buffer.rotate_left(1);
        for sum_buffer_item in &mut sum_buffer {
            sum_buffer_item.rotate_left(1);
        }

        buffer[preamble_len - 1] = number;

        for (j, buffer_item) in buffer.iter().enumerate().take(preamble_len - 1) {
            sum_buffer[preamble_len - 1 - 1][j] = number + buffer_item;
        }
    }

    panic!("No corrupt number found!");
}

fn find_xmas_weakness(input: &str, corrupt_number: u128) -> u128 {
    let mut buffer = VecDeque::new();
    let mut sum = 0;

    for line in input.lines() {
        let number: u128 = line.parse().unwrap();
        buffer.push_back(number);
        sum += number;

        if buffer.len() < 2 {
            continue;
        }

        while sum > corrupt_number {
            sum -= buffer.pop_front().unwrap();
        }

        if sum == corrupt_number {
            let min_max = buffer
                .into_iter()
                .fold((u128::MAX, u128::MIN), |(min, max), n| {
                    (if n < min { n } else { min }, if n > max { n } else { max })
                });

            return min_max.0 + min_max.1;
        }
    }

    panic!("No weakness found!");
}

#[cfg(test)]
mod tests {
    use crate::{find_corrupt_xmas_number, find_xmas_weakness};

    #[test]
    fn test_examples() {
        let xmas = "\
            35\n\
            20\n\
            15\n\
            25\n\
            47\n\
            40\n\
            62\n\
            55\n\
            65\n\
            95\n\
            102\n\
            117\n\
            150\n\
            182\n\
            127\n\
            219\n\
            299\n\
            277\n\
            309\n\
            576\
        ";

        let corrupt_number = find_corrupt_xmas_number(xmas, 5);
        assert_eq!(corrupt_number, 127);

        let weakness = find_xmas_weakness(xmas, corrupt_number);
        assert_eq!(weakness, 62);
    }

    #[test]
    fn test_input() {
        let xmas = std::fs::read_to_string("input/xmas.txt").unwrap();

        let corrupt_number = find_corrupt_xmas_number(&xmas, 25);
        assert_eq!(corrupt_number, 26796446);

        let weakness = find_xmas_weakness(&xmas, corrupt_number);
        assert_eq!(weakness, 3353494);
    }
}
