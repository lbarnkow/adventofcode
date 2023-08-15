#![allow(dead_code)]

use lazy_static::lazy_static;
use regex::Regex;
use std::{
    collections::HashMap,
    sync::{
        mpsc::{Receiver, Sender},
        Arc, Mutex,
    },
    time::Duration,
};

fn main() {
    println!("Advent of Code 2017 - day 18");
}

lazy_static! {
    static ref RE_UNARY: Regex = Regex::new(r"^(\w+) ([\-\w\d]+)$").unwrap();
    static ref RE_BINARY: Regex = Regex::new(r"^(\w+) ([\-\w\d]+) ([\-\w\d]+)$").unwrap();
}

type Register = char;
type Value = i64;

#[derive(Debug, Clone, Copy)]
enum Operand {
    Register(Register),
    Value(Value),
}

impl From<&str> for Operand {
    fn from(value: &str) -> Self {
        if let Some(r) = try_parse_register(value) {
            Self::Register(r)
        } else if let Ok(v) = value.parse::<Value>() {
            Self::Value(v)
        } else {
            panic!("Not a legal operand: {value}!");
        }
    }
}

fn try_parse_register(s: &str) -> Option<Register> {
    if s.len() == 1 {
        if let Some(c) = s.chars().next() {
            if c.is_alphabetic() {
                return Some(c);
            }
        }
    }

    None
}

fn parse_register(s: &str) -> Register {
    if let Some(r) = try_parse_register(s) {
        return r;
    }
    panic!("Not a legal register: {s}!");
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Snd(Operand),
    Set(Register, Operand),
    Add(Register, Operand),
    Mul(Register, Operand),
    Mod(Register, Operand),
    Rcv(Register),
    Jgz(Operand, Operand),
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        match &value[..3] {
            "snd" => {
                if let Some(caps) = RE_UNARY.captures(value) {
                    return Self::Snd(caps[2].into());
                }
            }
            "set" => {
                if let Some(caps) = RE_BINARY.captures(value) {
                    return Self::Set(parse_register(&caps[2]), caps[3].into());
                }
            }
            "add" => {
                if let Some(caps) = RE_BINARY.captures(value) {
                    return Self::Add(parse_register(&caps[2]), caps[3].into());
                }
            }
            "mul" => {
                if let Some(caps) = RE_BINARY.captures(value) {
                    return Self::Mul(parse_register(&caps[2]), caps[3].into());
                }
            }
            "mod" => {
                if let Some(caps) = RE_BINARY.captures(value) {
                    return Self::Mod(parse_register(&caps[2]), caps[3].into());
                }
            }
            "rcv" => {
                if let Some(caps) = RE_UNARY.captures(value) {
                    return Self::Rcv(parse_register(&caps[2]));
                }
            }
            "jgz" => {
                if let Some(caps) = RE_BINARY.captures(value) {
                    return Self::Jgz(caps[2].into(), caps[3].into());
                }
            }
            _ => (),
        }

        panic!("Illegal instruction: {value}!");
    }
}

trait BaseMachine {
    fn eval_operand(&self, operand: Operand) -> Value;
    fn do_get(&self, r: Register) -> Value;

    fn do_set(&mut self, x: Register, y: Operand) -> isize;
    fn do_snd(&mut self, x: Operand) -> isize;
    fn do_rcv(&mut self, x: Register) -> isize;

    fn do_add(&mut self, x: Register, y: Operand) -> isize {
        let mut v = self.do_get(x);
        v += self.eval_operand(y);
        self.do_set(x, Operand::Value(v));
        1
    }

    fn do_mul(&mut self, x: Register, y: Operand) -> isize {
        let mut v = self.do_get(x);
        v *= self.eval_operand(y);
        self.do_set(x, Operand::Value(v));
        1
    }

    fn do_mod(&mut self, x: Register, y: Operand) -> isize {
        let mut v = self.do_get(x);
        v %= self.eval_operand(y);
        self.do_set(x, Operand::Value(v));
        1
    }

    fn do_jgz(&self, x: Operand, y: Operand) -> isize {
        if self.eval_operand(x) > 0 {
            self.eval_operand(y).try_into().unwrap()
        } else {
            1
        }
    }

    fn current_instruction(&self) -> Instruction;

    fn instructions_count(&self) -> usize;
    fn get_program_counter(&self) -> usize;
    fn set_program_counter(&mut self, pc: usize);

    fn eval(&mut self) {
        loop {
            let instruction = self.current_instruction();
            let offset: isize = match instruction {
                Instruction::Snd(x) => self.do_snd(x),
                Instruction::Set(x, y) => self.do_set(x, y),
                Instruction::Add(x, y) => self.do_add(x, y),
                Instruction::Mul(x, y) => self.do_mul(x, y),
                Instruction::Mod(x, y) => self.do_mod(x, y),
                Instruction::Rcv(x) => self.do_rcv(x),
                Instruction::Jgz(x, y) => self.do_jgz(x, y),
            };

            let pc: isize = self.get_program_counter().try_into().unwrap();
            let pc = pc + offset;

            if pc < 0 || pc >= self.instructions_count().try_into().unwrap() {
                self.set_program_counter(usize::MAX);
                break;
            } else {
                self.set_program_counter(pc.try_into().unwrap());
            }
        }
    }
}

struct Machine {
    instructions: Vec<Instruction>,
    program_counter: usize,
    registers: HashMap<char, Value>,
    last_sound: Option<Value>,
}

impl From<&str> for Machine {
    fn from(value: &str) -> Self {
        let instructions = value.lines().map(|line| line.into()).collect();

        Self {
            instructions,
            program_counter: 0,
            registers: HashMap::new(),
            last_sound: None,
        }
    }
}

impl BaseMachine for Machine {
    fn eval_operand(&self, operand: Operand) -> Value {
        match operand {
            Operand::Register(r) => self.do_get(r),
            Operand::Value(v) => v,
        }
    }

    fn do_get(&self, r: Register) -> Value {
        if let Some(r) = self.registers.get(&r) {
            *r
        } else {
            0
        }
    }

    fn do_snd(&mut self, x: Operand) -> isize {
        self.last_sound = Some(self.eval_operand(x));
        1
    }

    fn do_set(&mut self, x: Register, y: Operand) -> isize {
        let v = self.eval_operand(y);
        self.registers.insert(x, v);
        1
    }

    fn do_rcv(&mut self, x: Register) -> isize {
        if self.do_get(x) != 0 {
            if self.last_sound.is_some() {
                return self.instructions.len().try_into().unwrap();
            }
        }
        1
    }

    fn current_instruction(&self) -> Instruction {
        self.instructions[self.program_counter]
    }

    fn instructions_count(&self) -> usize {
        self.instructions.len()
    }

    fn get_program_counter(&self) -> usize {
        self.program_counter
    }

    fn set_program_counter(&mut self, pc: usize) {
        self.program_counter = pc;
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum SendReceiveState {
    Working,
    Receiving,
}

struct SendReceiveSync {
    state: [SendReceiveState; 2],
    sent: [usize; 2],
    rcvd: [usize; 2],
    deadlocked: bool,
}

impl SendReceiveSync {
    fn new() -> Self {
        Self {
            state: [SendReceiveState::Working; 2],
            sent: [0; 2],
            rcvd: [0; 2],
            deadlocked: false,
        }
    }
}

struct SendReceiveMachine {
    program_id: usize,
    instructions: Vec<Instruction>,
    program_counter: usize,
    registers: HashMap<char, Value>,
    tx: Sender<Value>,
    rx: Receiver<Value>,
    sync: Arc<Mutex<SendReceiveSync>>,
}

impl SendReceiveMachine {
    fn from(
        program_id: usize,
        (tx, rx): (Sender<Value>, Receiver<Value>),
        sync: Arc<Mutex<SendReceiveSync>>,
        value: &str,
    ) -> Self {
        let instructions = value.lines().map(|line| line.into()).collect();

        let mut registers: HashMap<Register, i64> = HashMap::new();
        registers.insert('p', program_id.try_into().unwrap());

        Self {
            program_id,
            instructions,
            program_counter: 0,
            registers,
            tx,
            rx,
            sync,
        }
    }
}

impl BaseMachine for SendReceiveMachine {
    fn eval_operand(&self, operand: Operand) -> Value {
        match operand {
            Operand::Register(r) => self.do_get(r),
            Operand::Value(v) => v,
        }
    }

    fn do_get(&self, r: Register) -> Value {
        if let Some(r) = self.registers.get(&r) {
            *r
        } else {
            0
        }
    }

    fn do_snd(&mut self, x: Operand) -> isize {
        let mut lock = self.sync.lock().unwrap();
        lock.sent[self.program_id] += 1;
        drop(lock);

        self.tx.send(self.eval_operand(x)).unwrap();
        1
    }

    fn do_set(&mut self, x: Register, y: Operand) -> isize {
        let v = self.eval_operand(y);
        self.registers.insert(x, v);
        1
    }

    fn do_rcv(&mut self, x: Register) -> isize {
        let mut lock = self.sync.lock().unwrap();
        lock.state[self.program_id] = SendReceiveState::Receiving;
        drop(lock);

        let mut data = self.rx.recv_timeout(Duration::from_millis(1));
        while let Err(_) = data {
            let mut lock = self.sync.lock().unwrap();
            if lock.state[0] == SendReceiveState::Receiving
                && lock.state[1] == SendReceiveState::Receiving
                && lock.rcvd[0] == lock.sent[1]
                && lock.rcvd[1] == lock.sent[0]
            {
                lock.deadlocked = true;
                return self.instructions.len().try_into().unwrap();
            }
            drop(lock);

            data = self.rx.recv_timeout(Duration::from_millis(1));
        }

        self.do_set(x, Operand::Value(data.unwrap()));

        let mut lock = self.sync.lock().unwrap();
        lock.state[self.program_id] = SendReceiveState::Working;
        lock.rcvd[self.program_id] += 1;
        drop(lock);

        1
    }

    fn current_instruction(&self) -> Instruction {
        self.instructions[self.program_counter]
    }

    fn instructions_count(&self) -> usize {
        self.instructions.len()
    }

    fn get_program_counter(&self) -> usize {
        self.program_counter
    }

    fn set_program_counter(&mut self, pc: usize) {
        self.program_counter = pc;
    }
}

#[cfg(test)]
mod tests {
    use std::{
        sync::{mpsc::channel, Arc, Mutex},
        thread,
    };

    use crate::{BaseMachine, Machine, SendReceiveMachine, SendReceiveSync};

    #[test]
    fn test_examples() {
        let instructions = "\
            set a 1\n\
            add a 2\n\
            mul a a\n\
            mod a 5\n\
            snd a\n\
            set a 0\n\
            rcv a\n\
            jgz a -1\n\
            set a 1\n\
            jgz a -2\
        ";

        let mut machine = Machine::from(instructions);
        machine.eval();

        assert_eq!(machine.last_sound, Some(4));
    }

    #[test]
    fn test_examples_part2() {
        let instructions = "\
            snd 1\n\
            snd 2\n\
            snd p\n\
            rcv a\n\
            rcv b\n\
            rcv c\n\
            rcv d\
        ";

        let (tx0, rx0) = channel();
        let (tx1, rx1) = channel();

        let sync = Arc::new(Mutex::new(SendReceiveSync::new()));

        let mut machine0 = SendReceiveMachine::from(0, (tx0, rx1), sync.clone(), instructions);
        let mut machine1 = SendReceiveMachine::from(1, (tx1, rx0), sync.clone(), instructions);

        let threads = [
            thread::spawn(move || machine0.eval()),
            thread::spawn(move || machine1.eval()),
        ];
        for t in threads {
            t.join().unwrap();
        }

        assert_eq!(sync.lock().unwrap().sent[1], 3);
    }

    #[test]
    fn test_input() {
        let instructions = std::fs::read_to_string("input/instructions.txt").unwrap();

        let mut machine = Machine::from(instructions.as_str());
        machine.eval();

        assert_eq!(machine.last_sound, Some(7071));
    }

    #[test]
    fn test_input_part2() {
        let instructions = std::fs::read_to_string("input/instructions.txt").unwrap();

        let (tx0, rx0) = channel();
        let (tx1, rx1) = channel();

        let sync = Arc::new(Mutex::new(SendReceiveSync::new()));

        let mut machine0 =
            SendReceiveMachine::from(0, (tx0, rx1), sync.clone(), instructions.as_str());
        let mut machine1 =
            SendReceiveMachine::from(1, (tx1, rx0), sync.clone(), instructions.as_str());

        let threads = [
            thread::spawn(move || machine0.eval()),
            thread::spawn(move || machine1.eval()),
        ];
        for t in threads {
            t.join().unwrap();
        }

        assert_eq!(sync.lock().unwrap().sent[1], 8001);
        assert_eq!(sync.lock().unwrap().deadlocked, true);
    }
}
