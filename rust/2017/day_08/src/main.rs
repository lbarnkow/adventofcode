#![allow(dead_code)]

use std::collections::HashMap;

fn main() {
    println!("Advent of Code 2017 - day 08");
}

type Register = String;

#[derive(Debug, Clone)]
enum ComparisonOperator {
    Gt,
    Ge,
    Lt,
    Le,
    Eq,
    Ne,
}

impl From<&str> for ComparisonOperator {
    fn from(value: &str) -> Self {
        match value {
            ">" => Self::Gt,
            ">=" => Self::Ge,
            "<" => Self::Lt,
            "<=" => Self::Le,
            "==" => Self::Eq,
            "!=" => Self::Ne,
            _ => panic!("Illegal ComparisonOperator: {value}"),
        }
    }
}

impl ComparisonOperator {
    fn eval(&self, lhs: i64, rhs: i64) -> bool {
        match self {
            ComparisonOperator::Gt => lhs > rhs,
            ComparisonOperator::Ge => lhs >= rhs,
            ComparisonOperator::Lt => lhs < rhs,
            ComparisonOperator::Le => lhs <= rhs,
            ComparisonOperator::Eq => lhs == rhs,
            ComparisonOperator::Ne => lhs != rhs,
        }
    }
}

#[derive(Debug, Clone)]
enum BinaryOperator {
    Inc,
    Dec,
}

impl From<&str> for BinaryOperator {
    fn from(value: &str) -> Self {
        match value {
            "inc" => Self::Inc,
            "dec" => Self::Dec,
            _ => panic!("Illegal BinaryOperator: {value}"),
        }
    }
}

impl BinaryOperator {
    fn eval(&self, lhs: i64, rhs: i64) -> i64 {
        match self {
            BinaryOperator::Inc => lhs + rhs,
            BinaryOperator::Dec => lhs - rhs,
        }
    }
}

#[derive(Debug, Clone)]
struct Condition {
    lhs: Register,
    op: ComparisonOperator,
    rhs: i64,
}

#[derive(Debug, Clone)]
struct Instruction {
    lhs: Register,
    op: BinaryOperator,
    rhs: i64,
    cond: Condition,
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        let split: Vec<&str> = value.split(" ").collect();
        assert_eq!(split.len(), 7);

        let dst_r = split[0].to_owned();
        let op = BinaryOperator::from(split[1]);
        let operand = split[2].parse::<i64>().unwrap();

        assert_eq!(split[3], "if");

        let cond = Condition {
            lhs: split[4].to_owned(),
            op: ComparisonOperator::from(split[5]),
            rhs: split[6].parse::<i64>().unwrap(),
        };

        Self {
            lhs: dst_r,
            op,
            rhs: operand,
            cond,
        }
    }
}

struct Machine {
    registers: HashMap<String, i64>,
    instructions: Vec<Instruction>,
}

impl From<&str> for Machine {
    fn from(value: &str) -> Self {
        let instructions: Vec<Instruction> =
            value.lines().map(|line| Instruction::from(line)).collect();
        let mut m = Self {
            registers: HashMap::new(),
            instructions,
        };
        m.reset();
        m
    }
}

impl Machine {
    fn reset(&mut self) {
        self.registers.clear();
    }

    fn get_register(&mut self, r: &str) -> i64 {
        if let Some(v) = self.registers.get(r) {
            *v
        } else {
            self.registers.insert(r.to_owned(), 0);
            0
        }
    }

    fn set_register(&mut self, r: &str, v: i64) {
        self.registers.insert(r.to_owned(), v);
    }

    fn eval(&mut self) -> i64 {
        let mut max_val = self.largest_register_value();

        for instruction in self.instructions.clone() {
            let cond = instruction.cond;

            if cond.op.eval(self.get_register(&cond.lhs), cond.rhs) {
                let v = self.get_register(&instruction.lhs);
                let v = instruction.op.eval(v, instruction.rhs);
                self.set_register(&instruction.lhs, v);
                max_val = max_val.max(v);
            }
        }

        max_val
    }

    fn largest_register_value(&self) -> i64 {
        if let Some(v) = self.registers.iter().map(|(_, v)| *v).max() {
            v
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Machine;

    #[test]
    fn test_examples() {
        let instructions = "\
            b inc 5 if a > 1\n\
            a inc 1 if b < 5\n\
            c dec -10 if a >= 1\n\
            c inc -20 if c == 10\
        ";

        let mut m = Machine::from(instructions);
        let max_val = m.eval();
        assert_eq!(m.largest_register_value(), 1);
        assert_eq!(max_val, 10);
    }

    #[test]
    fn test_input() {
        let instructions = std::fs::read_to_string("input/instructions.txt").unwrap();

        let mut m = Machine::from(instructions.as_str());
        let max_val = m.eval();
        assert_eq!(m.largest_register_value(), 6828);
        assert_eq!(max_val, 7234);
    }
}
