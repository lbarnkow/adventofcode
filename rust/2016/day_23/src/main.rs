#![allow(dead_code)]

use std::{fmt::Display, num::ParseIntError};

fn main() {
    println!("Advent of Code 2016 - day 23");
}

#[derive(Debug, Clone, Copy)]
struct Register(usize);

impl From<&str> for Register {
    fn from(value: &str) -> Self {
        if value.len() != 1 {
            panic!("Register name too long!");
        }

        match value.chars().next().expect("Should not be empty!") {
            'a' => Self(0),
            'b' => Self(1),
            'c' => Self(2),
            'd' => Self(3),
            _ => panic!("Illegal register name: {value}!"),
        }
    }
}

impl Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self.0 {
                0 => 'a',
                1 => 'b',
                2 => 'c',
                3 => 'd',
                _ => panic!("Illegal Register"),
            },
        )
    }
}

#[derive(Debug, Clone, Copy)]
struct Value(i64);

impl TryFrom<&str> for Value {
    type Error = ParseIntError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Self(i64::from_str_radix(value, 10)?))
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
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
        } else {
            Self::Register(Register::from(value))
        }
    }
}

impl Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operand::Register(r) => r.fmt(f),
            Operand::Value(v) => v.fmt(f),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Cpy(Operand, Operand),
    Inc(Operand),
    Dec(Operand),
    Jnz(Operand, Operand),
    Tgl(Operand),
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        let split = value.split(' ').collect::<Vec<&str>>();

        if split.is_empty() {
            panic!("Empty instruction!");
        }

        match split[0] {
            "cpy" => Self::Cpy(Operand::from(split[1]), Operand::from(split[2])),
            "inc" => Self::Inc(Operand::Register(Register::from(split[1]))),
            "dec" => Self::Dec(Operand::Register(Register::from(split[1]))),
            "jnz" => Self::Jnz(Operand::from(split[1]), Operand::from(split[2])),
            "tgl" => Self::Tgl(Operand::from(split[1])),
            _ => panic!("Illegal instruction '{}'", split[0]),
        }
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Cpy(a, b) => write!(f, "cpy {} {}", a, b),
            Instruction::Inc(a) => write!(f, "inc {}", a),
            Instruction::Dec(a) => write!(f, "dec {}", a),
            Instruction::Jnz(a, b) => write!(f, "jnz {} {}", a, b),
            Instruction::Tgl(a) => write!(f, "tgl {}", a),
        }
    }
}

impl Instruction {
    fn from_lines(s: &str) -> Vec<Self> {
        s.lines()
            .filter(|line| !line.trim().is_empty() && !line.starts_with('#'))
            .map(|line| Instruction::from(line))
            .collect()
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

    fn do_cpy(&mut self, op: Operand, dst_r: Operand) -> isize {
        if let Operand::Register(dst_r) = dst_r {
            self.registers[dst_r.0] = match op {
                Operand::Register(src_r) => self.registers[src_r.0],
                Operand::Value(v) => v.0,
            };
        } else {
            // ignore
        }

        1
    }

    fn do_inc(&mut self, op: Operand) -> isize {
        if let Operand::Register(r) = op {
            self.registers[r.0] += 1;
        }
        1
    }

    fn do_dec(&mut self, op: Operand) -> isize {
        if let Operand::Register(r) = op {
            self.registers[r.0] -= 1;
        }
        1
    }

    fn do_jnz(&mut self, op: Operand, offset: Operand) -> isize {
        let v = match op {
            Operand::Register(r) => self.registers[r.0],
            Operand::Value(v) => v.0,
        };

        if v != 0 {
            match offset {
                Operand::Register(r) => self.registers[r.0].try_into().unwrap(),
                Operand::Value(v) => v.0.try_into().unwrap(),
            }
        } else {
            1
        }
    }

    fn do_tgl(&mut self, op: Operand) -> isize {
        let offset = match op {
            Operand::Register(r) => self.registers[r.0],
            Operand::Value(v) => v.0,
        };

        let pc: i64 = self
            .program_counter
            .try_into()
            .expect("Program counter should fit into i64!");
        let pc = pc + offset;

        let program_len: i64 = self
            .instructions
            .len()
            .try_into()
            .expect("Program length should fit into i64!");

        if pc >= 0 && pc < program_len {
            let pc: usize = pc
                .try_into()
                .expect("Offset program counter should fit into usize!");

            let toggled_instruction = match self.instructions[pc] {
                Instruction::Inc(r) => Instruction::Dec(r),
                Instruction::Dec(r) => Instruction::Inc(r),
                Instruction::Tgl(op) => Instruction::Inc(op),
                Instruction::Jnz(op, offset) => Instruction::Cpy(op, offset),
                Instruction::Cpy(op, dst_r) => Instruction::Jnz(op, dst_r),
            };
            self.instructions[pc] = toggled_instruction;
        }
        1
    }

    fn run(&mut self) {
        while self.program_counter < self.instructions.len() {
            let pc_offset = match self.instructions[self.program_counter] {
                Instruction::Cpy(op, r) => self.do_cpy(op, r),
                Instruction::Inc(op) => self.do_inc(op),
                Instruction::Dec(op) => self.do_dec(op),
                Instruction::Jnz(op, v) => self.do_jnz(op, v),
                Instruction::Tgl(op) => self.do_tgl(op),
            };

            self.update_pc(pc_offset);
        }
        self.state = State::Terminated
    }
}

#[cfg(test)]
mod tests {
    use crate::{Instruction, Machine, Register, State};

    #[test]
    fn test_example() {
        let instructions = "\
            cpy 2 a\n\
            tgl a\n\
            tgl a\n\
            tgl a\n\
            cpy 1 a\n\
            dec a\n\
            dec a\
        ";

        let instructions = Instruction::from_lines(instructions);
        assert_eq!(instructions.len(), 7);

        let mut machine = Machine::new(instructions);
        machine.run();
        assert_eq!(machine.state, State::Terminated);
        assert_eq!(machine.registers[0], 3);
    }

    #[test]
    fn test_input() {
        let instructions = std::fs::read_to_string("input/instructions.txt").unwrap();

        let instructions = Instruction::from_lines(&instructions);
        assert_eq!(instructions.len(), 26);

        let mut machine = Machine::new(instructions);

        machine.registers[Register::from("a").0] = 7;
        machine.run();
        assert_eq!(machine.state, State::Terminated);
        assert_eq!(machine.registers[0], 11130);
    }

    #[test]
    fn test_input_part_2() {
        let instructions = std::fs::read_to_string("input/instructions.txt").unwrap();

        let instructions = Instruction::from_lines(&instructions);
        assert_eq!(instructions.len(), 26);

        let mut machine = Machine::new(instructions);

        // didn't care about optimizing the assembunny
        // takes ~8s on a M1 Pro, good enough to not worry about it.
        machine.registers[Register::from("a").0] = 12;
        machine.run();
        assert_eq!(machine.state, State::Terminated);
        assert_eq!(machine.registers[0], 479007690);
    }
}
