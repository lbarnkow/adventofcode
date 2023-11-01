#![allow(dead_code)]

use std::{collections::VecDeque, fmt::Display, ops::Range};

fn main() {
    println!("Advent of Code 2019 - day 19");
}

const PARAMETER_MODE_FLAGS: usize = 3;

#[derive(Debug, Clone, Copy, PartialEq)]
enum ParameterMode {
    Position,
    Immediate,
    Relative,
}

impl From<isize> for ParameterMode {
    fn from(value: isize) -> Self {
        match value {
            0 => Self::Position,
            1 => Self::Immediate,
            2 => Self::Relative,
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
    AdjustRelBase,
    Halt,
}

impl Mnemonic {
    fn instruction_pointer_offset(&self) -> usize {
        match self {
            Mnemonic::Add | Mnemonic::Mul | Mnemonic::LessThan | Mnemonic::Equals => 4,
            Mnemonic::Input | Mnemonic::Output | Mnemonic::AdjustRelBase => 2,
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
            9 => Self::AdjustRelBase,
            99 => Self::Halt,
            x => panic!("Illegal mnemonic: {x}!"),
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

    fn get_paramter(
        computer: &mut Computer,
        parameter_mode: ParameterMode,
        offset: usize,
    ) -> isize {
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

    fn get_target_idx(
        computer: &mut Computer,
        parameter_mode: ParameterMode,
        offset: usize,
    ) -> usize {
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
    fn eval(&mut self, io: &mut InputOutput) {
        if self.state == State::Halted {
            panic!("Halted program can't be resumed!")
        }

        let mut opcode = OpCode::from(self.memory[self.instruction_pointer]);

        self.state = State::Running;
        while self.state == State::Running {
            opcode.eval(self, io);
            opcode = OpCode::from(self.memory[self.instruction_pointer]);
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum DroneState {
    Stationary,
    BeingPulled,
}

impl From<isize> for DroneState {
    fn from(value: isize) -> Self {
        match value {
            0 => Self::Stationary,
            1 => Self::BeingPulled,
            _ => panic!("Illegal DroneState value {value}!"),
        }
    }
}

impl Display for DroneState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            DroneState::Stationary => '.',
            DroneState::BeingPulled => '#',
        };
        write!(f, "{c}")
    }
}

fn binary_search_line(computer: &Computer, line: usize) -> Option<Range<usize>> {
    if line == 0 {
        return Some(0..1);
    }

    let max = line * 5;

    let Some(affected_x) = find_one_affected_spot_on_line(computer, line) else {
        return None;
    };

    let mut io = InputOutput::new(&[]);

    let left = if affected_x == 0 {
        0
    } else {
        binary_search_line_go(
            computer,
            &mut io,
            0,
            affected_x,
            line,
            DroneState::Stationary,
        )
    };
    let right = if affected_x == max {
        max
    } else {
        binary_search_line_go(
            computer,
            &mut io,
            affected_x,
            line * 5,
            line,
            DroneState::BeingPulled,
        )
    };

    Some(left..right)
}

fn binary_search_line_go(
    computer: &Computer,
    io: &mut InputOutput,
    lower: usize,
    upper: usize,
    line: usize,
    lower_state: DroneState,
) -> usize {
    let mut lower = lower;
    let mut upper = upper;

    while lower != upper {
        let mid = lower + ((upper - lower) / 2);

        io.in_q.push_back(mid as isize);
        io.in_q.push_back(line as isize);
        computer.clone().eval(io);
        let state = DroneState::from(io.out_q.pop_front().unwrap());
        if state == lower_state {
            if lower != mid {
                lower = mid;
            } else {
                lower = mid + 1;
            }
        } else {
            upper = mid;
        }
    }
    upper
}

fn find_one_affected_spot_on_line(computer: &Computer, line: usize) -> Option<usize> {
    let mut io = InputOutput::new(&[]);
    let max = line * 5;
    let mut ranges = VecDeque::new();
    ranges.push_back(0..max);

    while let Some(range) = ranges.pop_front() {
        let x = range.start + (range.len() / 2);
        io.in_q.push_back(x as isize);
        io.in_q.push_back(line as isize);
        computer.clone().eval(&mut io);
        match DroneState::from(io.out_q.pop_front().unwrap()) {
            DroneState::BeingPulled => return Some(x),
            DroneState::Stationary => {
                if x != range.start {
                    ranges.push_back(range.start..x)
                }
                if x + 1 != range.end {
                    ranges.push_back((x + 1)..range.end);
                }
            }
        }
    }

    None
}

fn compute_affected_cell_count(computer: &Computer, width: usize, height: usize) -> usize {
    (0..height)
        .filter_map(|line| {
            binary_search_line(computer, line).map(|range| {
                let end = range.end.min(width);
                end - range.start
            })
        })
        .sum()
}

const MAX_LINE: usize = 10000;

fn fit_santas_ship(computer: &Computer, ship_size: usize) -> (usize, usize) {
    let mut lower = 0;
    let mut upper = MAX_LINE;

    let (mut x, mut y) = (0, 0);

    while (upper - lower) > 1 {
        let mid = lower + (upper - lower) / 2;
        let Some(row_1) = binary_search_line(computer, mid) else {
            lower = mid;
            continue;
        };
        if row_1.len() < ship_size {
            lower = mid;
            continue;
        }
        let row_2 = binary_search_line(computer, mid + ship_size - 1).unwrap();
        if row_1.end - row_2.start < ship_size {
            lower = mid;
            continue;
        } else {
            (x, y) = (row_2.start, mid);
            upper = mid;
        }
    }

    (x, y)
}

#[cfg(test)]
mod tests {
    use crate::{compute_affected_cell_count, fit_santas_ship, Computer};

    #[test]
    fn test_examples() {}

    #[test]
    fn test_input() {
        let program = std::fs::read_to_string("input/program.txt").unwrap();
        let computer = Computer::from(program.as_str());

        let count = compute_affected_cell_count(&computer, 50, 50);
        assert_eq!(count, 183);
    }

    #[test]
    fn test_input_part2() {
        let program = std::fs::read_to_string("input/program.txt").unwrap();
        let computer = Computer::from(program.as_str());

        let (x, y) = fit_santas_ship(&computer, 100);
        let answer = x * 10000 + y;
        assert_eq!(answer, 11221248);
    }
}
