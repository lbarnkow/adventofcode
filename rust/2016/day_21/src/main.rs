#![allow(dead_code)]

use lazy_static::lazy_static;
use regex::Regex;
use std::collections::VecDeque;

fn main() {
    println!("Advent of Code 2016 - day 21");
}

lazy_static! {
    static ref RE_SWAP_POS: Regex =
        Regex::new(r"^swap position (\d+) with position (\d+)$").unwrap();
    static ref RE_SWAP_LETTER: Regex = Regex::new(r"^swap letter (\w) with letter (\w)$").unwrap();
    static ref RE_ROTATE: Regex = Regex::new(r"^rotate (\w+) (\d+) steps?$").unwrap();
    static ref RE_ROTATE_LETTER_POS_BASED: Regex =
        Regex::new(r"^rotate based on position of letter (\w)$").unwrap();
    static ref RE_REVERSE_POS: Regex =
        Regex::new(r"^reverse positions (\d+) through (\d+)$").unwrap();
    static ref RE_MOVE_POS: Regex = Regex::new(r"^move position (\d+) to position (\d+)$").unwrap();
}

#[derive(Debug, Clone, Copy)]
enum Dir {
    Left,
    Right,
}

impl From<&str> for Dir {
    fn from(value: &str) -> Self {
        match value {
            "left" => Self::Left,
            "right" => Self::Right,
            _ => panic!("Illegal rotation direction!"),
        }
    }
}

impl Dir {
    fn rev(&self) -> Self {
        match self {
            Dir::Left => Self::Right,
            Dir::Right => Self::Left,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Rule {
    SwapPositions(usize, usize),
    SwapLetters(char, char),
    RotateSteps(Dir, usize),
    RotateBasedOnLetterPos(char),
    ReversePositions(usize, usize),
    MovePositions(usize, usize),
    UndoRotateBasedOnLetterPos(char),
}

impl Rule {
    fn do_swap_positions(input: &mut VecDeque<char>, pos_a: usize, pos_b: usize) {
        let tmp = input[pos_a];
        input[pos_a] = input[pos_b];
        input[pos_b] = tmp;
    }

    fn do_swap_letters(input: &mut VecDeque<char>, a: char, b: char) {
        for i in 0..input.len() {
            if input[i] == a {
                input[i] = b;
            } else if input[i] == b {
                input[i] = a;
            }
        }
    }

    fn do_rotate_steps(input: &mut VecDeque<char>, dir: Dir, n: usize) {
        for _ in 0..n {
            match dir {
                Dir::Left => {
                    let c = input.pop_front().unwrap();
                    input.push_back(c);
                }
                Dir::Right => {
                    let c = input.pop_back().unwrap();
                    input.push_front(c);
                }
            }
        }
    }

    fn do_rotate_based_on_letter_pos(input: &mut VecDeque<char>, a: char) {
        let mut pos = None;
        for (idx, c) in input.iter().enumerate() {
            if *c == a {
                pos = Some(idx);
                break;
            }
        }
        let pos = pos.unwrap();
        let n = 1 + pos + if pos >= 4 { 1 } else { 0 };

        Self::do_rotate_steps(input, Dir::Right, n);
    }

    fn do_reverse_positions(input: &mut VecDeque<char>, pos_a: usize, pos_b: usize) {
        let reversed_chars = input
            .range(pos_a.min(pos_b)..=pos_b.max(pos_a))
            .rev()
            .map(|c| *c)
            .collect::<Vec<char>>();

        for (idx, c) in (pos_a..=pos_b).zip(reversed_chars) {
            input[idx] = c;
        }
    }

    fn do_move_positions(input: &mut VecDeque<char>, pos_a: usize, pos_b: usize) {
        let c = input.remove(pos_a).unwrap();
        input.insert(pos_b, c);
    }

    fn undo_rotate_based_on_letter_pos(input: &mut VecDeque<char>, a: char) {
        if input.len() % 2 == 1 {
            panic!("Input length must be even to undo rotation based on letter position!")
        }

        let (end_idx, _) = input
            .iter()
            .enumerate()
            .filter(|(_, c)| a == **c)
            .next()
            .unwrap();

        let (start_idx, _) = (0..input.len())
            .enumerate()
            .map(|(idx, i)| (idx, i + 1 + i + if i >= 4 { 1 } else { 0 }))
            .map(|(idx, i)| (idx, i % input.len()))
            .filter(|(_, i)| *i == end_idx)
            .next()
            .unwrap();

        if start_idx < end_idx {
            Self::do_rotate_steps(input, Dir::Left, end_idx - start_idx);
        } else {
            Self::do_rotate_steps(input, Dir::Right, start_idx - end_idx);
        }
    }

    fn apply(&self, input: &mut VecDeque<char>) {
        match self {
            Rule::SwapPositions(a, b) => Self::do_swap_positions(input, *a, *b),
            Rule::SwapLetters(a, b) => Self::do_swap_letters(input, *a, *b),
            Rule::RotateSteps(dir, n) => Self::do_rotate_steps(input, *dir, *n),
            Rule::RotateBasedOnLetterPos(a) => Self::do_rotate_based_on_letter_pos(input, *a),
            Rule::ReversePositions(a, b) => Self::do_reverse_positions(input, *a, *b),
            Rule::MovePositions(a, b) => Self::do_move_positions(input, *a, *b),
            Rule::UndoRotateBasedOnLetterPos(a) => Self::undo_rotate_based_on_letter_pos(input, *a),
        }
    }
}

impl From<&str> for Rule {
    fn from(value: &str) -> Self {
        if let Some(caps) = RE_SWAP_POS.captures(value) {
            Rule::SwapPositions(caps[1].parse().unwrap(), caps[2].parse().unwrap())
        } else if let Some(caps) = RE_SWAP_LETTER.captures(value) {
            Rule::SwapLetters(
                caps[1].chars().next().unwrap(),
                caps[2].chars().next().unwrap(),
            )
        } else if let Some(caps) = RE_ROTATE.captures(value) {
            Rule::RotateSteps(Dir::from(&caps[1]), caps[2].parse().unwrap())
        } else if let Some(caps) = RE_ROTATE_LETTER_POS_BASED.captures(value) {
            Rule::RotateBasedOnLetterPos(caps[1].chars().next().unwrap())
        } else if let Some(caps) = RE_REVERSE_POS.captures(value) {
            Rule::ReversePositions(caps[1].parse().unwrap(), caps[2].parse().unwrap())
        } else if let Some(caps) = RE_MOVE_POS.captures(value) {
            Rule::MovePositions(caps[1].parse().unwrap(), caps[2].parse().unwrap())
        } else {
            panic!("Illegal rule: {value}");
        }
    }
}

fn parse_rules(rules: &str) -> Vec<Rule> {
    rules.lines().map(|line| Rule::from(line)).collect()
}

fn scramble(plain: &str, rules: &Vec<Rule>) -> String {
    let mut input = plain.chars().collect::<VecDeque<char>>();

    for rule in rules {
        rule.apply(&mut input);
    }

    input.into_iter().collect()
}

fn unscramble(scrambled: &str, rules: &Vec<Rule>) -> String {
    let rules: Vec<Rule> = rules
        .iter()
        .map(|rule| match rule {
            Rule::SwapPositions(a, b) => Rule::SwapPositions(*a, *b),
            Rule::SwapLetters(a, b) => Rule::SwapLetters(*a, *b),
            Rule::RotateSteps(dir, n) => Rule::RotateSteps(dir.rev(), *n),
            Rule::RotateBasedOnLetterPos(a) => Rule::UndoRotateBasedOnLetterPos(*a),
            Rule::ReversePositions(a, b) => Rule::ReversePositions(*a, *b),
            Rule::MovePositions(a, b) => Rule::MovePositions(*b, *a),
            Rule::UndoRotateBasedOnLetterPos(_) => panic!("Illegal rule to undo!"),
        })
        .rev()
        .collect();

    scramble(scrambled, &rules)
}

#[cfg(test)]
mod tests {
    use crate::{parse_rules, scramble, unscramble};

    #[test]
    fn test_example() {
        let rules = "\
            swap position 4 with position 0\n\
            swap letter d with letter b\n\
            reverse positions 0 through 4\n\
            rotate left 1 step\n\
            move position 1 to position 4\n\
            move position 3 to position 0\n\
            rotate based on position of letter b\n\
            rotate based on position of letter d\n\
        ";
        let rules = parse_rules(rules);

        let input = "abcde";
        assert_eq!(scramble(input, &rules), "decab");

        let input = "abcdef";
        assert_eq!(scramble(input, &rules), "cafbde");

        let input = "cafbde";
        assert_eq!(unscramble(input, &rules), "abcdef");
    }

    #[test]
    fn test_input() {
        let rules = std::fs::read_to_string("input/rules.txt").unwrap();
        let rules = parse_rules(&rules);

        let input = "abcdefgh";
        assert_eq!(scramble(input, &rules), "ghfacdbe");

        assert_eq!(unscramble("fbgdceah", &rules), "fhgcdaeb");
    }
}
