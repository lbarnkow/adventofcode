#![allow(dead_code)]

use std::collections::VecDeque;

fn main() {
    println!("Advent of Code 2019 - day 07");
}

const PARAMETER_MODE_FLAGS: usize = 3;

#[derive(Debug, Clone, Copy, PartialEq)]
enum ParameterMode {
    Position,
    Immediate,
}

impl From<isize> for ParameterMode {
    fn from(value: isize) -> Self {
        match value {
            0 => Self::Position,
            1 => Self::Immediate,
            x => panic!("Illegal ParamterMode: {x}!"),
        }
    }
}

impl Default for ParameterMode {
    fn default() -> Self {
        Self::Position
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Mnemonic {
    Add,
    Mul,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    Halt,
}

impl Mnemonic {
    fn instruction_pointer_offset(&self) -> usize {
        match self {
            Mnemonic::Add | Mnemonic::Mul | Mnemonic::LessThan | Mnemonic::Equals => 4,
            Mnemonic::Input | Mnemonic::Output => 2,
            Mnemonic::Halt => 0,
            Mnemonic::JumpIfTrue | Mnemonic::JumpIfFalse => 3,
        }
    }
}

impl From<isize> for Mnemonic {
    fn from(value: isize) -> Self {
        match value {
            1 => Self::Add,
            2 => Self::Mul,
            3 => Self::Input,
            4 => Self::Output,
            5 => Self::JumpIfTrue,
            6 => Self::JumpIfFalse,
            7 => Self::LessThan,
            8 => Self::Equals,
            99 => Self::Halt,
            x => panic!("Illegal mnemonic: {x}!"),
        }
    }
}

struct OpCode {
    mnemonic: Mnemonic,
    parameter_modes: [ParameterMode; PARAMETER_MODE_FLAGS],
}

impl OpCode {
    fn new(mnemonic: Mnemonic, parameter_modes: [ParameterMode; PARAMETER_MODE_FLAGS]) -> Self {
        match mnemonic {
            Mnemonic::Add | Mnemonic::Mul | Mnemonic::LessThan | Mnemonic::Equals => {
                assert_eq!(parameter_modes[2], ParameterMode::Position)
            }
            Mnemonic::Input => {
                assert_eq!(parameter_modes[0], ParameterMode::Position)
            }
            Mnemonic::Output | Mnemonic::JumpIfFalse | Mnemonic::JumpIfTrue | Mnemonic::Halt => (),
        }

        Self {
            mnemonic,
            parameter_modes,
        }
    }

    fn get_paramter(memory: &mut [isize], parameter_mode: ParameterMode, offset: usize) -> isize {
        let parameter = memory[offset];
        match parameter_mode {
            ParameterMode::Position => memory[usize::try_from(parameter).unwrap()],
            ParameterMode::Immediate => memory[offset],
        }
    }

    fn eval_add(&self, memory: &mut [isize], instruction_pointer: usize) -> (usize, State) {
        let param_1 = Self::get_paramter(memory, self.parameter_modes[0], instruction_pointer + 1);
        let param_2 = Self::get_paramter(memory, self.parameter_modes[1], instruction_pointer + 2);
        let target_idx = usize::try_from(memory[instruction_pointer + 3]).unwrap();
        memory[target_idx] = param_1 + param_2;
        (
            instruction_pointer + Mnemonic::Add.instruction_pointer_offset(),
            State::Running,
        )
    }

    fn eval_mul(&self, memory: &mut [isize], instruction_pointer: usize) -> (usize, State) {
        let param_1: isize =
            Self::get_paramter(memory, self.parameter_modes[0], instruction_pointer + 1);
        let param_2 = Self::get_paramter(memory, self.parameter_modes[1], instruction_pointer + 2);
        let target_idx = usize::try_from(memory[instruction_pointer + 3]).unwrap();
        memory[target_idx] = param_1 * param_2;
        (
            instruction_pointer + Mnemonic::Mul.instruction_pointer_offset(),
            State::Running,
        )
    }

    fn eval_input(
        &self,
        memory: &mut [isize],
        instruction_pointer: usize,
        io: &mut InputOutput,
    ) -> (usize, State) {
        let target_idx = usize::try_from(memory[instruction_pointer + 1]).unwrap();

        match io.read_in() {
            None => (instruction_pointer, State::WaitingForInput),
            Some(next) => {
                memory[target_idx] = next;
                (
                    instruction_pointer + Mnemonic::Input.instruction_pointer_offset(),
                    State::Running,
                )
            }
        }
    }

    fn eval_output(
        &self,
        memory: &mut [isize],
        instruction_pointer: usize,
        io: &mut InputOutput,
    ) -> (usize, State) {
        let param_1: isize =
            Self::get_paramter(memory, self.parameter_modes[0], instruction_pointer + 1);
        io.write_out(param_1);
        (
            instruction_pointer + Mnemonic::Output.instruction_pointer_offset(),
            State::Running,
        )
    }

    fn eval_jump_if_true(
        &self,
        memory: &mut [isize],
        instruction_pointer: usize,
    ) -> (usize, State) {
        let param_1 = Self::get_paramter(memory, self.parameter_modes[0], instruction_pointer + 1);
        let param_2 = Self::get_paramter(memory, self.parameter_modes[1], instruction_pointer + 2);

        if param_1 != 0 {
            (usize::try_from(param_2).unwrap(), State::Running)
        } else {
            (
                instruction_pointer + Mnemonic::JumpIfTrue.instruction_pointer_offset(),
                State::Running,
            )
        }
    }

    fn eval_jump_if_false(
        &self,
        memory: &mut [isize],
        instruction_pointer: usize,
    ) -> (usize, State) {
        let param_1 = Self::get_paramter(memory, self.parameter_modes[0], instruction_pointer + 1);
        let param_2 = Self::get_paramter(memory, self.parameter_modes[1], instruction_pointer + 2);

        if param_1 == 0 {
            (usize::try_from(param_2).unwrap(), State::Running)
        } else {
            (
                instruction_pointer + Mnemonic::JumpIfTrue.instruction_pointer_offset(),
                State::Running,
            )
        }
    }

    fn eval_less_than(&self, memory: &mut [isize], instruction_pointer: usize) -> (usize, State) {
        let param_1 = Self::get_paramter(memory, self.parameter_modes[0], instruction_pointer + 1);
        let param_2 = Self::get_paramter(memory, self.parameter_modes[1], instruction_pointer + 2);
        let target_idx = usize::try_from(memory[instruction_pointer + 3]).unwrap();
        memory[target_idx] = if param_1 < param_2 { 1 } else { 0 };
        (
            instruction_pointer + Mnemonic::LessThan.instruction_pointer_offset(),
            State::Running,
        )
    }

    fn eval_equals(&self, memory: &mut [isize], instruction_pointer: usize) -> (usize, State) {
        let param_1 = Self::get_paramter(memory, self.parameter_modes[0], instruction_pointer + 1);
        let param_2 = Self::get_paramter(memory, self.parameter_modes[1], instruction_pointer + 2);
        let target_idx = usize::try_from(memory[instruction_pointer + 3]).unwrap();
        memory[target_idx] = if param_1 == param_2 { 1 } else { 0 };
        (
            instruction_pointer + Mnemonic::Equals.instruction_pointer_offset(),
            State::Running,
        )
    }

    fn eval(
        &self,
        memory: &mut [isize],
        instruction_pointer: usize,
        io: &mut InputOutput,
    ) -> (usize, State) {
        match self.mnemonic {
            Mnemonic::Add => self.eval_add(memory, instruction_pointer),
            Mnemonic::Mul => self.eval_mul(memory, instruction_pointer),
            Mnemonic::Input => self.eval_input(memory, instruction_pointer, io),
            Mnemonic::Output => self.eval_output(memory, instruction_pointer, io),
            Mnemonic::JumpIfTrue => self.eval_jump_if_true(memory, instruction_pointer),
            Mnemonic::JumpIfFalse => self.eval_jump_if_false(memory, instruction_pointer),
            Mnemonic::LessThan => self.eval_less_than(memory, instruction_pointer),
            Mnemonic::Equals => self.eval_equals(memory, instruction_pointer),
            Mnemonic::Halt => (instruction_pointer, State::Halted),
        }
    }
}

impl From<isize> for OpCode {
    fn from(mut value: isize) -> Self {
        let mut parameter_modes = [ParameterMode::default(); PARAMETER_MODE_FLAGS];

        let mnemonic = (value % 100).into();
        value /= 100;
        for parameter_mode in &mut parameter_modes {
            *parameter_mode = (value % 10).into();
            value /= 10;
        }

        Self::new(mnemonic, parameter_modes)
    }
}

struct InputOutput {
    in_q: VecDeque<isize>,
    out_q: VecDeque<isize>,
}

impl InputOutput {
    fn new(initial_in: &[isize]) -> Self {
        Self {
            in_q: initial_in.iter().copied().collect(),
            out_q: VecDeque::new(),
        }
    }

    fn read_in(&mut self) -> Option<isize> {
        self.in_q.pop_front()
    }

    fn write_out(&mut self, data: isize) {
        self.out_q.push_back(data);
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum State {
    Running,
    WaitingForInput,
    Halted,
}

#[derive(Debug, Clone)]
struct Computer {
    state: State,
    memory: Vec<isize>,
    instruction_pointer: usize,
}

impl From<&str> for Computer {
    fn from(value: &str) -> Self {
        let memory = value.split(',').map(|s| s.parse().unwrap()).collect();
        let instruction_pointer = 0;

        Self {
            state: State::Running,
            memory,
            instruction_pointer,
        }
    }
}

impl Computer {
    fn eval(&mut self, io: &mut InputOutput) {
        if self.state == State::Halted {
            panic!("Halted program can't be resumed!")
        }

        let mut opcode = OpCode::from(self.memory[self.instruction_pointer]);

        self.state = State::Running;
        while self.state == State::Running {
            (self.instruction_pointer, self.state) =
                opcode.eval(&mut self.memory, self.instruction_pointer, io);
            opcode = OpCode::from(self.memory[self.instruction_pointer]);
        }
    }
}

struct AmpChain {
    template: Computer,
}

impl From<Computer> for AmpChain {
    fn from(value: Computer) -> Self {
        Self { template: value }
    }
}

impl AmpChain {
    fn eval(&self, phases: &[isize], initial_input: isize) -> isize {
        let mut output = initial_input;
        let mut computers: Vec<Computer> = phases.iter().map(|_| self.template.clone()).collect();
        let mut ios: Vec<InputOutput> = phases.iter().map(|p| InputOutput::new(&[*p])).collect();

        ios[0].in_q.push_back(output);
        loop {
            for idx in 0..phases.len() {
                computers[idx].eval(&mut ios[idx]);
                assert_eq!(ios[idx].out_q.len(), 1);
                output = ios[idx].out_q.pop_back().unwrap();

                let next_idx = if idx != ios.len() - 1 { idx + 1 } else { 0 };
                ios[next_idx].in_q.push_back(output);
            }

            let state =
                computers[1..]
                    .iter()
                    .map(|c| c.state)
                    .fold(computers[0].state, |acc, s| {
                        if acc != s {
                            panic!("All computer should be in the same state!");
                        } else {
                            s
                        }
                    });

            if state == State::Halted {
                break;
            }
        }

        output
    }

    fn factorial(n: usize) -> usize {
        if n == 1 {
            1
        } else {
            n * Self::factorial(n - 1)
        }
    }

    fn phase_permutations_go(
        buf: &mut Vec<Vec<isize>>,
        current: &mut Vec<isize>,
        remaining: &Vec<isize>,
    ) {
        if remaining.is_empty() {
            buf.push(current.clone());
            return;
        }

        for (idx, phase) in remaining.iter().enumerate() {
            current.push(*phase);
            let mut remaining = remaining.clone();
            remaining.remove(idx);
            Self::phase_permutations_go(buf, current, &remaining);
            current.pop();
        }
    }

    fn phase_permutations(phases: &[isize]) -> Vec<Vec<isize>> {
        let mut buf = Vec::with_capacity(Self::factorial(phases.len()));
        Self::phase_permutations_go(
            &mut buf,
            &mut Vec::with_capacity(phases.len()),
            &Vec::from_iter(phases.iter().copied()),
        );
        buf
    }

    fn compute_max_output(&self, phases: &[isize], initial_input: isize) -> isize {
        let phase_combos = Self::phase_permutations(phases);
        let mut max_output = isize::MIN;

        for phase_combo in phase_combos {
            let output = self.eval(&phase_combo, initial_input);
            max_output = max_output.max(output);
        }

        max_output
    }
}

#[cfg(test)]
mod tests {
    use crate::{AmpChain, Computer};

    #[test]
    fn test_examples() {
        let program = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0";
        let chain = AmpChain::from(Computer::from(program));
        let thruster_signal = chain.eval(&[4, 3, 2, 1, 0], 0);
        assert_eq!(thruster_signal, 43210);
        let max_output = chain.compute_max_output(&[0, 1, 2, 3, 4], 0);
        assert_eq!(max_output, thruster_signal);

        let program = "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0";
        let chain = AmpChain::from(Computer::from(program));
        let thruster_signal = chain.eval(&[0, 1, 2, 3, 4], 0);
        assert_eq!(thruster_signal, 54321);
        let max_output = chain.compute_max_output(&[0, 1, 2, 3, 4], 0);
        assert_eq!(max_output, thruster_signal);

        let program = "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0";
        let chain = AmpChain::from(Computer::from(program));
        let thruster_signal = chain.eval(&[1, 0, 4, 3, 2], 0);
        assert_eq!(thruster_signal, 65210);
        let max_output = chain.compute_max_output(&[0, 1, 2, 3, 4], 0);
        assert_eq!(max_output, thruster_signal);

        let program =
            "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5";
        let chain = AmpChain::from(Computer::from(program));
        let thruster_signal = chain.eval(&[9, 8, 7, 6, 5], 0);
        assert_eq!(thruster_signal, 139629729);
        let max_output = chain.compute_max_output(&[5, 6, 7, 8, 9], 0);
        assert_eq!(max_output, thruster_signal);

        let program =
            "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10";
        let chain = AmpChain::from(Computer::from(program));
        let thruster_signal = chain.eval(&[9, 7, 8, 5, 6], 0);
        assert_eq!(thruster_signal, 18216);
        let max_output = chain.compute_max_output(&[5, 6, 7, 8, 9], 0);
        assert_eq!(max_output, thruster_signal);
    }

    #[test]
    fn test_input() {
        let program = std::fs::read_to_string("input/program.txt").unwrap();
        let chain = AmpChain::from(Computer::from(program.as_str()));
        let max_output = chain.compute_max_output(&[0, 1, 2, 3, 4], 0);
        assert_eq!(max_output, 24405);

        let max_output = chain.compute_max_output(&[5, 6, 7, 8, 9], 0);
        assert_eq!(max_output, 8271623);
    }
}
