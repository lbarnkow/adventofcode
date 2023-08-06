#![allow(dead_code)]

use std::{fmt::Display, num::ParseIntError};

fn main() {
    println!("Advent of Code 2016 - day 25");
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
    Out(Operand),
    Mul(Operand, Operand),
    Add(Operand, Operand),
    Nop,
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
            "out" => Self::Out(Operand::from(split[1])),
            "mul" => Self::Mul(Operand::from(split[1]), Operand::from(split[2])),
            "add" => Self::Add(Operand::from(split[1]), Operand::from(split[2])),
            "nop" => Self::Nop,
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
            Instruction::Out(a) => write!(f, "out {}", a),
            Instruction::Mul(a, b) => write!(f, "mul {} {}", a, b),
            Instruction::Add(a, b) => write!(f, "add {} {}", a, b),
            Instruction::Nop => write!(f, "nop"),
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
struct Machine<T>
where
    T: Output,
{
    registers: [i64; 4],
    instructions: Vec<Instruction>,
    program_counter: usize,
    state: State,
    output: T,
}

impl<T> Machine<T>
where
    T: Output,
{
    fn new(instructions: Vec<Instruction>, output: T) -> Self {
        Self {
            registers: [0; 4],
            instructions,
            program_counter: 0,
            state: State::Initialized,
            output,
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

    fn do_out(&mut self, op: Operand) -> isize
    where
        T: Output,
    {
        let v = match op {
            Operand::Register(r) => self.registers[r.0],
            Operand::Value(v) => v.0,
        };
        if self.output.write(v) {
            1
        } else {
            1_000_000
        }
    }

    fn do_mul(&mut self, a: Operand, b: Operand) -> isize {
        if let Operand::Register(b) = b {
            let a = match a {
                Operand::Register(r) => self.registers[r.0],
                Operand::Value(v) => v.0,
            };
            self.registers[b.0] *= a;
        } else {
            panic!("The second operand to mul must be a register!");
        }

        1
    }

    fn do_add(&mut self, a: Operand, b: Operand) -> isize {
        if let Operand::Register(b) = b {
            let a = match a {
                Operand::Register(r) => self.registers[r.0],
                Operand::Value(v) => v.0,
            };
            self.registers[b.0] += a;
        } else {
            panic!("The second operand to mul must be a register!");
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
                Instruction::Out(op) => self.do_out(op),
                Instruction::Mul(a, b) => self.do_mul(a, b),
                Instruction::Add(a, b) => self.do_add(a, b),
                Instruction::Nop => 1,
            };

            self.update_pc(pc_offset);
        }
        self.state = State::Terminated
    }
}

trait Output: Clone {
    fn write(&mut self, value: i64) -> bool;
}

#[derive(Debug, Clone)]
struct AntennaTester {
    next: i64,
    is_ok: bool,
    stop_after: usize,
    output: String,
}

impl AntennaTester {
    fn new(stop_after: usize) -> Self {
        Self {
            next: 0,
            is_ok: true,
            stop_after,
            output: String::with_capacity(100),
        }
    }
}

impl Output for AntennaTester {
    fn write(&mut self, value: i64) -> bool {
        self.output.push_str(&format!("{}", value));

        if self.next != value {
            self.is_ok = false;
            return false;
        }

        if value == 0 {
            self.next = 1;
        } else {
            self.next = 0;
        }

        !(self.stop_after == self.output.len())
    }
}

#[cfg(test)]
mod tests {
    use crate::{AntennaTester, Instruction, Machine, Register, State};

    #[test]
    fn test_input() {
        let instructions = std::fs::read_to_string("input/instructions.txt").unwrap();

        let instructions = Instruction::from_lines(&instructions);
        assert_eq!(instructions.len(), 30);

        for i in 0..1_000 {
            let mut machine = Machine::new(instructions.clone(), AntennaTester::new(100));

            machine.registers[Register::from("a").0] = i;
            machine.run();
            assert_eq!(machine.state, State::Terminated);

            if machine.output.is_ok {
                assert_eq!(i, 189);
                return;
            }
        }

        panic!("failed")
    }
}
