#![allow(dead_code)]

use lazy_static::lazy_static;
use regex::Regex;
use std::fmt::Display;

fn main() {
    println!("Advent of Code 2018 - day 21");
}

lazy_static! {
    static ref RE_IP: Regex = Regex::new(r"^#ip (\d+)$").unwrap();
    static ref RE_INSTRUCTION: Regex = Regex::new(r"^(\w+) (-?\d+) (-?\d+) (-?\d+)$").unwrap();
}

const NUM_REGISTERS: usize = 6;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Register(usize);

impl From<i64> for Register {
    fn from(value: i64) -> Self {
        let r: usize = value
            .try_into()
            .unwrap_or_else(|_| panic!("Register {value} can't be converted to usize!"));
        if r >= NUM_REGISTERS {
            panic!("Register {value} is not a legal register!");
        }
        Self(r)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Value(i64);

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum OpCode {
    Addr(Register, Register, Register),
    Addi(Register, Value, Register),
    Mulr(Register, Register, Register),
    Muli(Register, Value, Register),
    Banr(Register, Register, Register),
    Bani(Register, Value, Register),
    Borr(Register, Register, Register),
    Bori(Register, Value, Register),
    Setr(Register, Value, Register),
    Seti(Value, Value, Register),
    Gtir(Value, Register, Register),
    Gtri(Register, Value, Register),
    Gtrr(Register, Register, Register),
    Eqir(Value, Register, Register),
    Eqri(Register, Value, Register),
    Eqrr(Register, Register, Register),
}

impl From<&str> for OpCode {
    fn from(value: &str) -> Self {
        let caps = RE_INSTRUCTION.captures(value).unwrap();

        let a = caps[2].parse::<i64>().unwrap();
        let b = caps[3].parse::<i64>().unwrap();
        let c = caps[4].parse::<i64>().unwrap();

        match &caps[1] {
            "addr" => Self::Addr(a.into(), b.into(), c.into()),
            "addi" => Self::Addi(a.into(), b.into(), c.into()),
            "mulr" => Self::Mulr(a.into(), b.into(), c.into()),
            "muli" => Self::Muli(a.into(), b.into(), c.into()),
            "banr" => Self::Banr(a.into(), b.into(), c.into()),
            "bani" => Self::Bani(a.into(), b.into(), c.into()),
            "borr" => Self::Borr(a.into(), b.into(), c.into()),
            "bori" => Self::Bori(a.into(), b.into(), c.into()),
            "setr" => Self::Setr(a.into(), b.into(), c.into()),
            "seti" => Self::Seti(a.into(), b.into(), c.into()),
            "gtir" => Self::Gtir(a.into(), b.into(), c.into()),
            "gtri" => Self::Gtri(a.into(), b.into(), c.into()),
            "gtrr" => Self::Gtrr(a.into(), b.into(), c.into()),
            "eqir" => Self::Eqir(a.into(), b.into(), c.into()),
            "eqri" => Self::Eqri(a.into(), b.into(), c.into()),
            "eqrr" => Self::Eqrr(a.into(), b.into(), c.into()),
            _ => panic!("Illegal instruction: {value}!"),
        }
    }
}

impl Display for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OpCode::Addr(a, b, c) => write!(f, "addr {} {} {}", a.0, b.0, c.0),
            OpCode::Addi(a, b, c) => write!(f, "addi {} {} {}", a.0, b.0, c.0),
            OpCode::Mulr(a, b, c) => write!(f, "mulr {} {} {}", a.0, b.0, c.0),
            OpCode::Muli(a, b, c) => write!(f, "muli {} {} {}", a.0, b.0, c.0),
            OpCode::Banr(a, b, c) => write!(f, "banr {} {} {}", a.0, b.0, c.0),
            OpCode::Bani(a, b, c) => write!(f, "bani {} {} {}", a.0, b.0, c.0),
            OpCode::Borr(a, b, c) => write!(f, "borr {} {} {}", a.0, b.0, c.0),
            OpCode::Bori(a, b, c) => write!(f, "bori {} {} {}", a.0, b.0, c.0),
            OpCode::Setr(a, b, c) => write!(f, "setr {} {} {}", a.0, b.0, c.0),
            OpCode::Seti(a, b, c) => write!(f, "seti {} {} {}", a.0, b.0, c.0),
            OpCode::Gtir(a, b, c) => write!(f, "gtir {} {} {}", a.0, b.0, c.0),
            OpCode::Gtri(a, b, c) => write!(f, "gtri {} {} {}", a.0, b.0, c.0),
            OpCode::Gtrr(a, b, c) => write!(f, "gtrr {} {} {}", a.0, b.0, c.0),
            OpCode::Eqir(a, b, c) => write!(f, "eqir {} {} {}", a.0, b.0, c.0),
            OpCode::Eqri(a, b, c) => write!(f, "eqri {} {} {}", a.0, b.0, c.0),
            OpCode::Eqrr(a, b, c) => write!(f, "eqrr {} {} {}", a.0, b.0, c.0),
        }
    }
}

impl OpCode {
    fn eval(&self, r: &mut [i64; NUM_REGISTERS]) {
        match self {
            OpCode::Addr(a, b, c) => r[c.0] = r[a.0] + r[b.0],
            OpCode::Addi(a, b, c) => r[c.0] = r[a.0] + b.0,
            OpCode::Mulr(a, b, c) => r[c.0] = r[a.0] * r[b.0],
            OpCode::Muli(a, b, c) => r[c.0] = r[a.0] * b.0,
            OpCode::Banr(a, b, c) => r[c.0] = r[a.0] & r[b.0],
            OpCode::Bani(a, b, c) => r[c.0] = r[a.0] & b.0,
            OpCode::Borr(a, b, c) => r[c.0] = r[a.0] | r[b.0],
            OpCode::Bori(a, b, c) => r[c.0] = r[a.0] | b.0,
            OpCode::Setr(a, _, c) => r[c.0] = r[a.0],
            OpCode::Seti(a, _, c) => r[c.0] = a.0,
            OpCode::Gtir(a, b, c) => r[c.0] = if a.0 > r[b.0] { 1 } else { 0 },
            OpCode::Gtri(a, b, c) => r[c.0] = if r[a.0] > b.0 { 1 } else { 0 },
            OpCode::Gtrr(a, b, c) => r[c.0] = if r[a.0] > r[b.0] { 1 } else { 0 },
            OpCode::Eqir(a, b, c) => r[c.0] = if a.0 == r[b.0] { 1 } else { 0 },
            OpCode::Eqri(a, b, c) => r[c.0] = if r[a.0] == b.0 { 1 } else { 0 },
            OpCode::Eqrr(a, b, c) => r[c.0] = if r[a.0] == r[b.0] { 1 } else { 0 },
        }
    }
}

#[derive(Debug)]
struct Computer {
    ip: usize,
    registers: [i64; NUM_REGISTERS],
    instructions: Vec<OpCode>,
}

impl From<&str> for Computer {
    fn from(value: &str) -> Self {
        let mut lines = value.lines();

        let ip_cap = RE_IP.captures(lines.next().unwrap()).unwrap();
        let ip = ip_cap[1].parse::<usize>().unwrap();
        let instructions = lines.map(OpCode::from).collect();
        let registers = [0; NUM_REGISTERS];

        Self {
            ip,
            registers,
            instructions,
        }
    }
}

impl Computer {
    fn reset(&mut self, r0: i64) {
        self.registers = [0; NUM_REGISTERS];
        self.registers[0] = r0;
    }

    fn eval<F>(&mut self, mut watch: F) -> usize
    where
        F: FnMut(&[i64; NUM_REGISTERS]) -> bool,
    {
        let mut i = 0;
        loop {
            let ip = self.registers[self.ip] as usize;
            let op = self.instructions[ip];

            if watch(&self.registers) {
                break;
            }

            op.eval(&mut self.registers);
            i += 1;

            self.registers[self.ip] += 1;

            let ip = self.registers[self.ip];
            if ip < 0 || ip >= self.instructions.len() as i64 {
                break;
            }
        }
        i
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::{Computer, NUM_REGISTERS};

    #[test]
    fn test_input() {
        let program = std::fs::read_to_string("input/program.txt").unwrap();
        let mut com = Computer::from(program.as_str());

        com.eval(|regs| regs[5] == 28);

        let r0 = com.registers[1];
        com.reset(r0);
        let instructions = com.eval(|_| false);

        assert_eq!(r0, 2159153);
        assert_eq!(instructions, 1848);
    }

    #[test]
    fn test_input_2() {
        let program = std::fs::read_to_string("input/program.txt").unwrap();
        let mut com = Computer::from(program.as_str());

        com.reset(0);

        let mut r1_vals: HashSet<i64> = HashSet::new();
        let mut r1_prev = i64::MIN;
        let f = |regs: &[i64; NUM_REGISTERS]| {
            if regs[5] != 28 {
                return false;
            }
            if r1_vals.contains(&regs[1]) {
                return true;
            }
            r1_vals.insert(regs[1]);
            r1_prev = regs[1];
            false
        };

        com.eval(f);

        assert_eq!(r1_prev, 7494885)
    }
}
