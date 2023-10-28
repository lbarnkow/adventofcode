#![allow(dead_code)]

use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Display,
};

fn main() {
    println!("Advent of Code 2019 - day 15");
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
enum Dir {
    North,
    South,
    West,
    East,
}

static DIRS: [Dir; 4] = [Dir::North, Dir::East, Dir::South, Dir::West];

impl From<Dir> for isize {
    fn from(value: Dir) -> Self {
        match value {
            Dir::North => 1,
            Dir::South => 2,
            Dir::West => 3,
            Dir::East => 4,
        }
    }
}

impl Dir {
    fn reverse(&self) -> Self {
        match self {
            Dir::North => Self::South,
            Dir::South => Self::North,
            Dir::West => Self::East,
            Dir::East => Self::West,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default, Eq, Hash)]
struct Pos {
    x: isize,
    y: isize,
}

impl From<Dir> for Pos {
    fn from(value: Dir) -> Self {
        match value {
            Dir::North => Self { x: 0, y: -1 },
            Dir::South => Self { x: 0, y: 1 },
            Dir::West => Self { x: -1, y: 0 },
            Dir::East => Self { x: 1, y: 0 },
        }
    }
}

impl std::ops::Add for Pos {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Pos {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn step(&self, dir: Dir) -> Self {
        *self + dir.into()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Reply {
    HitWall,
    MoveOk,
    MoveOkFoundOxygenSystem,
}

impl From<isize> for Reply {
    fn from(value: isize) -> Self {
        match value {
            0 => Self::HitWall,
            1 => Self::MoveOk,
            2 => Self::MoveOkFoundOxygenSystem,
            x => panic!("Illegal robot reply: {x}!"),
        }
    }
}

impl From<Reply> for Tile {
    fn from(value: Reply) -> Self {
        match value {
            Reply::HitWall => Self::Wall,
            Reply::MoveOk => Self::Free,
            Reply::MoveOkFoundOxygenSystem => Self::OxygenSystem,
        }
    }
}

fn map_sealed_section_go(
    computer: &mut Computer,
    io: &mut InputOutput,
    map: &mut HashMap<Pos, Tile>,
    pos: Pos,
    came_from: Option<Dir>,
) {
    for dir in DIRS {
        let next_pos = pos.step(dir);
        if map.contains_key(&next_pos) {
            continue;
        }

        io.in_q.push_back(dir.into());
        computer.eval(io);
        assert_eq!(computer.state, State::WaitingForInput);
        let reply = Reply::from(io.out_q.pop_front().unwrap());

        map.insert(next_pos, reply.into());
        if reply == Reply::HitWall {
            continue;
        }
        map_sealed_section_go(computer, io, map, next_pos, Some(dir.reverse()));
    }

    if let Some(came_from) = came_from {
        io.in_q.push_back(came_from.into());
        computer.eval(io);
        assert_eq!(computer.state, State::WaitingForInput);
        let reply = Reply::from(io.out_q.pop_front().unwrap());
        assert_ne!(reply, Reply::HitWall);
    }
}

fn map_sealed_section(program: &str) -> HashMap<Pos, Tile> {
    let mut map = HashMap::new();
    map.insert(Pos::default(), Tile::Free);

    map_sealed_section_go(
        &mut Computer::from(program),
        &mut InputOutput::new(&[]),
        &mut map,
        Pos::default(),
        None,
    );

    map
}

fn measure_shorted_path(map: &HashMap<Pos, Tile>, start: Pos) -> usize {
    let mut q = VecDeque::new();
    q.push_back((start, 0));
    let mut seen = HashSet::new();
    seen.insert(start);

    while let Some((pos, steps)) = q.pop_front() {
        for dir in DIRS {
            let next_pos = pos.step(dir);
            if seen.contains(&next_pos) {
                continue;
            } else {
                seen.insert(next_pos);
            }
            let tile = map.get(&next_pos).unwrap();

            match tile {
                Tile::Wall => continue,
                Tile::Free => q.push_back((next_pos, steps + 1)),
                Tile::OxygenSystem => return steps + 1,
            }
        }
    }

    panic!("No path found!");
}

fn measure_fill_time(map: &HashMap<Pos, Tile>) -> usize {
    let mut longest = usize::MIN;

    let start = map
        .iter()
        .filter(|(_, tile)| **tile == Tile::OxygenSystem)
        .map(|(p, _)| *p)
        .next()
        .unwrap();

    let mut q = VecDeque::new();
    q.push_back((start, 0));
    let mut seen = HashSet::new();
    seen.insert(start);

    while let Some((pos, steps)) = q.pop_front() {
        longest = longest.max(steps);

        for dir in DIRS {
            let next_pos = pos.step(dir);
            if seen.contains(&next_pos) {
                continue;
            } else {
                seen.insert(next_pos);
            }
            let tile = map.get(&next_pos).unwrap();

            match tile {
                Tile::Wall => continue,
                Tile::Free => q.push_back((next_pos, steps + 1)),
                Tile::OxygenSystem => panic!("Shouldn't visit this tile twice!"),
            }
        }
    }

    longest
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Wall,
    Free,
    OxygenSystem,
}

fn print_map(map: &HashMap<Pos, Tile>) {
    let (mut min_x, mut min_y, mut max_x, mut max_y) =
        (isize::MAX, isize::MAX, isize::MIN, isize::MIN);
    map.keys().for_each(|p| {
        min_x = min_x.min(p.x);
        min_y = min_y.min(p.y);
        max_x = max_x.max(p.x);
        max_y = max_y.max(p.y);
    });

    for y in (min_y - 1)..=(max_y + 1) {
        for x in (min_x - 1)..=(max_x + 1) {
            let p = Pos::new(x, y);
            if x == 0 && y == 0 {
                print!("O");
            } else if let Some(tile) = map.get(&p) {
                match tile {
                    Tile::Wall => print!("#"),
                    Tile::Free => print!(" "),
                    Tile::OxygenSystem => print!("X"),
                }
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use crate::{map_sealed_section, measure_fill_time, measure_shorted_path, print_map, Pos};

    #[test]
    fn test_examples() {}

    #[test]
    fn test_input() {
        let program = std::fs::read_to_string("input/program.txt").unwrap();
        let map = map_sealed_section(program.as_str());

        print_map(&map);

        let steps = measure_shorted_path(&map, Pos::default());
        assert_eq!(steps, 232);

        let time = measure_fill_time(&map);
        assert_eq!(time, 320);
    }
}
