#![allow(dead_code)]

fn main() {
    println!("Advent of Code 2015 - day 23");
}

#[derive(Debug)]
enum Register {
    A,
    B,
}

impl From<&str> for Register {
    fn from(value: &str) -> Self {
        match value.trim() {
            "a" => Register::A,
            "b" => Register::B,
            _ => panic!("illegal register {value}"),
        }
    }
}

#[derive(Debug)]
enum Instruction {
    HLF(Register),
    TPL(Register),
    INC(Register),
    JMP(isize),
    JIE(Register, isize),
    JIO(Register, isize),
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        match &value[0..3] {
            "hlf" => Instruction::HLF(Register::from(&value[4..])),
            "tpl" => Instruction::TPL(Register::from(&value[4..])),
            "inc" => Instruction::INC(Register::from(&value[4..])),
            "jmp" => Instruction::JMP(parse_offset(&value[4..])),
            "jie" => Instruction::JIE(Register::from(&value[4..5]), parse_offset(&value[6..])),
            "jio" => Instruction::JIO(Register::from(&value[4..5]), parse_offset(&value[6..])),
            _ => panic!("illegal instruction"),
        }
    }
}

#[derive(Debug)]
struct InstructionPointer(usize);

impl InstructionPointer {
    fn offset(&mut self, o: isize) {
        let ip: isize = self.0.try_into().unwrap();
        self.0 = (ip + o).try_into().unwrap();
    }
}

#[derive(Debug)]
enum MachineState {
    Running,
    Exited,
}

#[derive(Debug)]
struct Machine {
    registers: [usize; 2],
    pgm: Vec<Instruction>,
    in_ptr: InstructionPointer,
    state: MachineState,
}

impl Machine {
    fn new(code: Vec<Instruction>) -> Self {
        Self {
            registers: [0; 2],
            pgm: code,
            in_ptr: InstructionPointer(0),
            state: MachineState::Running,
        }
    }

    fn register(&self, r: Register) -> usize {
        self.registers[self._r_idx(&r)]
    }

    fn _r_idx(&self, r: &Register) -> usize {
        match r {
            Register::A => 0,
            Register::B => 1,
        }
    }

    fn execute(&mut self) {
        let instruction = self.pgm.get(self.in_ptr.0).unwrap();
        let mut offset = 1;
        match instruction {
            Instruction::HLF(r) => self.registers[self._r_idx(r)] /= 2,
            Instruction::TPL(r) => self.registers[self._r_idx(r)] *= 3,
            Instruction::INC(r) => self.registers[self._r_idx(r)] += 1,
            Instruction::JMP(o) => offset = *o,
            Instruction::JIE(r, o) => {
                if self.registers[self._r_idx(r)] % 2 == 0 {
                    offset = *o;
                }
            }
            Instruction::JIO(r, o) => {
                if self.registers[self._r_idx(r)] == 1 {
                    offset = *o;
                }
            }
        }
        self.in_ptr.offset(offset);
        if self.in_ptr.0 >= self.pgm.len() {
            self.state = MachineState::Exited;
        }
    }
}

fn parse_offset(value: &str) -> isize {
    let value = value.trim();
    match &value[0..1] {
        "+" => (&value[1..]).parse::<isize>().unwrap(),
        "-" => -(&value[1..]).parse::<isize>().unwrap(),
        _ => panic!("illegal offset"),
    }
}

fn parse_code(code: &str) -> Vec<Instruction> {
    let mut result = Vec::new();

    for line in code.lines() {
        result.push(Instruction::from(line));
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::{parse_code, Machine, MachineState, Register};

    #[test]
    fn test_examples() {
        let s = "\
            inc a\n\
            jio a, +2\n\
            tpl a\n\
            inc a\
        ";

        let c = parse_code(s);
        let mut m = Machine::new(c);

        while let MachineState::Running = m.state {
            m.execute();
        }

        assert_eq!(m.register(Register::A), 2);
        assert_eq!(m.register(Register::B), 0);
    }

    #[test]
    fn test_input() {
        let s = std::fs::read_to_string("input/program.txt").unwrap();

        let c = parse_code(&s);
        let mut m = Machine::new(c);

        while let MachineState::Running = m.state {
            m.execute();
        }

        assert_eq!(m.register(Register::A), 1);
        assert_eq!(m.register(Register::B), 184);

        let c = parse_code(&s);
        let mut m = Machine::new(c);
        m.registers[0] = 1;

        while let MachineState::Running = m.state {
            m.execute();
        }

        assert_eq!(m.register(Register::A), 1);
        assert_eq!(m.register(Register::B), 231);
    }
}
