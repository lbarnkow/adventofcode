#![allow(dead_code)]

use std::ops::RangeInclusive;

fn main() {
    println!("Advent of Code 2019 - day 04");
}

fn u32_to_char_array(mut n: u32) -> [char; 6] {
    if n > 999_999 {
        panic!("Not a six-digit number: {n}!");
    }

    let mut result = ['0'; 6];

    for idx in (0..6).rev() {
        let digit = n % 10;
        n /= 10;
        result[idx] = char::from_u32('0' as u32 + digit).unwrap();
    }

    result
}

fn inc_pw(pw: &mut [char]) {
    for idx in (0..pw.len()).rev() {
        match pw[idx] {
            '9' => pw[idx] = '0',
            c => {
                pw[idx] = char::from_u32(c as u32 + 1).unwrap();
                break;
            }
        }
    }
}

fn two_adjacent_digits_are_the_same(pw: &[char]) -> bool {
    for idx in 1..pw.len() {
        if pw[idx] == pw[idx - 1] {
            return true;
        }
    }
    false
}

fn two_adjacent_digits_are_the_same_max(pw: &[char]) -> bool {
    let mut same_as_before_streak = 0;

    for idx in 1..pw.len() {
        if pw[idx] == pw[idx - 1] {
            same_as_before_streak += 1;
        } else if same_as_before_streak == 1 {
            return true;
        } else {
            same_as_before_streak = 0;
        }
    }

    same_as_before_streak == 1
}

fn digits_never_decrease(pw: &[char]) -> bool {
    for idx in 1..pw.len() {
        if pw[idx - 1] > pw[idx] {
            return false;
        }
    }
    true
}

fn is_pw_valid(pw: &[char], streak_rule: bool) -> bool {
    digits_never_decrease(pw)
        && if !streak_rule {
            two_adjacent_digits_are_the_same(pw)
        } else {
            two_adjacent_digits_are_the_same_max(pw)
        }
}

fn count_valid_passwords(range: RangeInclusive<u32>, streak_rule: bool) -> usize {
    let mut count = 0;

    let start = *range.start();
    let end = *range.end();
    let start = u32_to_char_array(start);
    let end = u32_to_char_array(end);

    let mut pw = start;
    if is_pw_valid(&pw, streak_rule) {
        count += 1;
    }
    while pw != end {
        inc_pw(&mut pw);
        if is_pw_valid(&pw, streak_rule) {
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use crate::{count_valid_passwords, inc_pw, is_pw_valid, u32_to_char_array};

    #[test]
    fn test_examples() {
        assert_eq!(u32_to_char_array(123), ['0', '0', '0', '1', '2', '3']);

        let mut n = u32_to_char_array(123);
        inc_pw(&mut n);
        assert_eq!(n, ['0', '0', '0', '1', '2', '4']);
        let mut n = u32_to_char_array(299_999);
        inc_pw(&mut n);
        assert_eq!(n, ['3', '0', '0', '0', '0', '0']);

        assert_eq!(is_pw_valid(&u32_to_char_array(111111), false), true);
        assert_eq!(is_pw_valid(&u32_to_char_array(223450), false), false);
        assert_eq!(is_pw_valid(&u32_to_char_array(123789), false), false);

        assert_eq!(is_pw_valid(&u32_to_char_array(112233), true), true);
        assert_eq!(is_pw_valid(&u32_to_char_array(123444), true), false);
        assert_eq!(is_pw_valid(&u32_to_char_array(111122), true), true);
    }

    #[test]
    fn test_input() {
        assert_eq!(count_valid_passwords(236491..=713787, false), 1169);
        assert_eq!(count_valid_passwords(236491..=713787, true), 757);
    }
}
