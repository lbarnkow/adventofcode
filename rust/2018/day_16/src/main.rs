#![allow(dead_code)]

use std::{
    collections::{HashMap, HashSet},
    iter::Peekable,
    str::Lines,
};

use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    println!("Advent of Code 2018 - day 16");
}

lazy_static! {
    static ref RE_BEFORE: Regex =
        Regex::new(r"^Before:\s+\[(-?\d+), (-?\d+), (-?\d+), (-?\d+)\]$").unwrap();
    static ref RE_OPERATION: Regex = Regex::new(r"^(-?\d+) (-?\d+) (-?\d+) (-?\d+)$").unwrap();
    static ref RE_AFTER: Regex =
        Regex::new(r"^After:\s+\[(-?\d+), (-?\d+), (-?\d+), (-?\d+)\]$").unwrap();
}

#[derive(Debug)]
enum Register {
    R0,
    R1,
    R2,
    R3,
}

impl From<i64> for Register {
    fn from(value: i64) -> Self {
        match value {
            0 => Self::R0,
            1 => Self::R1,
            2 => Self::R2,
            3 => Self::R3,
            _ => panic!("Illegal register {value}!"),
        }
    }
}

impl Register {
    fn idx(&self) -> usize {
        match self {
            Register::R0 => 0,
            Register::R1 => 1,
            Register::R2 => 2,
            Register::R3 => 3,
        }
    }
}

#[derive(Debug)]
enum Op {
    Addr {
        a: Register,
        b: Register,
        c: Register,
    },
    Addi {
        a: Register,
        b: i64,
        c: Register,
    },
    Mulr {
        a: Register,
        b: Register,
        c: Register,
    },
    Muli {
        a: Register,
        b: i64,
        c: Register,
    },
    Banr {
        a: Register,
        b: Register,
        c: Register,
    },
    Bani {
        a: Register,
        b: i64,
        c: Register,
    },
    Borr {
        a: Register,
        b: Register,
        c: Register,
    },
    Bori {
        a: Register,
        b: i64,
        c: Register,
    },
    Setr {
        a: Register,
        b: i64,
        c: Register,
    },
    Seti {
        a: i64,
        b: i64,
        c: Register,
    },
    Gtir {
        a: i64,
        b: Register,
        c: Register,
    },
    Gtri {
        a: Register,
        b: i64,
        c: Register,
    },
    Gtrr {
        a: Register,
        b: Register,
        c: Register,
    },
    Eqir {
        a: i64,
        b: Register,
        c: Register,
    },
    Eqri {
        a: Register,
        b: i64,
        c: Register,
    },
    Eqrr {
        a: Register,
        b: Register,
        c: Register,
    },
}

impl Op {
    fn eval_add(a: i64, b: i64) -> i64 {
        a + b
    }

    fn eval(&self, r: &mut [i64]) {
        match self {
            Op::Addr { a, b, c } => r[c.idx()] = r[a.idx()] + r[b.idx()],
            Op::Addi { a, b, c } => r[c.idx()] = r[a.idx()] + *b,
            Op::Mulr { a, b, c } => r[c.idx()] = r[a.idx()] * r[b.idx()],
            Op::Muli { a, b, c } => r[c.idx()] = r[a.idx()] * b,
            Op::Banr { a, b, c } => r[c.idx()] = r[a.idx()] & r[b.idx()],
            Op::Bani { a, b, c } => r[c.idx()] = r[a.idx()] & *b,
            Op::Borr { a, b, c } => r[c.idx()] = r[a.idx()] | r[b.idx()],
            Op::Bori { a, b, c } => r[c.idx()] = r[a.idx()] | *b,
            Op::Setr { a, b: _, c } => r[c.idx()] = r[a.idx()],
            Op::Seti { a, b: _, c } => r[c.idx()] = *a,
            Op::Gtir { a, b, c } => r[c.idx()] = if *a > r[b.idx()] { 1 } else { 0 },
            Op::Gtri { a, b, c } => r[c.idx()] = if r[a.idx()] > *b { 1 } else { 0 },
            Op::Gtrr { a, b, c } => r[c.idx()] = if r[a.idx()] > r[b.idx()] { 1 } else { 0 },
            Op::Eqir { a, b, c } => r[c.idx()] = if *a == r[b.idx()] { 1 } else { 0 },
            Op::Eqri { a, b, c } => r[c.idx()] = if r[a.idx()] == *b { 1 } else { 0 },
            Op::Eqrr { a, b, c } => r[c.idx()] = if r[a.idx()] == r[b.idx()] { 1 } else { 0 },
        }
    }

    fn name(&self) -> &'static str {
        match self {
            Op::Addr { a: _, b: _, c: _ } => "addr",
            Op::Addi { a: _, b: _, c: _ } => "addi",
            Op::Mulr { a: _, b: _, c: _ } => "mulr",
            Op::Muli { a: _, b: _, c: _ } => "muli",
            Op::Banr { a: _, b: _, c: _ } => "banr",
            Op::Bani { a: _, b: _, c: _ } => "bani",
            Op::Borr { a: _, b: _, c: _ } => "borr",
            Op::Bori { a: _, b: _, c: _ } => "bori",
            Op::Setr { a: _, b: _, c: _ } => "setr",
            Op::Seti { a: _, b: _, c: _ } => "seti",
            Op::Gtir { a: _, b: _, c: _ } => "gtir",
            Op::Gtri { a: _, b: _, c: _ } => "gtri",
            Op::Gtrr { a: _, b: _, c: _ } => "gtrr",
            Op::Eqir { a: _, b: _, c: _ } => "eqir",
            Op::Eqri { a: _, b: _, c: _ } => "eqri",
            Op::Eqrr { a: _, b: _, c: _ } => "eqrr",
        }
    }

    fn lookup(table: &HashMap<i64, &'static str>, op: &[i64]) -> Self {
        match table[&op[0]] {
            "addr" => Self::Addr {
                a: op[1].into(),
                b: op[2].into(),
                c: op[3].into(),
            },
            "addi" => Self::Addi {
                a: op[1].into(),
                b: op[2],
                c: op[3].into(),
            },
            "mulr" => Self::Mulr {
                a: op[1].into(),
                b: op[2].into(),
                c: op[3].into(),
            },
            "muli" => Self::Muli {
                a: op[1].into(),
                b: op[2],
                c: op[3].into(),
            },
            "banr" => Self::Banr {
                a: op[1].into(),
                b: op[2].into(),
                c: op[3].into(),
            },
            "bani" => Self::Bani {
                a: op[1].into(),
                b: op[2],
                c: op[3].into(),
            },
            "borr" => Self::Borr {
                a: op[1].into(),
                b: op[2].into(),
                c: op[3].into(),
            },
            "bori" => Self::Bori {
                a: op[1].into(),
                b: op[2],
                c: op[3].into(),
            },
            "setr" => Self::Setr {
                a: op[1].into(),
                b: op[2],
                c: op[3].into(),
            },
            "seti" => Self::Seti {
                a: op[1],
                b: op[2],
                c: op[3].into(),
            },
            "gtir" => Self::Gtir {
                a: op[1],
                b: op[2].into(),
                c: op[3].into(),
            },
            "gtri" => Self::Gtri {
                a: op[1].into(),
                b: op[2],
                c: op[3].into(),
            },
            "gtrr" => Self::Gtrr {
                a: op[1].into(),
                b: op[2].into(),
                c: op[3].into(),
            },
            "eqir" => Self::Eqir {
                a: op[1],
                b: op[2].into(),
                c: op[3].into(),
            },
            "eqri" => Self::Eqri {
                a: op[1].into(),
                b: op[2],
                c: op[3].into(),
            },
            "eqrr" => Self::Eqrr {
                a: op[1].into(),
                b: op[2].into(),
                c: op[3].into(),
            },
            _ => panic!(),
        }
    }
}

#[derive(Debug)]
struct Capture {
    before: [i64; 4],
    operation: [i64; 4],
    after: [i64; 4],
}

impl Capture {
    fn from_iter<'a, T>(iter: &mut T) -> Self
    where
        T: Iterator<Item = &'a str>,
    {
        let caps = RE_BEFORE.captures(iter.next().unwrap()).unwrap();
        let before = [
            caps[1].parse().unwrap(),
            caps[2].parse().unwrap(),
            caps[3].parse().unwrap(),
            caps[4].parse().unwrap(),
        ];

        let caps = RE_OPERATION.captures(iter.next().unwrap()).unwrap();
        let operation = [
            caps[1].parse().unwrap(),
            caps[2].parse().unwrap(),
            caps[3].parse().unwrap(),
            caps[4].parse().unwrap(),
        ];

        let caps = RE_AFTER.captures(iter.next().unwrap()).unwrap();
        let after = [
            caps[1].parse().unwrap(),
            caps[2].parse().unwrap(),
            caps[3].parse().unwrap(),
            caps[4].parse().unwrap(),
        ];

        assert!(iter.next().unwrap().is_empty()); // consume empty line

        Self {
            before,
            operation,
            after,
        }
    }
}

fn determine_viable_op_codes(cap: &Capture) -> HashSet<&'static str> {
    let ops = [
        Op::Addr {
            a: cap.operation[1].into(),
            b: cap.operation[2].into(),
            c: cap.operation[3].into(),
        },
        Op::Addi {
            a: cap.operation[1].into(),
            b: cap.operation[2],
            c: cap.operation[3].into(),
        },
        Op::Mulr {
            a: cap.operation[1].into(),
            b: cap.operation[2].into(),
            c: cap.operation[3].into(),
        },
        Op::Muli {
            a: cap.operation[1].into(),
            b: cap.operation[2],
            c: cap.operation[3].into(),
        },
        Op::Banr {
            a: cap.operation[1].into(),
            b: cap.operation[2].into(),
            c: cap.operation[3].into(),
        },
        Op::Bani {
            a: cap.operation[1].into(),
            b: cap.operation[2],
            c: cap.operation[3].into(),
        },
        Op::Borr {
            a: cap.operation[1].into(),
            b: cap.operation[2].into(),
            c: cap.operation[3].into(),
        },
        Op::Bori {
            a: cap.operation[1].into(),
            b: cap.operation[2],
            c: cap.operation[3].into(),
        },
        Op::Setr {
            a: cap.operation[1].into(),
            b: cap.operation[2],
            c: cap.operation[3].into(),
        },
        Op::Seti {
            a: cap.operation[1],
            b: cap.operation[2],
            c: cap.operation[3].into(),
        },
        Op::Gtir {
            a: cap.operation[1],
            b: cap.operation[2].into(),
            c: cap.operation[3].into(),
        },
        Op::Gtri {
            a: cap.operation[1].into(),
            b: cap.operation[2],
            c: cap.operation[3].into(),
        },
        Op::Gtrr {
            a: cap.operation[1].into(),
            b: cap.operation[2].into(),
            c: cap.operation[3].into(),
        },
        Op::Eqir {
            a: cap.operation[1],
            b: cap.operation[2].into(),
            c: cap.operation[3].into(),
        },
        Op::Eqri {
            a: cap.operation[1].into(),
            b: cap.operation[2],
            c: cap.operation[3].into(),
        },
        Op::Eqrr {
            a: cap.operation[1].into(),
            b: cap.operation[2].into(),
            c: cap.operation[3].into(),
        },
    ];

    ops.into_iter()
        .filter_map(|op| {
            let mut r = cap.before;
            op.eval(&mut r);
            if r == cap.after {
                Some(op.name())
            } else {
                None
            }
        })
        .collect()
}

fn parse_input(file: &str) -> (Vec<Capture>, Vec<Vec<i64>>) {
    let input = std::fs::read_to_string(file).unwrap();
    let mut iter: Peekable<Lines<'_>> = input.lines().peekable();

    let mut captures = Vec::new();

    while !iter.peek().unwrap().is_empty() {
        captures.push(Capture::from_iter(&mut iter));
    }

    assert!(iter.next().unwrap().is_empty()); // consume empty line
    assert!(iter.next().unwrap().is_empty()); // consume empty line

    let instructions = iter
        .map(|s| {
            s.split(' ')
                .map(|i| i.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<i64>>>();

    (captures, instructions)
}

fn determine_op_codes(captures: &[Capture]) -> HashMap<i64, &'static str> {
    let ops = [
        "addr", "addi", "mulr", "muli", "banr", "bani", "borr", "bori", "setr", "seti", "gtir",
        "gtri", "gtrr", "eqir", "eqri", "eqrr",
    ];

    let matched = captures
        .iter()
        .map(|cap| (determine_viable_op_codes(cap), cap))
        .collect::<Vec<(HashSet<&str>, &Capture)>>();

    let mut op_codes = HashMap::new();
    for i in 0..ops.len() {
        let mut ops = HashSet::from(ops);
        for (viable_ops, _) in matched.iter().filter(|(_, c)| c.operation[0] == i as i64) {
            ops = ops.intersection(viable_ops).copied().collect();
        }
        op_codes.insert(i as i64, ops);
    }

    let mut solved = HashMap::new();
    loop {
        for (op_code, viable_ops) in &op_codes {
            if viable_ops.len() == 1 {
                solved.insert(*op_code, *viable_ops.iter().next().unwrap());
            }
        }
        for (op_code, op) in &solved {
            op_codes.remove(op_code);
            for (_, viable_ops) in op_codes.iter_mut() {
                if viable_ops.contains(*op) {
                    viable_ops.remove(*op);
                }
            }
        }
        if op_codes.is_empty() {
            break;
        }
    }

    solved
}

fn convert_ops(op_table: &HashMap<i64, &'static str>, captures: &[Vec<i64>]) -> Vec<Op> {
    captures.iter().map(|c| Op::lookup(op_table, c)).collect()
}

fn eval_program(registers: &mut [i64; 4], ops: &[Op]) {
    for op in ops {
        op.eval(registers);
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::{
        convert_ops, determine_op_codes, determine_viable_op_codes, eval_program, parse_input,
        Capture,
    };

    #[test]
    fn test_part_1() {
        let (captures, _) = parse_input("input/log.txt");
        let filtered = captures
            .into_iter()
            .map(|cap| (determine_viable_op_codes(&cap), cap))
            .filter(|(list, _)| list.len() >= 3)
            .collect::<Vec<(HashSet<&str>, Capture)>>();

        assert_eq!(filtered.len(), 596);
    }

    #[test]
    fn test_part_2() {
        let (captures, instructions) = parse_input("input/log.txt");
        let matched = determine_op_codes(&captures);

        assert_eq!(matched[&0], "bani");
        assert_eq!(matched[&1], "addr");
        assert_eq!(matched[&2], "mulr");
        assert_eq!(matched[&3], "addi");
        assert_eq!(matched[&4], "gtri");
        assert_eq!(matched[&5], "banr");
        assert_eq!(matched[&6], "borr");
        assert_eq!(matched[&7], "eqri");
        assert_eq!(matched[&8], "seti");
        assert_eq!(matched[&9], "eqrr");
        assert_eq!(matched[&10], "bori");
        assert_eq!(matched[&11], "setr");
        assert_eq!(matched[&12], "eqir");
        assert_eq!(matched[&13], "muli");
        assert_eq!(matched[&14], "gtrr");
        assert_eq!(matched[&15], "gtir");

        let instructions = convert_ops(&matched, &instructions);
        let mut registers = [0; 4];

        eval_program(&mut registers, &instructions);

        assert_eq!(registers[0], 554);
        assert_eq!(registers[1], 2);
        assert_eq!(registers[2], 3);
        assert_eq!(registers[3], 554);
    }
}
