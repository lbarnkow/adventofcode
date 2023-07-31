#![allow(dead_code)]

use std::num::ParseIntError;

use thiserror::Error;

fn main() {
    println!("Advent of Code 2016 - day 12");
}

#[derive(Debug, Error)]
#[error("Not a valid register: {0}")]
struct RegisterParseError(String);

#[derive(Debug, Clone, Copy)]
struct Register(usize);

impl TryFrom<&str> for Register {
    type Error = RegisterParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() != 1 {
            return Err(RegisterParseError(value.to_owned()));
        }

        match value.chars().next().expect("Should not be empty!") {
            'a' => Ok(Self(0)),
            'b' => Ok(Self(1)),
            'c' => Ok(Self(2)),
            'd' => Ok(Self(3)),
            _ => Err(RegisterParseError(value.to_owned())),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Value(i64);

impl TryFrom<&str> for Value {
    type Error = ParseIntError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = i64::from_str_radix(value, 10)?;
        Ok(Self(value))
    }
}

#[derive(Debug, Clone, Copy)]
enum Operand {
    Register(Register),
    Value(Value),
}

impl From<&str> for Operand {
    fn from(value: &str) -> Self {
        if let Ok(value) = Value::try_from(value) {
            Self::Value(value)
        } else if let Ok(register) = Register::try_from(value) {
            Self::Register(register)
        } else {
            panic!("Neither an integer value nor a register: {}", value)
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Cpy(Operand, Register),
    Inc(Register),
    Dec(Register),
    Jnz(Operand, Value),
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        let split = value.split(' ').collect::<Vec<&str>>();

        if split.is_empty() {
            panic!("Empty instruction!");
        }

        match split[0] {
            "cpy" => Self::Cpy(
                Operand::from(split[1]),
                Register::try_from(split[2]).expect(&format!("Not a valid register: {}", split[2])),
            ),
            "inc" => Self::Inc(
                Register::try_from(split[1]).expect(&format!("Not a valid register: {}", split[1])),
            ),
            "dec" => Self::Dec(
                Register::try_from(split[1]).expect(&format!("Not a valid register: {}", split[1])),
            ),
            "jnz" => Self::Jnz(
                Operand::from(split[1]),
                Value::try_from(split[2]).expect(&format!("Not a valid value: {}", split[2])),
            ),
            _ => panic!("Illegal instruction '{}'", split[0]),
        }
    }
}

impl Instruction {
    fn from_lines(s: &str) -> Vec<Self> {
        s.lines().map(|line| Instruction::from(line)).collect()
    }
}

#[derive(Debug, PartialEq)]
enum State {
    Initialized,
    Terminated,
}

#[derive(Debug)]
struct Machine {
    registers: [i64; 4],
    instructions: Vec<Instruction>,
    program_counter: usize,
    state: State,
}

impl Machine {
    fn new(instructions: Vec<Instruction>) -> Self {
        Self {
            registers: [0; 4],
            instructions,
            program_counter: 0,
            state: State::Initialized,
        }
    }

    fn update_pc(&mut self, offset: isize) {
        let pc: isize = self
            .program_counter
            .try_into()
            .expect("Program counter should fit into isize!");
        let pc: usize = (pc + offset)
            .try_into()
            .expect("Updated program counter should fit into usize");
        self.program_counter = pc;
    }

    fn do_cpy(&mut self, op: Operand, dst_r: Register) -> isize {
        self.registers[dst_r.0] = match op {
            Operand::Register(src_r) => self.registers[src_r.0],
            Operand::Value(v) => v.0,
        };
        1
    }

    fn do_inc(&mut self, r: Register) -> isize {
        self.registers[r.0] += 1;
        1
    }

    fn do_dec(&mut self, r: Register) -> isize {
        self.registers[r.0] -= 1;
        1
    }

    fn do_jnz(&mut self, op: Operand, offset: Value) -> isize {
        let v = match op {
            Operand::Register(r) => self.registers[r.0],
            Operand::Value(v) => v.0,
        };

        if v != 0 {
            offset
                .0
                .try_into()
                .expect("Value for jnz should fit into isize!")
        } else {
            1
        }
    }

    fn run(&mut self) {
        while self.program_counter < self.instructions.len() {
            let pc_offset = match self.instructions[self.program_counter] {
                Instruction::Cpy(op, r) => self.do_cpy(op, r),
                Instruction::Inc(r) => self.do_inc(r),
                Instruction::Dec(r) => self.do_dec(r),
                Instruction::Jnz(op, v) => self.do_jnz(op, v),
            };

            self.update_pc(pc_offset);
        }
        self.state = State::Terminated
    }
}

#[cfg(test)]
mod tests {
    use crate::{Instruction, Machine, State};

    #[test]
    fn test_example() {
        let instructions = "\
            cpy 41 a\n\
            inc a\n\
            inc a\n\
            dec a\n\
            jnz a 2\n\
            dec a\
        ";

        let instructions = Instruction::from_lines(instructions);
        assert_eq!(instructions.len(), 6);

        let mut machine = Machine::new(instructions);
        machine.run();
        assert_eq!(machine.state, State::Terminated);
        assert_eq!(machine.registers[0], 42);
    }

    #[test]
    fn test_input() {
        let instructions = std::fs::read_to_string("input/instructions.txt").unwrap();

        let instructions = Instruction::from_lines(&instructions);
        assert_eq!(instructions.len(), 23);

        let mut machine = Machine::new(instructions);
        machine.run();
        assert_eq!(machine.state, State::Terminated);
        println!("{:?}", machine.registers);
        assert_eq!(machine.registers[0], 318083);
    }

    #[test]
    fn test_input_part2() {
        let mut instructions = std::fs::read_to_string("input/instructions.txt").unwrap();
        instructions.insert_str(0, "cpy 1 c\n");

        let instructions = Instruction::from_lines(&instructions);
        assert_eq!(instructions.len(), 24);

        let mut machine = Machine::new(instructions);
        machine.run();
        assert_eq!(machine.state, State::Terminated);
        println!("{:?}", machine.registers);
        assert_eq!(machine.registers[0], 9227737);
    }
}
