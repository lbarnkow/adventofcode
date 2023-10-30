#![allow(dead_code)]

use std::{
    collections::{HashSet, VecDeque},
    fmt::Display,
};

fn main() {
    println!("Advent of Code 2019 - day 17");
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
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn turn(&self, dir: Dir) -> Self {
        match (self, dir) {
            (Dir::Up, Dir::Left) => Self::Left,
            (Dir::Up, Dir::Right) => Self::Right,
            (Dir::Down, Dir::Left) => Self::Right,
            (Dir::Down, Dir::Right) => Self::Left,
            (Dir::Left, Dir::Left) => Self::Down,
            (Dir::Left, Dir::Right) => Self::Up,
            (Dir::Right, Dir::Left) => Self::Up,
            (Dir::Right, Dir::Right) => Self::Down,
            _ => panic!("Illegal turn direction!"),
        }
    }
}

impl Display for Dir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Dir::Up => '^',
            Dir::Down => 'v',
            Dir::Left => '<',
            Dir::Right => '>',
        };
        write!(f, "{c}")
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum CameraTile {
    Scaffold,
    Space,
    Bot(Dir),
    TumblingBot,
}

impl CameraTile {
    fn is_bot(&self) -> bool {
        matches!(self, CameraTile::Bot(_))
    }
}

impl From<char> for CameraTile {
    fn from(value: char) -> Self {
        match value {
            '#' => Self::Scaffold,
            '.' => Self::Space,
            '^' => Self::Bot(Dir::Up),
            'v' => Self::Bot(Dir::Down),
            '<' => Self::Bot(Dir::Left),
            '>' => Self::Bot(Dir::Right),
            'X' => Self::TumblingBot,
            _ => panic!("Not a valid map character: {value}"),
        }
    }
}

impl From<isize> for CameraTile {
    fn from(value: isize) -> Self {
        match char::from_u32(value as u32) {
            None => panic!("Not a valid unicode character: {value}"),
            Some(c) => c.into(),
        }
    }
}

impl Display for CameraTile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CameraTile::Scaffold => write!(f, "#"),
            CameraTile::Space => write!(f, "."),
            CameraTile::Bot(dir) => dir.fmt(f),
            CameraTile::TumblingBot => write!(f, "X"),
        }
    }
}

struct Map {
    width: usize,
    height: usize,
    tiles: Vec<CameraTile>,
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let intersection = self.identify_intersections();
        let mut i = 0;

        for y in 0..self.height {
            for x in 0..self.width {
                let tile = &self.tiles[y * self.width + x];
                if intersection.contains(&(x, y)) {
                    write!(f, "O").unwrap();
                } else {
                    write!(f, "{tile}").unwrap();
                }
                i += 1;
                if i == self.width {
                    i = 0;
                    writeln!(f).unwrap();
                }
            }
        }

        Ok(())
    }
}

impl Map {
    fn new(io: &mut InputOutput) -> Self {
        static NEWLINE: isize = '\n' as isize;
        let mut tiles = Vec::with_capacity(io.out_q.len());

        let mut consecutive_new_lines = 0;
        let mut new_lines = 0;
        while let Some(tile_char) = io.out_q.pop_front() {
            match tile_char {
                10 => {
                    new_lines += 1;
                    consecutive_new_lines += 1;
                }
                _ => {
                    consecutive_new_lines = 0;
                    tiles.push(tile_char.into());
                }
            }

            if consecutive_new_lines == 2 {
                break;
            }
        }

        let height = new_lines - 1;
        let width = tiles.len() / height;

        Self {
            width,
            height,
            tiles,
        }
    }

    fn identify_intersections(&self) -> Vec<(usize, usize)> {
        let mut intersections = Vec::new();

        for y in 1..(self.height - 1) {
            for x in 1..(self.width - 1) {
                if self.tiles[y * self.width + x] == CameraTile::Scaffold
                    && self.tiles[(y - 1) * self.width + x] == CameraTile::Scaffold
                    && self.tiles[(y + 1) * self.width + x] == CameraTile::Scaffold
                    && self.tiles[y * self.width + (x - 1)] == CameraTile::Scaffold
                    && self.tiles[y * self.width + (x + 1)] == CameraTile::Scaffold
                {
                    intersections.push((x, y));
                }
            }
        }

        intersections
    }

    fn alignment_paramter_for((intersection_x, intersection_y): (usize, usize)) -> usize {
        intersection_x * intersection_y
    }

    fn step(&self, (x, y): (usize, usize), dir: Dir) -> Option<(usize, usize)> {
        if x == 0 && dir == Dir::Left
            || y == 0 && dir == Dir::Up
            || x + 1 == self.width && dir == Dir::Right
            || y + 1 == self.height && dir == Dir::Down
        {
            return None;
        }

        let (x, y) = match dir {
            Dir::Up => (x, y - 1),
            Dir::Down => (x, y + 1),
            Dir::Left => (x - 1, y),
            Dir::Right => (x + 1, y),
        };

        match self.tiles[y * self.width + x] {
            CameraTile::Scaffold => Some((x, y)),
            CameraTile::Space => None,
            CameraTile::Bot(_) => None,
            CameraTile::TumblingBot => panic!("Bot already tumbled into space..."),
        }
    }

    fn simple_path(&self) -> Vec<Move> {
        // Most likely not a general solution, but both the example and the actual input allow simply following the path
        // all the way to the end, which yields a path that can be split in into 3 reusable segments.
        let mut moves = Vec::new();
        let mut seen = HashSet::new();
        let start_idx = self
            .tiles
            .iter()
            .enumerate()
            .filter(|(_, tile)| tile.is_bot())
            .map(|(idx, _)| idx)
            .next()
            .unwrap();
        let (mut x, mut y) = (start_idx % self.width, start_idx / self.width);
        seen.insert((x, y));
        let CameraTile::Bot(mut dir) = self.tiles[y * self.width + x] else {
            panic!()
        };

        let mut steps = 0;
        loop {
            if let Some((next_x, next_y)) = self.step((x, y), dir) {
                (x, y) = (next_x, next_y);
                steps += 1;
                seen.insert((x, y));
                continue;
            }

            let prev_dir = dir;
            for turn in [Dir::Left, Dir::Right] {
                let next_dir = dir.turn(turn);
                if self.step((x, y), next_dir).is_some() {
                    dir = next_dir;
                    if steps > 0 {
                        moves.push(Move::Forward(steps));
                        steps = 0;
                    }
                    moves.push(turn.into());
                    break;
                }
            }
            if prev_dir != dir {
                continue;
            }

            if steps > 0 {
                moves.push(Move::Forward(steps));
            }
            break;
        }

        let num_scaffold_tiles = self
            .tiles
            .iter()
            .filter(|tile| **tile == CameraTile::Scaffold)
            .count();
        assert_eq!(num_scaffold_tiles, seen.len() - 1);

        moves
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Move {
    Left,
    Right,
    Forward(usize),
}

impl From<Dir> for Move {
    fn from(value: Dir) -> Self {
        match value {
            Dir::Left => Self::Left,
            Dir::Right => Self::Right,
            _ => panic!("Can't transform {value:?} into Move!"),
        }
    }
}

trait MovementRoutine {
    fn extend_with_str(&mut self, s: &str);
}

impl MovementRoutine for VecDeque<isize> {
    fn extend_with_str(&mut self, s: &str) {
        for c in s.chars() {
            self.push_back(c as isize);
        }
    }
}

fn moves_list_to_ascii_str(moves: &[Move]) -> String {
    let mut s = String::new();
    let mut sep = "";

    for mv in moves {
        s.push_str(sep);
        match mv {
            Move::Left => s.push('L'),
            Move::Right => s.push('R'),
            Move::Forward(steps) => s.push_str(&steps.to_string()),
        }
        sep = ",";
    }
    s.push('\n');

    s
}

fn first_replacable_char(s: &str) -> usize {
    for (idx, c) in s.char_indices() {
        match c {
            'A' | 'B' | 'C' | ',' => continue,
            _ => return idx,
        }
    }
    panic!("No more replacable characters in {s}!");
}

fn len_of_largest_repeated_pattern(s: &str) -> usize {
    let mut max_len = 0;

    for len in 1..=20.max(s.len()) {
        let substring = &s[0..len];

        let mut iter = substring.chars().rev();
        if let Some(',') = iter.next() {
            if let Some(c) = iter.next() {
                if !c.is_ascii_digit() {
                    continue;
                }
            }
        } else {
            continue;
        }

        if s[len..].contains(substring) {
            max_len = len;
        } else {
            break;
        }
    }

    if max_len == 0 {
        panic!("No repeating pattern found in {s}!")
    }
    max_len
}

fn build_movement_routines(moves: &[Move]) -> [String; 4] {
    let mut moves = moves_list_to_ascii_str(moves);
    moves.pop();
    moves.push(',');

    let mut routines = [String::new(), String::new(), String::new(), String::new()];

    for (i, item) in routines.iter_mut().enumerate().skip(1) {
        let replacement = match i {
            1 => "A,",
            2 => "B,",
            3 => "C,",
            _ => panic!("Should never happen"),
        };

        let start = first_replacable_char(&moves);
        let len = len_of_largest_repeated_pattern(&moves[start..]);

        *item = moves[start..start + len].to_string();
        moves = moves.replace(item.as_str(), replacement);
    }
    routines[0] = moves;

    for r in routines.iter_mut() {
        r.pop();
        r.push('\n');
    }

    routines
}

#[cfg(test)]
mod tests {
    use crate::{
        build_movement_routines, moves_list_to_ascii_str, CameraTile, Computer, InputOutput, Map,
        MovementRoutine, State,
    };

    #[test]
    fn test_examples() {
        let tiles = "\
            ..#..........\n\
            ..#..........\n\
            #######...###\n\
            #.#...#...#.#\n\
            #############\n\
            ..#...#...#..\n\
            ..#####...^..\
        ";
        let tiles = tiles
            .lines()
            .flat_map(|line| line.chars())
            .map(CameraTile::from)
            .collect();
        let map = Map {
            width: 13,
            height: 7,
            tiles,
        };

        let expected = "\
            ..#..........\n\
            ..#..........\n\
            ##O####...###\n\
            #.#...#...#.#\n\
            ##O###O###O##\n\
            ..#...#...#..\n\
            ..#####...^..\n\
        ";
        assert_eq!(map.to_string(), expected);

        let intersections = map.identify_intersections();
        let alignment: Vec<usize> = intersections
            .iter()
            .map(|intersection| Map::alignment_paramter_for(*intersection))
            .collect();

        assert_eq!(intersections.len(), 4);
        assert_eq!(alignment.len(), 4);

        assert!(intersections.contains(&(2, 2)));
        assert!(intersections.contains(&(2, 4)));
        assert!(intersections.contains(&(6, 4)));
        assert!(intersections.contains(&(10, 4)));

        assert!(alignment.contains(&4));
        assert!(alignment.contains(&8));
        assert!(alignment.contains(&24));
        assert!(alignment.contains(&40));

        let sum = alignment.into_iter().sum::<usize>();
        assert_eq!(sum, 76);
    }

    #[test]
    fn test_examples_part2() {
        let tiles = "\
            #######...#####\n\
            #.....#...#...#\n\
            #.....#...#...#\n\
            ......#...#...#\n\
            ......#...###.#\n\
            ......#.....#.#\n\
            ^########...#.#\n\
            ......#.#...#.#\n\
            ......#########\n\
            ........#...#..\n\
            ....#########..\n\
            ....#...#......\n\
            ....#...#......\n\
            ....#...#......\n\
            ....#####......\
        ";
        let tiles = tiles
            .lines()
            .flat_map(str::chars)
            .map(CameraTile::from)
            .collect();
        let map = Map {
            width: 15,
            height: 15,
            tiles,
        };

        let path = map.simple_path();
        let path_str = moves_list_to_ascii_str(&path);
        let expected = "R,8,R,8,R,4,R,4,R,8,L,6,L,2,R,4,R,4,R,8,R,8,R,8,L,6,L,2\n";
        assert_eq!(path_str, expected);

        let routines = build_movement_routines(&path);
        assert_eq!(&routines[0], "A,B,C,B,A,C\n");
        assert_eq!(&routines[1], "R,8,R,8\n");
        assert_eq!(&routines[2], "R,4,R,4\n");
        assert_eq!(&routines[3], "R,8,L,6,L,2\n");
    }

    #[test]
    fn test_input() {
        let program = std::fs::read_to_string("input/program.txt").unwrap();
        let mut computer = Computer::from(program.as_str());
        let mut io = InputOutput::new(&[]);
        computer.eval(&mut io);

        let map = Map::new(&mut io);

        assert_eq!(map.width, 51);
        assert_eq!(map.height, 57);

        let intersections = map.identify_intersections();
        let alignment = intersections
            .iter()
            .map(|intersection| Map::alignment_paramter_for(*intersection))
            .sum::<usize>();
        assert_eq!(alignment, 5948);
    }

    #[test]
    fn test_input_part2() {
        let program = std::fs::read_to_string("input/program.txt").unwrap();
        let mut computer = Computer::from(program.as_str());
        computer.memory[0] = 2;
        let mut io = InputOutput::new(&[]);

        computer.eval(&mut io);
        assert_eq!(computer.state, State::WaitingForInput);

        let map = Map::new(&mut io);
        assert_eq!(map.width, 51);
        assert_eq!(map.height, 57);

        let query = in_q_to_ascii_str(&mut io);
        assert_eq!(query, "Main:\n");
        io.in_q.extend_with_str("B,C,B,A,C,A,C,A,B,A\n");
        computer.eval(&mut io);
        assert_eq!(computer.state, State::WaitingForInput);

        let query = in_q_to_ascii_str(&mut io);
        assert_eq!(query, "Function A:\n");
        io.in_q.extend_with_str("R,12,L,10,L,6,R,10\n");
        computer.eval(&mut io);
        assert_eq!(computer.state, State::WaitingForInput);

        let query = in_q_to_ascii_str(&mut io);
        assert_eq!(query, "Function B:\n");
        io.in_q.extend_with_str("R,12,L,6,R,12\n");
        computer.eval(&mut io);
        assert_eq!(computer.state, State::WaitingForInput);

        let query = in_q_to_ascii_str(&mut io);
        assert_eq!(query, "Function C:\n");
        io.in_q.extend_with_str("L,8,L,6,L,10\n");
        computer.eval(&mut io);
        assert_eq!(computer.state, State::WaitingForInput);

        let query = in_q_to_ascii_str(&mut io);
        assert_eq!(query, "Continuous video feed?\n");
        io.in_q.extend_with_str("n\n");
        computer.eval(&mut io);
        assert_eq!(computer.state, State::Halted);

        let _ = Map::new(&mut io);
        let dust = io.out_q.pop_back().unwrap();
        assert_eq!(dust, 997790);
    }

    fn in_q_to_ascii_str(io: &mut InputOutput) -> String {
        let mut s = String::with_capacity(io.out_q.len());

        while let Some(char_i) = io.out_q.pop_front() {
            s.push(char::from_u32(char_i as u32).unwrap());
        }

        s
    }
}
