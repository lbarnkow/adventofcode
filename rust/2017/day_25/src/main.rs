#![allow(dead_code)]

use std::{collections::HashMap, fmt::Display, str::Lines};

use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    println!("Advent of Code 2017 - day 25");
}

lazy_static! {
    static ref RE_BEGIN: Regex = Regex::new(r"^Begin in state (\w).$").unwrap();
    static ref RE_DIAGNOSTIC: Regex =
        Regex::new(r"^Perform a diagnostic checksum after (\d+) steps.$").unwrap();
    static ref RE_STATE_START: Regex = Regex::new(r"^In state (\w):$").unwrap();
    static ref RE_STATE_CURRENT: Regex = Regex::new(r"^  If the current value is (\d):$").unwrap();
    static ref RE_STATE_WRITE: Regex = Regex::new(r"^    - Write the value (\d).$").unwrap();
    static ref RE_STATE_MOVE: Regex = Regex::new(r"^    - Move one slot to the (\w+).$").unwrap();
    static ref RE_STATE_NEXT: Regex = Regex::new(r"^    - Continue with state (\w).$").unwrap();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Value {
    Zero,
    One,
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        match value {
            "0" => Self::Zero,
            "1" => Self::One,
            _ => panic!("Illegal value: {value}!"),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Value::Zero => '0',
                Value::One => '1',
            }
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
    Left,
    Right,
}

impl From<&str> for Dir {
    fn from(value: &str) -> Self {
        match value {
            "left" => Self::Left,
            "right" => Self::Right,
            _ => panic!("Illegal direction: {value}!"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Step {
    write: Value,
    dir: Dir,
    next: char,
}

impl Step {
    fn from(input: &mut Lines<'_>) -> Self {
        let write = input.next().unwrap();
        let dir = input.next().unwrap();
        let next = input.next().unwrap();

        let write = RE_STATE_WRITE.captures(write).unwrap();
        let dir = RE_STATE_MOVE.captures(dir).unwrap();
        let next = RE_STATE_NEXT.captures(next).unwrap();

        let write = write[1].into();
        let dir = dir[1].into();
        let next = next[1].chars().next().unwrap();

        Self { write, dir, next }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct State {
    name: char,
    on_zero: Step,
    on_one: Step,
}

impl State {
    fn from(input: &mut Lines<'_>) -> Self {
        let name = input.next().unwrap();
        let name = RE_STATE_START.captures(name).unwrap();
        let name = name[1].chars().next().unwrap();

        let current = input.next().unwrap();
        let current = RE_STATE_CURRENT.captures(current).unwrap();
        let current: Value = current[1].into();
        assert_eq!(current, Value::Zero);
        let on_zero: Step = Step::from(input);

        let current = input.next().unwrap();
        let current = RE_STATE_CURRENT.captures(current).unwrap();
        let current: Value = current[1].into();
        assert_eq!(current, Value::One);
        let on_one: Step = Step::from(input);

        Self {
            name,
            on_zero,
            on_one,
        }
    }
}

#[derive(Debug)]
struct Tape {
    buf_pos: Vec<Value>,
    buf_neg: Vec<Value>,
    pos: isize,
}

impl Display for Tape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for idx in (0..self.buf_neg.len()).rev() {
            if self.pos < 0 && (-1 - self.pos) as usize == idx {
                write!(f, "[{}]", self.buf_neg[idx]).unwrap();
            } else {
                write!(f, " {} ", self.buf_neg[idx]).unwrap();
            }
        }
        for idx in 0..self.buf_pos.len() {
            if self.pos >= 0 && self.pos as usize == idx {
                write!(f, "[{}]", self.buf_pos[idx]).unwrap();
            } else {
                write!(f, " {} ", self.buf_pos[idx]).unwrap();
            }
        }

        Ok(())
    }
}

impl Tape {
    fn new() -> Self {
        Self {
            buf_pos: vec![Value::Zero],
            buf_neg: Vec::new(),
            pos: 0,
        }
    }

    fn get(&self) -> Value {
        if self.pos >= 0 {
            let idx = self.pos as usize;
            self.buf_pos[idx]
        } else {
            let idx = (-1 - self.pos) as usize;
            self.buf_neg[idx]
        }
    }

    fn set(&mut self, v: Value) {
        if self.pos >= 0 {
            let idx = self.pos as usize;
            self.buf_pos[idx] = v;
        } else {
            let idx = (-1 - self.pos) as usize;
            self.buf_neg[idx] = v;
        }
    }

    fn move_to(&mut self, dir: Dir) {
        match dir {
            Dir::Left => self.pos -= 1,
            Dir::Right => self.pos += 1,
        }

        if self.pos >= 0 {
            let idx = self.pos as usize;
            if idx == self.buf_pos.len() {
                self.buf_pos.push(Value::Zero);
            }
        } else {
            let idx = (-1 - self.pos) as usize;
            if idx == self.buf_neg.len() {
                self.buf_neg.push(Value::Zero);
            }
        }
    }

    fn checksum(&self) -> usize {
        self.buf_pos.iter().filter(|e| **e == Value::One).count()
            + self.buf_neg.iter().filter(|e| **e == Value::One).count()
    }
}

#[derive(Debug)]
struct Machine {
    start_state: char,
    current_state: char,
    checksum_steps: usize,
    steps: usize,
    states: HashMap<char, State>,
    tape: Tape,
}

impl From<&str> for Machine {
    fn from(value: &str) -> Self {
        let mut input = value.lines();

        let start_state = input.next().unwrap();
        let start_state = RE_BEGIN.captures(start_state).unwrap();
        let start_state = start_state[1].chars().next().unwrap();

        let checksum_steps = input.next().unwrap();
        let checksum_steps = RE_DIAGNOSTIC.captures(checksum_steps).unwrap();
        let checksum_steps = checksum_steps[1].parse().unwrap();

        let mut states = HashMap::new();
        while let Some(_) = input.next() {
            let state = State::from(&mut input);
            states.insert(state.name, state);
        }

        assert!(states.contains_key(&start_state));
        for state in states.values() {
            assert!(states.contains_key(&state.on_zero.next));
            assert!(states.contains_key(&state.on_one.next));
        }

        Self {
            start_state,
            current_state: start_state,
            checksum_steps,
            steps: 0,
            states,
            tape: Tape::new(),
        }
    }
}

impl Machine {
    fn run_to_diagnostic(&mut self) -> usize {
        for _ in 0..self.checksum_steps {
            let state = self.states.get(&self.current_state).unwrap();
            let current_value = self.tape.get();

            let step = match current_value {
                Value::Zero => &state.on_zero,
                Value::One => &state.on_one,
            };

            self.tape.set(step.write);
            self.tape.move_to(step.dir);
            self.current_state = step.next;
        }

        self.tape.checksum()
    }
}

#[cfg(test)]
mod tests {
    use crate::Machine;

    #[test]
    fn test_examples() {
        let blueprint = std::fs::read_to_string("input/blueprint_example.txt").unwrap();
        let mut machine = Machine::from(blueprint.as_str());

        let checksum = machine.run_to_diagnostic();

        assert_eq!(checksum, 3);
        assert_eq!(machine.current_state, 'A');
    }

    #[test]
    fn test_input() {
        let blueprint = std::fs::read_to_string("input/blueprint.txt").unwrap();
        let mut machine = Machine::from(blueprint.as_str());

        let checksum = machine.run_to_diagnostic();

        assert_eq!(checksum, 5744);
        assert_eq!(machine.current_state, 'D');
    }
}
