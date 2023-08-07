#![allow(dead_code)]

fn main() {
    println!("Advent of Code 2017 - day 01");
}

fn sum_repeating_digits(input: &str) -> u32 {
    if input.len() < 2 {
        return 0;
    }

    let mut sum = 0;
    let mut iter = input.chars();
    let c1 = iter.next().unwrap();
    let mut prev = c1;

    for c in iter {
        if prev == c {
            sum += prev.to_digit(10).unwrap();
        }
        prev = c;
    }

    if prev == c1 {
        if prev == c1 {
            sum += prev.to_digit(10).unwrap();
        }
    }

    sum
}

fn sum_halfway_digits(input: &str) -> u32 {
    if input.is_empty() {
        return 0;
    }
    if input.len() % 2 == 1 {
        panic!("Input string must have an even number of characters!")
    }

    let mut sum = 0;

    let mut left = Vec::with_capacity(input.len() / 2);
    let mut right = Vec::with_capacity(input.len() / 2);

    for (idx, c) in input.chars().enumerate() {
        if idx < input.len() / 2 {
            left.push(c);
        } else {
            right.push(c);
        }
    }

    for i in 0..input.len() / 2 {
        if left[i] == right[i] {
            sum += 2 * left[i].to_digit(10).unwrap();
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use crate::{sum_halfway_digits, sum_repeating_digits};

    #[test]
    fn test_examples() {
        let input = "1122";
        assert_eq!(sum_repeating_digits(input), 3);
        let input = "1111";
        assert_eq!(sum_repeating_digits(input), 4);
        let input = "1234";
        assert_eq!(sum_repeating_digits(input), 0);
        let input = "91212129";
        assert_eq!(sum_repeating_digits(input), 9);

        let input = "1212";
        assert_eq!(sum_halfway_digits(input), 6);
        let input = "1221";
        assert_eq!(sum_halfway_digits(input), 0);
        let input = "123425";
        assert_eq!(sum_halfway_digits(input), 4);
        let input = "123123";
        assert_eq!(sum_halfway_digits(input), 12);
        let input = "12131415";
        assert_eq!(sum_halfway_digits(input), 4);
    }

    #[test]
    fn test_input() {
        let input = std::fs::read_to_string("input/digits.txt").unwrap();

        assert_eq!(sum_repeating_digits(&input), 997);

        assert_eq!(sum_halfway_digits(&input), 1358);
    }
}
