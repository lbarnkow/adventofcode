#![allow(dead_code)]

use lazy_static::lazy_static;
use regex::Regex;
use std::ops::RangeInclusive;

fn main() {
    println!("Advent of Code 2020 - day 02");
}

struct TryFromError {
    msg: String,
}

lazy_static! {
    static ref RE_POLICY: Regex = Regex::new(r"^(\d+)-(\d+)\s(\w)$").unwrap();
}

struct Policy {
    range: RangeInclusive<usize>,
    chr: char,
}

impl TryFrom<&str> for Policy {
    type Error = TryFromError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Some(caps) = RE_POLICY.captures(value) {
            let min = caps[1].parse::<usize>();
            let max = caps[2].parse::<usize>();
            let chr = caps[3].chars().next();

            if let (Ok(min), Ok(max), Some(chr)) = (min, max, chr) {
                return Ok(Self {
                    range: min..=max,
                    chr,
                });
            }
        }

        Err(Self::Error {
            msg: format!("Input '{value}' did not match a valid policy!"),
        })
    }
}

struct Password {
    policy: Policy,
    str: String,
}

impl TryFrom<&str> for Password {
    type Error = TryFromError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut split = value.split(": ");
        let policy = split.next();
        let password = split.next();

        if let (None, Some(policy), Some(password)) = (split.next(), policy, password) {
            Ok(Self {
                policy: policy.try_into()?,
                str: password.to_owned(),
            })
        } else {
            Err(Self::Error {
                msg: format!("Input '{value}' did not match a valid password entry!"),
            })
        }
    }
}

impl Password {
    fn is_valid(&self) -> bool {
        let count = self.str.chars().filter(|c| *c == self.policy.chr).count();

        count >= *self.policy.range.start() && count <= *self.policy.range.end()
    }

    fn is_officially_valid(&self) -> bool {
        let a = *self.policy.range.start() - 1;
        let b = *self.policy.range.end() - 1;

        let a = self.str[a..a + 1].chars().next().unwrap();
        let b = self.str[b..b + 1].chars().next().unwrap();

        (a == self.policy.chr) ^ (b == self.policy.chr)
    }
}

#[cfg(test)]
mod tests {
    use crate::Password;

    #[test]
    fn test_examples() {
        let list = "\
            1-3 a: abcde\n\
            1-3 b: cdefg\n\
            2-9 c: ccccccccc\
        ";

        let list = list
            .lines()
            .map(|line| match Password::try_from(line) {
                Ok(pw) => pw,
                Err(e) => panic!("ERR: {}", e.msg),
            })
            .collect::<Vec<Password>>();

        let valid_count = list.iter().filter(|pw| pw.is_valid()).count();
        assert_eq!(valid_count, 2);

        let valid_count = list.iter().filter(|pw| pw.is_officially_valid()).count();
        assert_eq!(valid_count, 1);
    }

    #[test]
    fn test_input() {
        let list = std::fs::read_to_string("input/pwlist.txt").unwrap();

        let list = list
            .lines()
            .map(|line| match Password::try_from(line) {
                Ok(pw) => pw,
                Err(e) => panic!("ERR: {}", e.msg),
            })
            .collect::<Vec<Password>>();

        let valid_count = list.iter().filter(|pw| pw.is_valid()).count();
        assert_eq!(valid_count, 569);

        let valid_count = list.iter().filter(|pw| pw.is_officially_valid()).count();
        assert_eq!(valid_count, 346);
    }
}
