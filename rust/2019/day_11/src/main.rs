#![allow(dead_code)]

use std::{
    collections::{HashMap, VecDeque},
    fmt::Display,
};

fn main() {
    println!("Advent of Code 2019 - day 11");
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
enum Color {
    Black,
    White,
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Color::Black => write!(f, "."),
            Color::White => write!(f, "#"),
        }
    }
}

impl From<isize> for Color {
    fn from(value: isize) -> Self {
        match value {
            0 => Self::Black,
            1 => Self::White,
            x => panic!("Illegal color value: {x}!"),
        }
    }
}

impl From<Color> for isize {
    fn from(value: Color) -> Self {
        match value {
            Color::Black => 0,
            Color::White => 1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Turn {
    Left,
    Right,
}

impl From<isize> for Turn {
    fn from(value: isize) -> Self {
        match value {
            0 => Self::Left,
            1 => Self::Right,
            x => panic!("Illegal turn action: {x}!"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn turn(&self, turn: Turn) -> Self {
        match (self, turn) {
            (Dir::Up, Turn::Left) => Self::Left,
            (Dir::Up, Turn::Right) => Self::Right,
            (Dir::Down, Turn::Left) => Self::Right,
            (Dir::Down, Turn::Right) => Self::Left,
            (Dir::Left, Turn::Left) => Self::Down,
            (Dir::Left, Turn::Right) => Self::Up,
            (Dir::Right, Turn::Left) => Self::Up,
            (Dir::Right, Turn::Right) => Self::Down,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
struct Pos {
    x: isize,
    y: isize,
}

impl Pos {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn step(&mut self, dir: Dir) {
        match dir {
            Dir::Up => self.y -= 1,
            Dir::Down => self.y += 1,
            Dir::Left => self.x -= 1,
            Dir::Right => self.x += 1,
        }
    }
}

fn paint(computer: &mut Computer, start_color: Color) -> HashMap<Pos, Color> {
    let mut tiles: HashMap<Pos, Color> = HashMap::new();
    let mut io = InputOutput::new(&[]);
    let mut pos = Pos::default();
    let mut dir = Dir::Up;

    tiles.insert(pos, start_color);
    loop {
        let current_tile = *tiles.get(&pos).unwrap_or(&Color::Black);
        io.in_q.push_back(current_tile.into());
        computer.eval(&mut io);
        if computer.state == State::Halted {
            break;
        }
        let (color, turn): (Color, Turn) = (
            io.out_q.pop_front().unwrap().into(),
            io.out_q.pop_front().unwrap().into(),
        );
        tiles.insert(pos, color);
        dir = dir.turn(turn);
        pos.step(dir);
    }

    tiles
}

fn render(tiles: &HashMap<Pos, Color>) -> String {
    let (mut min_x, mut min_y, mut max_x, mut max_y) = (0, 0, 0, 0);
    tiles.keys().for_each(|p| {
        min_x = min_x.min(p.x);
        min_y = min_y.min(p.y);
        max_x = max_x.max(p.x);
        max_y = max_y.max(p.y);
    });

    let mut buf = String::with_capacity(
        usize::try_from((max_x - min_x + 1 + 1) * (max_y - min_y + 1)).unwrap(),
    );
    let mut sep = "";
    for y in min_y..=max_y {
        buf.push_str(sep);
        for x in min_x..=max_x {
            let p = Pos::new(x, y);
            match tiles.get(&p) {
                Some(color) => buf.push_str(&color.to_string()),
                None => buf.push_str(&Color::Black.to_string()),
            }
        }
        sep = "\n";
    }

    buf
}

#[cfg(test)]
mod tests {
    use crate::{paint, render, Color, Computer};

    #[test]
    fn test_examples() {}

    #[test]
    fn test_input() {
        let program = std::fs::read_to_string("input/program.txt").unwrap();
        let mut computer = Computer::from(program.as_str());

        let tiles_painted = paint(&mut computer, Color::Black);
        assert_eq!(tiles_painted.len(), 1909);

        let program = std::fs::read_to_string("input/program.txt").unwrap();
        let mut computer = Computer::from(program.as_str());
        let tiles_painted = paint(&mut computer, Color::White);

        let registration_id = render(&tiles_painted);
        let expected = "\
            ...##.#..#.####.####.#..#.#..#.###..#..#...\n\
            ....#.#..#.#....#....#.#..#..#.#..#.#..#...\n\
            ....#.#..#.###..###..##...####.#..#.####...\n\
            ....#.#..#.#....#....#.#..#..#.###..#..#...\n\
            .#..#.#..#.#....#....#.#..#..#.#....#..#...\n\
            ..##...##..#....####.#..#.#..#.#....#..#...\
        ";
        assert_eq!(registration_id, expected);
    }
}
