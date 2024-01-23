#![allow(dead_code)]

use std::{collections::VecDeque, fmt::Display, io::Write};

fn main() {
    println!("Advent of Code 2019 - day 25");

    let program = std::fs::read_to_string("input/program.txt").unwrap();
    let mut computer = Computer::from(program.as_str());

    let mut io = InputOutput::new(&[]);

    let mut buf = String::new();

    loop {
        if let Err(e) = computer.eval(&mut io) {
            panic!("ERR: {}", e.msg);
        }
        if let Some(s) = io.read_from_out_q_ascii() {
            println!("ROBOT:\n{s}\n\n");
        }

        println!("Your options:");
        println!("- 'north', 'south', 'east' or 'west' to move that way.");
        println!("- 'take <name of item>' to pick stuff up.");
        println!("- 'drop <name of item>' to drop carried item.");
        println!("- 'inv' to review your carried items.");
        print!("\nChoice: ");
        std::io::stdout()
            .flush()
            .expect("stdout::flush() should not fail.");

        buf.clear();
        std::io::stdin()
            .read_line(&mut buf)
            .expect("Failed to read stdin!");

        println!("You wrote: {buf}");

        io.write_to_in_q_ascii(&buf);

        println!();
    }
}

struct TryFromError {
    msg: String,
}

const PARAMETER_MODE_FLAGS: usize = 3;

#[derive(Debug, Clone, Copy, PartialEq)]
enum ParameterMode {
    Position,
    Immediate,
    Relative,
}

impl TryFrom<isize> for ParameterMode {
    type Error = TryFromError;

    fn try_from(value: isize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Position),
            1 => Ok(Self::Immediate),
            2 => Ok(Self::Relative),
            x => Err(TryFromError {
                msg: format!("Illegal ParamterMode: {x}!"),
            }),
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
    AdjustRelBase,
    Halt,
}

impl Mnemonic {
    const fn instruction_pointer_offset(&self) -> usize {
        match self {
            Self::Add | Self::Mul | Self::LessThan | Self::Equals => 4,
            Self::Input | Self::Output | Self::AdjustRelBase => 2,
            Self::Halt => 0,
            Self::JumpIfTrue | Self::JumpIfFalse => 3,
        }
    }
}

impl TryFrom<isize> for Mnemonic {
    type Error = TryFromError;

    fn try_from(value: isize) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Add),
            2 => Ok(Self::Mul),
            3 => Ok(Self::Input),
            4 => Ok(Self::Output),
            5 => Ok(Self::JumpIfTrue),
            6 => Ok(Self::JumpIfFalse),
            7 => Ok(Self::LessThan),
            8 => Ok(Self::Equals),
            9 => Ok(Self::AdjustRelBase),
            99 => Ok(Self::Halt),
            x => Err(TryFromError {
                msg: format!("Illegal mnemonic: {x}!"),
            }),
        }
    }
}

#[derive(Debug)]
struct OpCode {
    mnemonic: Mnemonic,
    parameter_modes: [ParameterMode; PARAMETER_MODE_FLAGS],
}

impl OpCode {
    fn new(mnemonic: Mnemonic, parameter_modes: [ParameterMode; PARAMETER_MODE_FLAGS]) -> Self {
        match mnemonic {
            Mnemonic::Add | Mnemonic::Mul | Mnemonic::LessThan | Mnemonic::Equals => {
                assert_ne!(parameter_modes[2], ParameterMode::Immediate)
            }
            Mnemonic::Input => {
                assert_ne!(parameter_modes[0], ParameterMode::Immediate)
            }
            Mnemonic::Output
            | Mnemonic::JumpIfFalse
            | Mnemonic::JumpIfTrue
            | Mnemonic::Halt
            | Mnemonic::AdjustRelBase => (),
        }

        Self {
            mnemonic,
            parameter_modes,
        }
    }

    fn get_paramter(computer: &Computer, parameter_mode: ParameterMode, offset: usize) -> isize {
        let offset = computer.instruction_pointer + offset;
        let memory = &computer.memory;
        let parameter = memory[offset];
        match parameter_mode {
            ParameterMode::Position => memory[usize::try_from(parameter).unwrap()],
            ParameterMode::Immediate => parameter,
            ParameterMode::Relative => {
                let offset = usize::try_from(computer.relative_base + parameter).unwrap();
                memory[offset]
            }
        }
    }

    fn get_target_idx(computer: &Computer, parameter_mode: ParameterMode, offset: usize) -> usize {
        let offset = computer.instruction_pointer + offset;
        let memory = &computer.memory;
        let parameter = memory[offset];
        let p = match parameter_mode {
            ParameterMode::Position => parameter,
            ParameterMode::Relative => computer.relative_base + parameter,
            ParameterMode::Immediate => panic!("Target index cannot be in immediate mode!"),
        };

        usize::try_from(p).unwrap()
    }

    fn eval_add(&self, computer: &mut Computer) {
        let param_1 = Self::get_paramter(computer, self.parameter_modes[0], 1);
        let param_2 = Self::get_paramter(computer, self.parameter_modes[1], 2);
        let target_idx = Self::get_target_idx(computer, self.parameter_modes[2], 3);
        computer.memory[target_idx] = param_1 + param_2;
        computer.instruction_pointer += Mnemonic::Add.instruction_pointer_offset();
    }

    fn eval_mul(&self, computer: &mut Computer) {
        let param_1: isize = Self::get_paramter(computer, self.parameter_modes[0], 1);
        let param_2 = Self::get_paramter(computer, self.parameter_modes[1], 2);
        let target_idx = Self::get_target_idx(computer, self.parameter_modes[2], 3);
        computer.memory[target_idx] = param_1 * param_2;
        computer.instruction_pointer += Mnemonic::Mul.instruction_pointer_offset();
    }

    fn eval_input(&self, computer: &mut Computer, io: &mut InputOutput) {
        let target_idx = Self::get_target_idx(computer, self.parameter_modes[0], 1);

        match io.read_in() {
            None => computer.state = State::WaitingForInput,
            Some(next) => {
                computer.memory[target_idx] = next;
                computer.instruction_pointer += Mnemonic::Input.instruction_pointer_offset();
            }
        }
    }

    fn eval_output(&self, computer: &mut Computer, io: &mut InputOutput) {
        let param_1: isize = Self::get_paramter(computer, self.parameter_modes[0], 1);
        io.write_out(param_1);
        computer.instruction_pointer += Mnemonic::Output.instruction_pointer_offset();
    }

    fn eval_jump_if_true(&self, computer: &mut Computer) {
        let param_1 = Self::get_paramter(computer, self.parameter_modes[0], 1);
        let param_2 = Self::get_paramter(computer, self.parameter_modes[1], 2);

        if param_1 != 0 {
            computer.instruction_pointer = usize::try_from(param_2).unwrap();
        } else {
            computer.instruction_pointer += Mnemonic::JumpIfTrue.instruction_pointer_offset();
        }
    }

    fn eval_jump_if_false(&self, computer: &mut Computer) {
        let param_1 = Self::get_paramter(computer, self.parameter_modes[0], 1);
        let param_2 = Self::get_paramter(computer, self.parameter_modes[1], 2);

        if param_1 == 0 {
            computer.instruction_pointer = usize::try_from(param_2).unwrap();
        } else {
            computer.instruction_pointer += Mnemonic::JumpIfFalse.instruction_pointer_offset();
        }
    }

    fn eval_less_than(&self, computer: &mut Computer) {
        let param_1 = Self::get_paramter(computer, self.parameter_modes[0], 1);
        let param_2 = Self::get_paramter(computer, self.parameter_modes[1], 2);
        let target_idx = Self::get_target_idx(computer, self.parameter_modes[2], 3);
        computer.memory[target_idx] = if param_1 < param_2 { 1 } else { 0 };
        computer.instruction_pointer += Mnemonic::LessThan.instruction_pointer_offset();
    }

    fn eval_equals(&self, computer: &mut Computer) {
        let param_1 = Self::get_paramter(computer, self.parameter_modes[0], 1);
        let param_2 = Self::get_paramter(computer, self.parameter_modes[1], 2);
        let target_idx = Self::get_target_idx(computer, self.parameter_modes[2], 3);
        computer.memory[target_idx] = if param_1 == param_2 { 1 } else { 0 };
        computer.instruction_pointer += Mnemonic::Equals.instruction_pointer_offset();
    }

    fn eval_adjust_rel_base(&self, computer: &mut Computer) {
        let param_1 = Self::get_paramter(computer, self.parameter_modes[0], 1);
        computer.relative_base += param_1;
        computer.instruction_pointer += Mnemonic::AdjustRelBase.instruction_pointer_offset();
    }

    fn eval(&self, computer: &mut Computer, io: &mut InputOutput) {
        match self.mnemonic {
            Mnemonic::Add => self.eval_add(computer),
            Mnemonic::Mul => self.eval_mul(computer),
            Mnemonic::Input => self.eval_input(computer, io),
            Mnemonic::Output => self.eval_output(computer, io),
            Mnemonic::JumpIfTrue => self.eval_jump_if_true(computer),
            Mnemonic::JumpIfFalse => self.eval_jump_if_false(computer),
            Mnemonic::LessThan => self.eval_less_than(computer),
            Mnemonic::Equals => self.eval_equals(computer),
            Mnemonic::AdjustRelBase => self.eval_adjust_rel_base(computer),
            Mnemonic::Halt => computer.state = State::Halted,
        }
    }
}

impl TryFrom<isize> for OpCode {
    type Error = TryFromError;

    fn try_from(mut value: isize) -> Result<Self, Self::Error> {
        let mut parameter_modes = [ParameterMode::default(); PARAMETER_MODE_FLAGS];

        let mnemonic = (value % 100).try_into()?;
        value /= 100;
        for parameter_mode in &mut parameter_modes {
            *parameter_mode = (value % 10).try_into()?;
            value /= 10;
        }

        Ok(Self::new(mnemonic, parameter_modes))
    }
}

struct InputOutput {
    in_q: VecDeque<isize>,
    out_q: VecDeque<isize>,
}

impl Display for InputOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "IO:").unwrap();
        writeln!(f, "  i_q: {:?}", self.in_q).unwrap();
        writeln!(f, "  o_q: {:?}", self.out_q).unwrap();
        Ok(())
    }
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

    fn write_to_in_q(&mut self, i: isize) {
        self.in_q.push_back(i)
    }

    fn write_to_in_q_ascii(&mut self, data: &str) {
        for c in data.chars() {
            self.write_to_in_q(c as isize);
        }
    }

    fn read_from_out_q(&mut self) -> Option<isize> {
        self.out_q.pop_front()
    }

    fn read_from_out_q_ascii(&mut self) -> Option<String> {
        let mut s = String::new();

        while let Some(i) = self.out_q.pop_front() {
            if !(0..=255).contains(&i) {
                self.out_q.push_front(i);
                break;
            }
            s.push(char::from_u32(i as u32).unwrap());
        }

        if !s.is_empty() {
            Some(s)
        } else {
            None
        }
    }

    fn debug(&self) {
        for (name, q) in [("in_q", &self.in_q), ("out_q", &self.out_q)] {
            let mut sep = "";
            let mut ascii = false;
            println!("{name}:");
            for i in q {
                print!("{sep}");
                if *i >= 0 && *i <= 255 {
                    if !ascii {
                        print!("ASCII: ");
                        ascii = true;
                    }
                    print!("{}", char::from_u32(*i as u32).unwrap());
                    sep = "";
                } else {
                    print!("isize: {}", *i);
                    sep = "\n";
                }
            }
            print!("{sep}");
        }
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
    relative_base: isize,
}

impl Display for Computer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Computer:").unwrap();
        writeln!(f, "  State: {:?}", self.state).unwrap();
        writeln!(f, "  Instruction Pointer: {}", self.instruction_pointer).unwrap();
        let window_start = self.instruction_pointer;
        let window_end = window_start + 4;
        writeln!(
            f,
            "  Memory around ip: {:?}",
            &self.memory[window_start..window_end]
        )
        .unwrap();
        writeln!(f, "  Relative Base: {}", self.relative_base).unwrap();

        let window_start = usize::try_from((self.relative_base - 3).max(0)).unwrap();
        let window_end = window_start + 7;
        writeln!(
            f,
            "  Memory around rel: {:?}",
            &self.memory[window_start..window_end]
        )
        .unwrap();

        Ok(())
    }
}

impl From<&str> for Computer {
    fn from(value: &str) -> Self {
        let mut memory: Vec<isize> = value.split(',').map(|s| s.parse().unwrap()).collect();
        memory.resize(1_000_000, 0);
        let instruction_pointer = 0;

        Self {
            state: State::Running,
            memory,
            instruction_pointer,
            relative_base: 0,
        }
    }
}

impl Computer {
    fn eval(&mut self, io: &mut InputOutput) -> Result<(), TryFromError> {
        if self.state == State::Halted {
            panic!("Halted program can't be resumed!")
        }

        let mut opcode = OpCode::try_from(self.memory[self.instruction_pointer])?;

        self.state = State::Running;
        while self.state == State::Running {
            opcode.eval(self, io);
            opcode = OpCode::try_from(self.memory[self.instruction_pointer])?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_examples() {}

    #[test]
    fn test_input() {}
}
