#![allow(dead_code)]
use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::{Match, Regex};

lazy_static! {
    static ref REGEX: Regex = Regex::new(
        r"^(?:(NOT)\s)?(\w+|\d+)(?:\s(AND|OR|LSHIFT|RSHIFT)\s)?(?:(NOT)\s)?(\w+|\d+)?\s->\s(\w+)$"
    )
    .unwrap();
}

fn main() {
    println!("Advent of Code 2015 - day 7");
}

#[derive(Debug)]
enum Operand {
    Value(u16),
    Wire(String),
    NotWire(String),
}

impl Operand {
    fn parse(not: Option<Match<'_>>, operand: &str) -> Self {
        if let Ok(val) = str::parse::<u16>(operand) {
            if not.is_none() {
                Self::Value(val)
            } else {
                Self::Value(!val)
            }
        } else {
            if not.is_none() {
                Self::Wire(operand.to_owned())
            } else {
                Self::NotWire(operand.to_owned())
            }
        }
    }
}

#[derive(Debug)]
enum Op {
    And,
    Or,
    Lshift,
    Rshift,
}

impl Op {
    fn parse(s: &str) -> Self {
        match s {
            "AND" => Self::And,
            "OR" => Self::Or,
            "LSHIFT" => Self::Lshift,
            "RSHIFT" => Self::Rshift,
            _ => panic!("Invalid instruction!"),
        }
    }
}

#[derive(Debug)]
enum Expression {
    Simple {
        operand: Operand,
    },
    Complex {
        operand1: Operand,
        op: Op,
        operand2: Operand,
    },
}

fn parse_instruction(instruction: &str) -> (String, Expression) {
    let cap = REGEX.captures(instruction).unwrap();

    if let Some(op) = cap.get(3) {
        let operand1 = Operand::parse(cap.get(1), cap.get(2).unwrap().as_str());
        let op = Op::parse(op.as_str());
        let operand2 = Operand::parse(cap.get(4), cap.get(5).unwrap().as_str());
        let rhs = cap.get(6).unwrap().as_str().to_owned();

        (
            rhs.clone(),
            Expression::Complex {
                operand1,
                op,
                operand2,
            },
        )
    } else {
        let lhs = Operand::parse(cap.get(1), cap.get(2).unwrap().as_str());
        let rhs = cap.get(6).unwrap().as_str().to_owned();

        (rhs.clone(), Expression::Simple { operand: lhs })
    }
}

fn compute_output(instructions: &str, out_wire: &str) -> u16 {
    let mut circuit = HashMap::new();

    for line in instructions.lines() {
        let (rhs, i) = parse_instruction(line);
        println!("{i:?} --> {rhs}");

        circuit.insert(rhs, i);
    }

    let resolved = resolve_circuit(&circuit);

    *resolved.get(out_wire).unwrap()
}

fn try_resolve_operand(resolved: &HashMap<String, u16>, op: &Operand) -> Option<u16> {
    match op {
        Operand::Value(val) => Some(*val),
        Operand::Wire(wire) => {
            if let Some(val) = resolved.get(wire) {
                Some(*val)
            } else {
                None
            }
        }
        Operand::NotWire(wire) => {
            if let Some(val) = resolved.get(wire) {
                Some(!*val)
            } else {
                None
            }
        }
    }
}

fn apply_op(operand1: u16, op: &Op, operand2: u16) -> u16 {
    match op {
        Op::And => operand1 & operand2,
        Op::Or => operand1 | operand2,
        Op::Lshift => operand1 << operand2,
        Op::Rshift => operand1 >> operand2,
    }
}

fn resolve_circuit(circuit: &HashMap<String, Expression>) -> HashMap<String, u16> {
    let mut resolved = HashMap::new();

    let mut made_changes = true;

    while made_changes {
        made_changes = false;

        for (wire, exp) in circuit {
            if resolved.contains_key(wire) {
                continue;
            }

            match exp {
                Expression::Simple { operand } => {
                    if let Some(val) = try_resolve_operand(&resolved, operand) {
                        resolved.insert(wire.clone(), val);
                        made_changes = true;
                    }
                }
                Expression::Complex {
                    operand1,
                    op,
                    operand2,
                } => {
                    let operand1 = try_resolve_operand(&resolved, operand1);
                    let operand2 = try_resolve_operand(&resolved, operand2);

                    if operand1.is_some() && operand2.is_some() {
                        let val = apply_op(operand1.unwrap(), op, operand2.unwrap());
                        resolved.insert(wire.clone(), val);
                        made_changes = true;
                    }
                }
            };
        }
    }

    if circuit.len() != resolved.len() {
        panic!("Couldn't resolve all wirings!");
    }

    resolved
}

#[cfg(test)]
mod tests {
    use crate::compute_output;

    #[test]
    fn test_examples() {
        let instructions = "123 -> x\n\
            456 -> y\n\
            x AND y -> d\n\
            x OR y -> e\n\
            x LSHIFT 2 -> f\n\
            y RSHIFT 2 -> g\n\
            NOT x -> h\n\
            NOT y -> i";

        assert_eq!(compute_output(instructions, "x"), 123);
        assert_eq!(compute_output(instructions, "y"), 456);
        assert_eq!(compute_output(instructions, "d"), 72);
        assert_eq!(compute_output(instructions, "e"), 507);
        assert_eq!(compute_output(instructions, "f"), 492);
        assert_eq!(compute_output(instructions, "g"), 114);
        assert_eq!(compute_output(instructions, "h"), 65412);
        assert_eq!(compute_output(instructions, "i"), 65079);
    }

    #[test]
    fn test_input() {
        let instructions = std::fs::read_to_string("input/instructions.txt").unwrap();
        assert_eq!(compute_output(&instructions, "a"), 3176);

        let instructions = std::fs::read_to_string("input/instructions_step2.txt").unwrap();
        assert_eq!(compute_output(&instructions, "a"), 14710);
    }
}
