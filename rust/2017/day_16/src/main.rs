#![allow(dead_code)]

use std::{collections::HashMap, str::Chars};

use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    println!("Advent of Code 2017 - day 16");
}

lazy_static! {
    static ref RE_SPIN: Regex = Regex::new(r"^s(\d+)$").unwrap();
    static ref RE_EXCHANGE: Regex = Regex::new(r"^x(\d+)/(\d+)$").unwrap();
    static ref RE_PARTNER: Regex = Regex::new(r"^p(\w)/(\w)$").unwrap();
}

#[derive(Debug)]
enum DanceMove {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

impl From<&str> for DanceMove {
    fn from(value: &str) -> Self {
        if let Some(caps) = RE_SPIN.captures(value) {
            Self::Spin(caps[1].parse::<usize>().unwrap())
        } else if let Some(caps) = RE_EXCHANGE.captures(value) {
            Self::Exchange(
                caps[1].parse::<usize>().unwrap(),
                caps[2].parse::<usize>().unwrap(),
            )
        } else if let Some(caps) = RE_PARTNER.captures(value) {
            Self::Partner(
                caps[1].chars().next().unwrap(),
                caps[2].chars().next().unwrap(),
            )
        } else {
            panic!("Illegal dance move: {value}!")
        }
    }
}

impl DanceMove {
    fn from_list(moves: &str) -> Vec<Self> {
        moves.split(",").map(|line| Self::from(line)).collect()
    }

    fn apply_spin(&self, dancers: &mut Vec<char>, n: usize) {
        for _ in 0..n {
            let tmp = dancers.pop().unwrap();
            dancers.insert(0, tmp);
        }
    }

    fn apply_exchange(&self, dancers: &mut Vec<char>, a: usize, b: usize) {
        let tmp = dancers[a];
        dancers[a] = dancers[b];
        dancers[b] = tmp;
    }

    fn apply_partner(&self, dancers: &mut Vec<char>, a: char, b: char) {
        let to_swap: Vec<usize> = dancers
            .iter()
            .enumerate()
            .filter(|(_, c)| a == **c || b == **c)
            .map(|(idx, _)| idx)
            .collect();
        self.apply_exchange(dancers, to_swap[0], to_swap[1]);
    }

    fn apply(&self, dancers: &mut Vec<char>) {
        match self {
            DanceMove::Spin(n) => self.apply_spin(dancers, *n),
            DanceMove::Exchange(a, b) => self.apply_exchange(dancers, *a, *b),
            DanceMove::Partner(a, b) => self.apply_partner(dancers, *a, *b),
        }
    }
}

fn perform_dance(dancers: &mut Vec<char>, moves: &Vec<DanceMove>, rounds: usize) {
    let mut seen = HashMap::new();
    let mut remaining = 0;

    for idx in 1..=rounds {
        for mv in moves {
            mv.apply(dancers);
        }
        if let Some(seen_idx) = seen.get(dancers) {
            let span = idx - seen_idx;
            remaining = (rounds - idx) % span;
            break;
        }
        seen.insert(dancers.clone(), idx);
    }

    for _ in 0..remaining {
        for mv in moves {
            mv.apply(dancers);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{perform_dance, DanceMove};

    #[test]
    fn test_examples() {
        let moves = "s1,x3/4,pe/b";
        let moves = DanceMove::from_list(moves);
        let mut dancers: Vec<char> = ('a'..='e').collect();

        moves.iter().for_each(|mv| mv.apply(&mut dancers));
        let dancers: String = dancers.iter().collect();

        assert_eq!(dancers, "baedc");
    }

    #[test]
    fn test_examples_part2() {
        let moves = "s1,x3/4,pe/b";
        let moves = DanceMove::from_list(moves);
        let mut dancers: Vec<char> = ('a'..='e').collect();

        perform_dance(&mut dancers, &moves, 2);
        let dancers: String = dancers.iter().collect();

        assert_eq!(dancers, "ceadb");
    }

    #[test]
    fn test_input() {
        let moves = std::fs::read_to_string("input/moves.txt").unwrap();
        let moves = DanceMove::from_list(&moves);
        let mut dancers: Vec<char> = ('a'..='p').collect();

        moves.iter().for_each(|mv| mv.apply(&mut dancers));
        let dancers: String = dancers.iter().collect();

        assert_eq!(dancers, "kpfonjglcibaedhm");
    }

    #[test]
    fn test_input_part2() {
        let moves = std::fs::read_to_string("input/moves.txt").unwrap();
        let moves = DanceMove::from_list(&moves);
        let mut dancers: Vec<char> = ('a'..='p').collect();

        perform_dance(&mut dancers, &moves, 1_000_000_000);
        let dancers: String = dancers.iter().collect();

        assert_eq!(dancers, "odiabmplhfgjcekn");
    }
}
