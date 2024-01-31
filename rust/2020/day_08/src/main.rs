#![allow(dead_code)]

use core::panic;
use std::{collections::HashSet, fmt::Display};

fn main() {
    println!("Advent of Code 2020 - day 08");
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

#[derive(Debug, Clone, Copy, Default)]
struct Operand {
    value: isize,
}

impl From<isize> for Operand {
    fn from(value: isize) -> Self {
        Self { value }
    }
}

impl TryFrom<&str> for Operand {
    type Error = TryFromError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value
            .parse()
            .map_err(|_| -> TryFromError { format!("Illegal operand '{value}'!").into() })?;
        Ok(Self { value })
    }
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Nop(Operand),
    Acc(Operand),
    Jmp(Operand),
}

impl TryFrom<&str> for Instruction {
    type Error = TryFromError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Some(nop) = value.strip_prefix("nop ") {
            Ok(Self::Nop(nop.try_into()?))
        } else if let Some(acc) = value.strip_prefix("acc ") {
            Ok(Self::Acc(acc.try_into()?))
        } else if let Some(jmp) = value.strip_prefix("jmp ") {
            Ok(Self::Jmp(jmp.try_into()?))
        } else {
            Err(format!("Unrecognized instruction: '{value}'!").into())
        }
    }
}

struct Handheld {
    ip: usize,
    code: Vec<Instruction>,
    acc: isize,
}

impl TryFrom<&str> for Handheld {
    type Error = TryFromError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut code = Vec::new();
        for line in value.lines() {
            code.push(line.try_into()?);
        }
        Ok(Self {
            ip: 0,
            code,
            acc: 0,
        })
    }
}

impl Handheld {
    fn reset(&mut self) {
        self.acc = 0;
        self.ip = 0;
    }

    fn step(&mut self) {
        let new_ip = self.ip as isize
            + match self.code[self.ip] {
                Instruction::Nop(_) => 1,
                Instruction::Acc(op) => {
                    self.acc += op.value;
                    1
                }
                Instruction::Jmp(op) => op.value,
            };

        if new_ip < 0 || new_ip as usize > self.code.len() {
            panic!(
                "New instruction pointer ({}) is out of bounds (0-{})!",
                new_ip,
                self.code.len()
            );
        }

        self.ip = new_ip as usize;
    }

    fn run(&mut self) -> HaltReason {
        self.reset();

        let mut ips = HashSet::new();

        loop {
            let acc = self.acc;
            ips.insert(self.ip);
            self.step();
            if self.ip == self.code.len() {
                return HaltReason::NormalTermination {
                    final_acc: self.acc,
                };
            } else if ips.contains(&self.ip) {
                return HaltReason::InfiniteLoop {
                    acc_before_repeat: acc,
                };
            }
        }
    }
}

fn fix_corrupted_instruction(handheld: &mut Handheld) -> isize {
    let code = handheld.code.clone();

    for (idx, instruction) in code.iter().enumerate() {
        let original_instruction = handheld.code[idx];
        match instruction {
            Instruction::Acc(_) => continue,
            Instruction::Nop(op) => handheld.code[idx] = Instruction::Jmp(*op),
            Instruction::Jmp(op) => handheld.code[idx] = Instruction::Nop(*op),
        }
        handheld.reset();
        let result = handheld.run();

        match result {
            HaltReason::NormalTermination { final_acc } => return final_acc,
            HaltReason::InfiniteLoop {
                acc_before_repeat: _,
            } => handheld.code[idx] = original_instruction,
        }
    }

    panic!("No solution found!");
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum HaltReason {
    NormalTermination { final_acc: isize },
    InfiniteLoop { acc_before_repeat: isize },
}

#[cfg(test)]
mod tests {
    use crate::{fix_corrupted_instruction, HaltReason, Handheld, TryFromError};

    #[test]
    fn test_examples() -> Result<(), TryFromError> {
        let code = "\
            nop +0\n\
            acc +1\n\
            jmp +4\n\
            acc +3\n\
            jmp -3\n\
            acc -99\n\
            acc +1\n\
            jmp -4\n\
            acc +6\
        ";
        let mut handheld = Handheld::try_from(code)?;

        let result = handheld.run();
        assert_eq!(
            result,
            HaltReason::InfiniteLoop {
                acc_before_repeat: 5
            }
        );

        let acc = fix_corrupted_instruction(&mut handheld);
        assert_eq!(acc, 8);

        Ok(())
    }

    #[test]
    fn test_input() -> Result<(), TryFromError> {
        let code = std::fs::read_to_string("input/code.txt").unwrap();
        let mut handheld = Handheld::try_from(code.as_str())?;

        let result = handheld.run();
        assert_eq!(
            result,
            HaltReason::InfiniteLoop {
                acc_before_repeat: 2058
            }
        );

        let acc = fix_corrupted_instruction(&mut handheld);
        assert_eq!(acc, 1000);

        Ok(())
    }
}
