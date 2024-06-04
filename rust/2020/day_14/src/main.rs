#![allow(dead_code)]

use std::{collections::HashMap, fmt::Display};

use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    println!("Advent of Code 2020 - day 14");
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

lazy_static! {
    static ref RE_MASK: Regex = Regex::new(r"^mask\s+=\s+([01X]{36})$").unwrap();
    static ref RE_MEM: Regex = Regex::new(r"^mem\[(\d+)\]\s+=\s+(\d+)$").unwrap();
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum ChipVersion {
    V1,
    V2,
}

#[derive(Debug, Clone)]
enum Instruction {
    ValueMask { and_mask: u64, or_mask: u64 },
    AddrMask { masks: Vec<(u64, u64)> },
    MemAssign { address: usize, value: u64 },
}

impl Instruction {
    fn parse_addr_mask(value: &str) -> Result<Self, TryFromError> {
        let Some(caps) = RE_MASK.captures(value) else {
            return Err(format!("Not a valid bit mask: '{value}'!").into());
        };

        let Some(mask) = caps.get(1) else {
            return Err(format!("Not a valid bit mask: '{value}'!").into());
        };

        let mut masks = vec![(u64::MAX, 0)];

        for c in mask.as_str().chars() {
            let mut alt = Vec::new();
            for mask in masks.iter_mut() {
                mask.0 <<= 1;
                mask.0 += 1;
                mask.1 <<= 1;

                match c {
                    '0' => (),
                    '1' => mask.1 += 1,
                    'X' => {
                        mask.0 -= 1;
                        alt.push((mask.0, mask.1 + 1));
                    }
                    _ => return Err(format!("Not a valid bit mask: '{value}'!").into()),
                }
            }
            masks.append(&mut alt);
        }

        Ok(Self::AddrMask { masks })
    }

    fn parse_value_mask(value: &str) -> Result<Self, TryFromError> {
        let Some(caps) = RE_MASK.captures(value) else {
            return Err(format!("Not a valid bit mask: '{value}'!").into());
        };

        let Some(mask) = caps.get(1) else {
            return Err(format!("Not a valid bit mask: '{value}'!").into());
        };

        let (mut and_mask, mut or_mask) = (u64::MAX, 0);
        for c in mask.as_str().chars() {
            and_mask <<= 1;
            and_mask += 1;
            or_mask <<= 1;

            match c {
                '0' => and_mask ^= 0x1,
                '1' => or_mask += 0x1,
                'X' => (),
                _ => return Err(format!("Not a valid bit mask: '{value}'!").into()),
            }
        }

        Ok(Self::ValueMask { and_mask, or_mask })
    }

    fn parse_mem_assign(value: &str) -> Result<Self, TryFromError> {
        let Some(caps) = RE_MEM.captures(value) else {
            return Err(format!("Not a valid mem assignment: '{value}'!").into());
        };

        let Some(address) = caps.get(1) else {
            return Err(format!("Not a valid mem assignment: '{value}'!").into());
        };
        let Some(value) = caps.get(2) else {
            return Err(format!("Not a valid mem assignment: '{value}'!").into());
        };

        let Ok(address) = address.as_str().parse::<usize>() else {
            return Err(format!("Not a valid address: '{}'!", address.as_str()).into());
        };
        let Ok(value) = value.as_str().parse::<u64>() else {
            return Err(format!("Not a valid value: '{}'!", value.as_str()).into());
        };

        Ok(Self::MemAssign { address, value })
    }
}

impl TryFrom<(ChipVersion, &str)> for Instruction {
    type Error = TryFromError;

    fn try_from((version, value): (ChipVersion, &str)) -> Result<Self, Self::Error> {
        if value.starts_with("mem") {
            Self::parse_mem_assign(value)
        } else if value.starts_with("mask") && version == ChipVersion::V1 {
            Self::parse_value_mask(value)
        } else if value.starts_with("mask") && version == ChipVersion::V2 {
            Self::parse_addr_mask(value)
        } else {
            Err(format!("Not a legal instruction: '{value}'!").into())
        }
    }
}

struct Program {
    version: ChipVersion,
    instructions: Vec<Instruction>,
}

impl TryFrom<(ChipVersion, &str)> for Program {
    type Error = TryFromError;

    fn try_from((version, value): (ChipVersion, &str)) -> Result<Self, Self::Error> {
        let mut instructions = Vec::new();

        for line in value.lines() {
            instructions.push((version, line).try_into()?);
        }

        Ok(Self {
            version,
            instructions,
        })
    }
}

impl Program {
    fn apply(&self) -> u64 {
        let mut mem = HashMap::new();

        let mut cur_mask = Instruction::ValueMask {
            and_mask: 0,
            or_mask: 0,
        };

        for instruction in &self.instructions {
            match instruction {
                Instruction::MemAssign { address, value } => match cur_mask {
                    Instruction::MemAssign {
                        address: _,
                        value: _,
                    } => panic!("Should never happen!"),
                    Instruction::ValueMask { and_mask, or_mask } => {
                        let mut value = *value;
                        value &= and_mask;
                        value |= or_mask;
                        mem.insert(*address, value);
                    }
                    Instruction::AddrMask { ref masks } => {
                        for (and_mask, or_mask) in masks {
                            let mut address = *address;
                            address &= *and_mask as usize;
                            address |= *or_mask as usize;
                            mem.insert(address, *value);
                        }
                    }
                },
                otherwise => cur_mask = otherwise.clone(),
            }
        }

        mem.values().sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::{ChipVersion, Program, TryFromError};

    #[test]
    fn test_examples() -> Result<(), TryFromError> {
        let raw = "\
            mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X\n\
            mem[8] = 11\n\
            mem[7] = 101\n\
            mem[8] = 0\
        ";
        let raw = Program::try_from((ChipVersion::V1, raw))?;
        assert_eq!(raw.apply(), 165);

        let prg = "\
            mask = 000000000000000000000000000000X1001X\n\
            mem[42] = 100\n\
            mask = 00000000000000000000000000000000X0XX\n\
            mem[26] = 1\
        ";
        let prg = Program::try_from((ChipVersion::V2, prg))?;
        assert_eq!(prg.apply(), 208);

        Ok(())
    }

    #[test]
    fn test_input() -> Result<(), TryFromError> {
        let raw = std::fs::read_to_string("input/program.txt").unwrap();

        let prg = Program::try_from((ChipVersion::V1, raw.as_str()))?;
        assert_eq!(prg.apply(), 10035335144067);

        let prg = Program::try_from((ChipVersion::V2, raw.as_str()))?;
        assert_eq!(prg.apply(), 3817372618036);

        Ok(())
    }
}
