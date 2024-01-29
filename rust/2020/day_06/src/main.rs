#![allow(dead_code)]

use std::{fmt::Display, num::ParseIntError, ops::RangeInclusive};

fn main() {
    println!("Advent of Code 2020 - day 06");
}

#[derive(Debug)]
struct TryFromError {
    msg: String,
}

impl From<&str> for TryFromError {
    fn from(msg: &str) -> Self {
        Self {
            msg: msg.to_owned(),
        }
    }
}

impl From<String> for TryFromError {
    fn from(msg: String) -> Self {
        Self { msg }
    }
}

impl Display for TryFromError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ERR: {}", &self.msg)
    }
}

impl From<ParseIntError> for TryFromError {
    fn from(value: ParseIntError) -> Self {
        value.to_string().into()
    }
}

struct Form {
    answered: [u8; 26],
}

const CP_A: usize = 'a' as usize;
static VALID_RANGE: RangeInclusive<usize> = ('a' as usize)..=('z' as usize);

impl TryFrom<&str> for Form {
    type Error = TryFromError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut answered = [0; 26];

        for c in value.chars() {
            let cp = c as usize;
            if !VALID_RANGE.contains(&cp) {
                return Err(format!(
                    "Customs form '{}' contains illegal character '{}'!",
                    value, c
                )
                .into());
            }
            let idx = cp - CP_A;
            answered[idx] = 1;
        }

        Ok(Self { answered })
    }
}

struct Group {
    forms: Vec<Form>,
}

impl TryFrom<&str> for Group {
    type Error = TryFromError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut forms = Vec::with_capacity(value.lines().count());
        for line in value.lines() {
            forms.push(line.try_into()?);
        }

        Ok(Self { forms })
    }
}

impl Group {
    fn try_from_batch(groups: &str) -> Result<Vec<Self>, TryFromError> {
        let split = groups.split("\n\n");
        let mut groups = Vec::new();

        for group in split {
            groups.push(group.try_into()?);
        }

        Ok(groups)
    }

    fn count_answered_questions_by_anyone(&self) -> usize {
        self.forms
            .iter()
            .fold(vec![0u8; 26], |acc, form| {
                form.answered
                    .iter()
                    .zip(acc)
                    .map(|(a, b)| *a + b)
                    .collect::<Vec<u8>>()
            })
            .into_iter()
            .filter(|a| *a > 0)
            .count()
    }

    fn count_answered_questions_by_everyone(&self) -> usize {
        let len = self.forms.len() as u8;

        self.forms
            .iter()
            .fold(vec![0u8; 26], |acc, form| {
                form.answered
                    .iter()
                    .zip(acc)
                    .map(|(a, b)| *a + b)
                    .collect::<Vec<u8>>()
            })
            .into_iter()
            .filter(|a| *a == len)
            .count()
    }
}

#[cfg(test)]
mod tests {
    use crate::{Group, TryFromError};

    #[test]
    fn test_examples() -> Result<(), TryFromError> {
        let groups = "\
            abc\n\
            \n\
            a\n\
            b\n\
            c\n\
            \n\
            ab\n\
            ac\n\
            \n\
            a\n\
            a\n\
            a\n\
            a\n\
            \n\
            b\
        ";
        let groups = Group::try_from_batch(groups)?;

        let sum_of_counts: usize = groups
            .iter()
            .map(Group::count_answered_questions_by_anyone)
            .sum();
        assert_eq!(sum_of_counts, 11);

        let sum_of_counts: usize = groups
            .iter()
            .map(Group::count_answered_questions_by_everyone)
            .sum();
        assert_eq!(sum_of_counts, 6);

        Ok(())
    }

    #[test]
    fn test_input() -> Result<(), TryFromError> {
        let groups = std::fs::read_to_string("input/forms.txt").unwrap();
        let groups = Group::try_from_batch(&groups)?;

        let sum_of_counts: usize = groups
            .iter()
            .map(Group::count_answered_questions_by_anyone)
            .sum();
        assert_eq!(sum_of_counts, 6416);

        let sum_of_counts: usize = groups
            .iter()
            .map(Group::count_answered_questions_by_everyone)
            .sum();
        assert_eq!(sum_of_counts, 3050);

        Ok(())
    }
}
