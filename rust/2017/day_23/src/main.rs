#![allow(dead_code)]

use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::Regex;
use strum::EnumCount;

fn main() {
    println!("Advent of Code 2017 - day 23");

    let instructions = std::fs::read_to_string("input/instructions.txt").unwrap();
    let mut machine = Machine::from(instructions.as_str());
    machine.eval(false);

    assert_eq!(machine.registers[usize::from(Register::H)], 16);
}

lazy_static! {
    static ref RE_INSTRUCTION: Regex = Regex::new(r"^(\w+) ([^\s]+) ([^\s]+)$").unwrap();
}

#[derive(Debug, Clone, Copy, PartialEq, EnumCount)]
enum Register {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

impl From<char> for Register {
    fn from(value: char) -> Self {
        match value {
            'a' => Self::A,
            'b' => Self::B,
            'c' => Self::C,
            'd' => Self::D,
            'e' => Self::E,
            'f' => Self::F,
            'g' => Self::G,
            'h' => Self::H,
            _ => panic!("Illegal register: {value}!"),
        }
    }
}

impl From<&str> for Register {
    fn from(value: &str) -> Self {
        if value.len() == 1 {
            value.chars().next().unwrap().into()
        } else {
            panic!("Illegal register: {value}!")
        }
    }
}

impl From<Register> for usize {
    fn from(value: Register) -> Self {
        match value {
            Register::A => 0,
            Register::B => 1,
            Register::C => 2,
            Register::D => 3,
            Register::E => 4,
            Register::F => 5,
            Register::G => 6,
            Register::H => 7,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Value(i64);

#[derive(Debug, Clone, Copy, PartialEq)]
enum Operand {
    Register(Register),
    Value(Value),
}

impl From<&str> for Operand {
    fn from(value: &str) -> Self {
        if let Ok(value) = value.parse::<i64>() {
            Self::Value(Value(value))
        } else {
            Self::Register(value.into())
        }
    }
}

#[derive(Debug)]
enum Instruction {
    Set(Register, Operand),
    Sub(Register, Operand),
    Mul(Register, Operand),
    Mod(Register, Operand),
    Jnz(Operand, Operand),
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        let caps = RE_INSTRUCTION.captures(value).unwrap();

        match &caps[1] {
            "set" => Instruction::Set(caps[2].into(), caps[3].into()),
            "sub" => Instruction::Sub(caps[2].into(), caps[3].into()),
            "mul" => Instruction::Mul(caps[2].into(), caps[3].into()),
            "mod" => Instruction::Mod(caps[2].into(), caps[3].into()),
            "jnz" => Instruction::Jnz(caps[2].into(), caps[3].into()),
            _ => panic!("Illegal instruction: {value}!"),
        }
    }
}

#[derive(Debug)]
struct Machine {
    instructions: Vec<Instruction>,
    program_counter: usize,
    registers: Vec<i64>,
    instruction_count: HashMap<&'static str, usize>,
}

impl From<&str> for Machine {
    fn from(value: &str) -> Self {
        let instructions = value
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .filter(|line| !line.starts_with("#"))
            .map(|line| line.into())
            .collect();

        Self {
            instructions,
            program_counter: 0,
            registers: Vec::with_capacity(0),
            instruction_count: HashMap::new(),
        }
    }
}

impl Machine {
    fn reset(&mut self, debug_mode: bool) {
        self.registers = vec![0; Register::COUNT];

        if !debug_mode {
            self.registers[usize::from(Register::A)] = 1;
        }

        self.program_counter = 0;
        self.instruction_count = HashMap::new();
    }

    fn inc_instruction_count(&mut self, instruction: &'static str) {
        let old_count = self
            .instruction_count
            .get(instruction)
            .or(Some(&0))
            .unwrap();
        self.instruction_count.insert(instruction, old_count + 1);
    }

    fn eval_register(&self, r: Register) -> i64 {
        let idx: usize = r.into();
        self.registers[idx]
    }

    fn eval_operand(&self, o: Operand) -> i64 {
        match o {
            Operand::Register(r) => self.eval_register(r),
            Operand::Value(v) => v.0,
        }
    }

    fn eval_set(&mut self, x: Register, y: Operand) -> isize {
        self.inc_instruction_count("set");
        let idx: usize = x.into();
        self.registers[idx] = self.eval_operand(y);
        1
    }

    fn eval_sub(&mut self, x: Register, y: Operand) -> isize {
        self.inc_instruction_count("sub");
        let idx: usize = x.into();
        self.registers[idx] = self.eval_register(x) - self.eval_operand(y);

        if x == Register::H {
            println!("#################");
            let c = 'a';
            for i in 0..Register::COUNT {
                let c: char = ((c as u32) + (i as u32)).try_into().unwrap();
                println!(
                    "{} => {}",
                    c,
                    self.registers[usize::from(Register::from(c))]
                );
            }
        }

        1
    }

    fn eval_mul(&mut self, x: Register, y: Operand) -> isize {
        self.inc_instruction_count("mul");
        let idx: usize = x.into();
        self.registers[idx] = self.eval_register(x) * self.eval_operand(y);
        1
    }

    fn eval_mod(&mut self, x: Register, y: Operand) -> isize {
        self.inc_instruction_count("mul");
        let idx: usize = x.into();
        self.registers[idx] = self.eval_register(x) % self.eval_operand(y);
        1
    }

    fn eval_jnz(&mut self, x: Operand, y: Operand) -> isize {
        self.inc_instruction_count("jnz");
        if self.eval_operand(x) != 0 {
            self.eval_operand(y) as isize
        } else {
            1
        }
    }

    fn eval(&mut self, debug_mode: bool) {
        self.reset(debug_mode);

        while self.program_counter < self.instructions.len() {
            let offset = match self.instructions[self.program_counter] {
                Instruction::Set(x, y) => self.eval_set(x, y),
                Instruction::Sub(x, y) => self.eval_sub(x, y),
                Instruction::Mul(x, y) => self.eval_mul(x, y),
                Instruction::Mod(x, y) => self.eval_mod(x, y),
                Instruction::Jnz(x, y) => self.eval_jnz(x, y),
            };
            let mut pc = self.program_counter as isize + offset;
            if pc < 0 {
                pc = self.instructions.len() as isize;
            }
            self.program_counter = pc as usize;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Machine, Register};

    #[test]
    fn test_examples() {
        let instructions = "\
            set a 1234\n\
            set b 117\n\
            set c 2\n\
            mul b c\n\
            mul c c\n\
            mul c c\n\
            sub a b\n\
            jnz h -5\
        ";
        let mut machine = Machine::from(instructions);
        machine.eval(true);

        assert_eq!(machine.registers[usize::from(Register::A)], 1000);
        assert_eq!(machine.registers[usize::from(Register::B)], 234);
        assert_eq!(machine.registers[usize::from(Register::C)], 16);

        assert_eq!(machine.instruction_count.get("mul"), Some(&3));

        machine.eval(false);

        assert_eq!(machine.registers[usize::from(Register::A)], 1000);
        assert_eq!(machine.registers[usize::from(Register::B)], 234);
        assert_eq!(machine.registers[usize::from(Register::C)], 16);

        assert_eq!(machine.instruction_count.get("mul"), Some(&3));
    }

    #[test]
    fn test_input() {
        let instructions = std::fs::read_to_string("input/instructions.txt").unwrap();
        let mut machine = Machine::from(instructions.as_str());
        machine.eval(true);

        assert_eq!(machine.instruction_count.get("mul"), Some(&5929));
    }

    #[test]
    fn test_input_part2() {
        // instructions_part2.txt is "optimized" enough to be completed
        // in ~8s on a laptop/mobile dual-core Core i7 from 2016 when
        // run a optimized release build. Good enough.
        let instructions = std::fs::read_to_string("input/instructions_part2.txt").unwrap();
        let mut machine = Machine::from(instructions.as_str());
        machine.eval(false);

        assert_eq!(machine.registers[usize::from(Register::H)], 907);
    }
}
