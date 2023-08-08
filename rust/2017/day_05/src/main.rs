#![allow(dead_code)]

fn main() {
    println!("Advent of Code 2017 - day 05");
}

#[derive(Debug, PartialEq)]
enum MachineType {
    RegularJump,
    ModifiedJump,
}

#[derive(Debug, PartialEq)]
enum MachineState {
    Alive,
    Dead,
}

enum Instruction {
    Jmp(isize),
}

struct Machine {
    mt: MachineType,
    state: MachineState,
    instructions: Vec<Instruction>,
    program_counter: isize,
    cycles: usize,
}

impl Machine {
    fn from(mt: MachineType, value: &str) -> Self {
        let instructions = value
            .lines()
            .map(|line| line.parse::<isize>().unwrap())
            .map(|offset| Instruction::Jmp(offset))
            .collect();

        Self {
            mt,
            state: MachineState::Alive,
            instructions,
            program_counter: 0,
            cycles: 0,
        }
    }
}

impl Machine {
    fn run(&mut self) {
        if self.state != MachineState::Alive {
            panic!("Illegal state!");
        }

        loop {
            let pc: usize = self.program_counter.try_into().unwrap();
            if self.program_counter < 0 || pc >= self.instructions.len() {
                break;
            }

            let (mod_instr, offset) = match self.instructions[pc] {
                Instruction::Jmp(offset) => {
                    let mod_offset = match self.mt {
                        MachineType::RegularJump => offset + 1,
                        MachineType::ModifiedJump => {
                            if offset >= 3 {
                                offset - 1
                            } else {
                                offset + 1
                            }
                        }
                    };
                    (Instruction::Jmp(mod_offset), offset)
                }
            };
            self.instructions[pc] = mod_instr;
            self.program_counter += offset;

            self.cycles += 1;
        }

        self.state = MachineState::Dead;
    }
}

#[cfg(test)]
mod tests {
    use crate::{Machine, MachineState, MachineType};

    #[test]
    fn test_examples() {
        let msg = "\
            0\n\
            3\n\
            0\n\
            1\n\
            -3\
        ";

        let mut m = Machine::from(MachineType::RegularJump, msg);
        assert_eq!(m.state, MachineState::Alive);
        m.run();
        assert_eq!(m.state, MachineState::Dead);
        assert_eq!(m.cycles, 5);

        let mut m = Machine::from(MachineType::ModifiedJump, msg);
        assert_eq!(m.state, MachineState::Alive);
        m.run();
        assert_eq!(m.state, MachineState::Dead);
        assert_eq!(m.cycles, 10);
    }

    #[test]
    fn test_input() {
        let msg = std::fs::read_to_string("input/instructions.txt").unwrap();

        let mut m = Machine::from(MachineType::RegularJump, msg.as_str());
        assert_eq!(m.state, MachineState::Alive);
        m.run();
        assert_eq!(m.state, MachineState::Dead);
        assert_eq!(m.cycles, 318883);

        let mut m = Machine::from(MachineType::ModifiedJump, msg.as_str());
        assert_eq!(m.state, MachineState::Alive);
        m.run();
        assert_eq!(m.state, MachineState::Dead);
        assert_eq!(m.cycles, 23948711);
    }
}
