#![allow(dead_code)]

fn main() {
    println!("Advent of Code 2019 - day 02");
}

#[derive(Debug, PartialEq, Clone)]
enum ProgramState {
    Running,
    Halted,
}

#[derive(Debug, Clone)]
struct Program {
    memory: Vec<usize>,
    state: ProgramState,
    instruction_pointer: usize,
}

impl From<&str> for Program {
    fn from(value: &str) -> Self {
        let memory = value.split(',').map(|s| s.parse().unwrap()).collect();
        let state = ProgramState::Running;
        let program_counter = 0;
        Self {
            memory,
            state,
            instruction_pointer: program_counter,
        }
    }
}

impl Program {
    fn step(&mut self) {
        let op = self.memory[self.instruction_pointer];

        if op == 99 {
            self.state = ProgramState::Halted;
            return;
        }

        let param1 = self.memory[self.memory[self.instruction_pointer + 1]];
        let param2 = self.memory[self.memory[self.instruction_pointer + 2]];
        let param3_address = self.memory[self.instruction_pointer + 3];

        match op {
            1 => self.memory[param3_address] = param1 + param2,
            2 => self.memory[param3_address] = param1 * param2,
            illegal => panic!("Illegal op code {illegal}!"),
        }

        self.instruction_pointer += 4;
    }

    fn eval(&mut self) {
        while self.state != ProgramState::Halted {
            self.step();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Program, ProgramState};

    #[test]
    fn test_examples_1() {
        let program = "1,9,10,3,2,3,11,0,99,30,40,50";

        let mut program = Program::from(program);
        assert_eq!(program.instruction_pointer, 0);
        assert_eq!(program.state, ProgramState::Running);
        assert_eq!(
            program.memory,
            vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]
        );
        program.step();
        assert_eq!(program.instruction_pointer, 4);
        assert_eq!(program.state, ProgramState::Running);
        assert_eq!(
            program.memory,
            vec![1, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
        );
        program.step();
        assert_eq!(program.instruction_pointer, 8);
        assert_eq!(program.state, ProgramState::Running);
        assert_eq!(
            program.memory,
            vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
        );
        program.step();
        assert_eq!(program.instruction_pointer, 8);
        assert_eq!(program.state, ProgramState::Halted);
        assert_eq!(
            program.memory,
            vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
        );
    }

    #[test]
    fn test_examples_2() {
        let program = "1,0,0,0,99";
        let mut program = Program::from(program);
        program.eval();
        assert_eq!(program.instruction_pointer, 4);
        assert_eq!(program.state, ProgramState::Halted);
        assert_eq!(program.memory, vec![2, 0, 0, 0, 99]);
    }

    #[test]
    fn test_examples_3() {
        let program = "2,3,0,3,99";
        let mut program = Program::from(program);
        program.eval();
        assert_eq!(program.instruction_pointer, 4);
        assert_eq!(program.state, ProgramState::Halted);
        assert_eq!(program.memory, vec![2, 3, 0, 6, 99]);
    }

    #[test]
    fn test_examples_4() {
        let program = "2,4,4,5,99,0";
        let mut program = Program::from(program);
        program.eval();
        assert_eq!(program.instruction_pointer, 4);
        assert_eq!(program.state, ProgramState::Halted);
        assert_eq!(program.memory, vec![2, 4, 4, 5, 99, 9801]);
    }

    #[test]
    fn test_examples_5() {
        let program = "1,1,1,4,99,5,6,0,99";
        let mut program = Program::from(program);
        program.eval();
        assert_eq!(program.instruction_pointer, 8);
        assert_eq!(program.state, ProgramState::Halted);
        assert_eq!(program.memory, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }

    #[test]
    fn test_input() {
        let program = std::fs::read_to_string("input/program.txt").unwrap();
        let mut program = Program::from(program.as_str());
        program.memory[1] = 12;
        program.memory[2] = 2;
        program.eval();
        assert_eq!(program.state, ProgramState::Halted);
        assert_eq!(program.memory[0], 3931283);
    }

    #[test]
    fn test_input2() {
        let program = std::fs::read_to_string("input/program.txt").unwrap();
        let program_reset = Program::from(program.as_str());

        let inputs: Vec<(usize, usize)> = (0..=99)
            .map(|noun| {
                (0..=99)
                    .map(|verb| (noun, verb))
                    .collect::<Vec<(usize, usize)>>()
            })
            .flatten()
            .collect();

        let mut program = program_reset.clone();
        let mut result = usize::MAX;
        for (noun, verb) in inputs {
            result = 100 * noun + verb;
            program.memory[1] = noun;
            program.memory[2] = verb;
            program.eval();

            if program.memory[0] == 19690720 {
                break;
            }

            program = program_reset.clone();
        }

        assert_eq!(program.state, ProgramState::Halted);
        assert_eq!(program.memory[0], 19690720);
        assert_eq!(result, 6979);
    }
}
