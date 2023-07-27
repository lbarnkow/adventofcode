#![allow(dead_code)]

use std::{
    collections::{HashMap, VecDeque},
    vec,
};

use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    println!("Advent of Code 2016 - day 09");
}

lazy_static! {
    static ref RE_SUBJECT: Regex = Regex::new(r"^(\w+) (\d+)$").unwrap();
    static ref RE_VALUE_GOES_TO_SUBJECT: Regex =
        Regex::new(r"^value (\d+) goes to (\w+ \d+)$").unwrap();
    static ref RE_SUBJECT_GIVES_LOW_AND_HIGH: Regex =
        Regex::new(r"^(\w+ \d+) gives low to (\w+ \d+) and high to (\w+ \d+)$").unwrap();
}

enum Subject {
    Bot(usize),
    Output(usize),
}

impl Subject {
    fn from(s: &str) -> Self {
        let caps = RE_SUBJECT.captures(s).unwrap();
        match &caps[1] {
            "bot" => Self::Bot(caps[2].parse::<usize>().unwrap()),
            "output" => Self::Output(caps[2].parse::<usize>().unwrap()),
            _ => panic!("Illegal subject: {s}"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    ValueGoesToBot {
        chip: usize,
        bot: usize,
    },
    BotGivesLowToBotAndHighToBot {
        source: usize,
        low: usize,
        high: usize,
    },
    BotGivesLowToOutputAndHighToBot {
        source: usize,
        low: usize,
        high: usize,
    },
    BotGivesLowToOutputAndHighToOutput {
        source: usize,
        low: usize,
        high: usize,
    },
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        if let Some(caps) = RE_VALUE_GOES_TO_SUBJECT.captures(value) {
            let sub = Subject::from(&caps[2]);
            match sub {
                Subject::Bot(id) => Self::ValueGoesToBot {
                    chip: caps[1].parse::<usize>().unwrap(),
                    bot: id,
                },
                Subject::Output(_) => panic!("Unsupported instruction: {value}"),
            }
        } else if let Some(caps) = RE_SUBJECT_GIVES_LOW_AND_HIGH.captures(value) {
            let sub = Subject::from(&caps[1]);
            let low = Subject::from(&caps[2]);
            let high = Subject::from(&caps[3]);

            match (sub, low, high) {
                (Subject::Bot(s), Subject::Bot(l), Subject::Bot(h)) => {
                    Self::BotGivesLowToBotAndHighToBot {
                        source: s,
                        low: l,
                        high: h,
                    }
                }
                (Subject::Bot(s), Subject::Output(l), Subject::Bot(h)) => {
                    Self::BotGivesLowToOutputAndHighToBot {
                        source: s,
                        low: l,
                        high: h,
                    }
                }
                (Subject::Bot(s), Subject::Output(l), Subject::Output(h)) => {
                    Self::BotGivesLowToOutputAndHighToOutput {
                        source: s,
                        low: l,
                        high: h,
                    }
                }
                _ => panic!("Unsupported instruction: {value}"),
            }
        } else {
            panic!("Illegal instruction: {value}");
        }
    }
}

impl Instruction {
    fn from_multi(instructions: &str) -> VecDeque<Self> {
        instructions.lines().map(|line| Self::from(line)).collect()
    }
}

fn release_from_bot(bots: &mut HashMap<usize, Vec<usize>>, bot: usize) -> Option<(usize, usize)> {
    if let Some(chips) = bots.get_mut(&bot) {
        if chips.len() == 2 {
            chips.sort();
            let h = chips.pop().unwrap();
            let l = chips.pop().unwrap();
            return Some((l, h));
        }
    }
    None
}

fn give_chip_to_bot_or_output(dest: &mut HashMap<usize, Vec<usize>>, dest_key: usize, chip: usize) {
    if let Some(chips) = dest.get_mut(&dest_key) {
        chips.push(chip);
    } else {
        dest.insert(dest_key, vec![chip]);
    }
}

enum Query {
    BotWithSpecificChips(usize, usize),
    Outputs(Vec<usize>),
}

fn find_bot_handling_specific_chips(instructions: &VecDeque<Instruction>, query: Query) -> usize {
    let mut instructions = instructions.clone();

    let mut bots: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut outputs: HashMap<usize, Vec<usize>> = HashMap::new();

    while let Some(instruction) = instructions.pop_front() {
        match instruction {
            Instruction::ValueGoesToBot { chip, bot } => {
                give_chip_to_bot_or_output(&mut bots, bot, chip);
                continue;
            }
            Instruction::BotGivesLowToBotAndHighToBot { source, low, high } => {
                if let Some((l, h)) = release_from_bot(&mut bots, source) {
                    if let Query::BotWithSpecificChips(c1, c2) = query {
                        if (c1 == l && c2 == h) || (c2 == l && c1 == h) {
                            return source;
                        }
                    }
                    give_chip_to_bot_or_output(&mut bots, low, l);
                    give_chip_to_bot_or_output(&mut bots, high, h);
                    continue;
                }
            }
            Instruction::BotGivesLowToOutputAndHighToBot { source, low, high } => {
                if let Some((l, h)) = release_from_bot(&mut bots, source) {
                    if let Query::BotWithSpecificChips(c1, c2) = query {
                        if (c1 == l && c2 == h) || (c2 == l && c1 == h) {
                            return source;
                        }
                    }
                    give_chip_to_bot_or_output(&mut outputs, low, l);
                    give_chip_to_bot_or_output(&mut bots, high, h);
                    continue;
                }
            }
            Instruction::BotGivesLowToOutputAndHighToOutput { source, low, high } => {
                if let Some((l, h)) = release_from_bot(&mut bots, source) {
                    if let Query::BotWithSpecificChips(c1, c2) = query {
                        if (c1 == l && c2 == h) || (c2 == l && c1 == h) {
                            return source;
                        }
                    }
                    give_chip_to_bot_or_output(&mut outputs, low, l);
                    give_chip_to_bot_or_output(&mut outputs, high, h);
                    continue;
                }
            }
        }
        instructions.push_back(instruction);
    }

    match query {
        Query::Outputs(o) => {
            return o
                .iter()
                .map(|idx| outputs.get(idx).unwrap().first().unwrap())
                .fold(1, |acc, chip| acc * *chip);
        }
        Query::BotWithSpecificChips(c1, c2) => panic!("No bot handled chips {c1} and {c2}!"),
    }
}

#[cfg(test)]
mod tests {
    use crate::{find_bot_handling_specific_chips, Instruction, Query};

    #[test]
    fn test_example() {
        let instructions = "\
            value 5 goes to bot 2\n\
            bot 2 gives low to bot 1 and high to bot 0\n\
            value 3 goes to bot 1\n\
            bot 1 gives low to output 1 and high to bot 0\n\
            bot 0 gives low to output 2 and high to output 0\n\
            value 2 goes to bot 2\
        ";

        let instructions = Instruction::from_multi(instructions);
        assert_eq!(
            find_bot_handling_specific_chips(&instructions, Query::BotWithSpecificChips(5, 2)),
            2
        );
        assert_eq!(
            find_bot_handling_specific_chips(&instructions, Query::BotWithSpecificChips(2, 3)),
            1
        );
        assert_eq!(
            find_bot_handling_specific_chips(&instructions, Query::BotWithSpecificChips(5, 3)),
            0
        );
    }

    #[test]
    fn test_input() {
        let instructions = std::fs::read_to_string("input/instructions.txt").unwrap();

        let instructions = Instruction::from_multi(&instructions);
        assert_eq!(
            find_bot_handling_specific_chips(&instructions, Query::BotWithSpecificChips(61, 17)),
            98
        );
        assert_eq!(
            find_bot_handling_specific_chips(&instructions, Query::Outputs(vec![0, 1, 2])),
            4042
        );
    }
}
