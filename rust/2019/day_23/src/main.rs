#![allow(dead_code)]

use std::{
    collections::VecDeque,
    sync::mpsc::{self, Receiver, Sender},
    thread::{self},
    time::Duration,
};

fn main() {
    println!("Advent of Code 2019 - day 23");
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
            _ => Err(TryFromError {
                msg: format!("Illegal ParamterMode '{value}'!"),
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
            _ => Err(TryFromError {
                msg: format!("Illegal mnemonic: {value}!"),
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

#[derive(Debug, Clone, Copy)]
struct Packet {
    x: isize,
    y: isize,
}

impl Packet {
    const fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

#[derive(Debug)]
struct InputOutput {
    network_address: isize,
    net_tx: Vec<Sender<Packet>>,
    net_rx: Receiver<Packet>,
    buf_tx: VecDeque<isize>,
    buf_rx: VecDeque<isize>,
    nat: Sender<isize>,
}

impl InputOutput {
    fn new(
        network_address: isize,
        net_tx: Vec<Sender<Packet>>,
        net_rx: Receiver<Packet>,
        nat: Sender<isize>,
    ) -> Self {
        let mut s = Self {
            network_address,
            net_tx,
            net_rx,
            buf_tx: VecDeque::with_capacity(3),
            buf_rx: VecDeque::with_capacity(2),
            nat,
        };
        s.buf_rx.push_back(network_address);
        s
    }

    fn read_in(&mut self) -> Option<isize> {
        if !self.buf_rx.is_empty() {
            return self.buf_rx.pop_front();
        }

        match self.net_rx.recv_timeout(Duration::from_millis(1)) {
            Ok(packet) => {
                self.buf_rx.push_back(packet.x);
                self.buf_rx.push_back(packet.y);
                self.read_in()
            }
            Err(err) => match err {
                std::sync::mpsc::RecvTimeoutError::Timeout => Some(-1),
                std::sync::mpsc::RecvTimeoutError::Disconnected => None,
            },
        }
    }

    fn write_out(&mut self, data: isize) {
        self.buf_tx.push_back(data);

        if self.buf_tx.len() == 3 {
            let target = self.buf_tx.pop_front().unwrap();
            let packet = Packet::new(
                self.buf_tx.pop_front().unwrap(),
                self.buf_tx.pop_front().unwrap(),
            );

            let tx = &mut self.net_tx[target as usize];
            tx.send(packet).unwrap_or(()); // don't worry about receivers having hung up
            self.nat.send(1).unwrap_or(());
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum State {
    Running,
    WaitingForInput,
    Halted,
}

#[derive(Debug)]
struct Computer {
    state: State,
    memory: Vec<isize>,
    instruction_pointer: usize,
    relative_base: isize,
}

impl Computer {
    fn new(code: &str) -> Self {
        let mut memory: Vec<isize> = code.split(',').map(|s| s.parse().unwrap()).collect();
        memory.resize(1_000_000, 0);
        let instruction_pointer = 0;

        Self {
            state: State::Running,
            memory,
            instruction_pointer,
            relative_base: 0,
        }
    }

    fn eval(&mut self, io: &mut InputOutput) {
        if self.state == State::Halted {
            panic!("Halted program can't be resumed!")
        }

        let mut opcode = match OpCode::try_from(self.memory[self.instruction_pointer]) {
            Ok(opcode) => opcode,
            Err(e) => panic!("{}", e.msg),
        };

        self.state = State::Running;
        while self.state == State::Running {
            opcode.eval(self, io);
            opcode = match OpCode::try_from(self.memory[self.instruction_pointer]) {
                Ok(opcode) => opcode,
                Err(e) => panic!("{}", e.msg),
            };
        }
    }
}

fn prepare_channels(n: usize) -> (Vec<Sender<Packet>>, Vec<Option<Receiver<Packet>>>) {
    let (mut senders, mut receivers) = (Vec::with_capacity(n), Vec::with_capacity(n));
    for _ in 0..n {
        let (tx, rx) = mpsc::channel::<Packet>();
        senders.push(tx);
        receivers.push(Some(rx));
    }
    (senders, receivers)
}

fn prepare_threads(
    code: &str,
    n: usize,
    senders: &[Sender<Packet>],
    receivers: &mut [Option<Receiver<Packet>>],
    nat_tx: &Sender<isize>,
) {
    for (network_address, net_rx) in receivers.iter_mut().enumerate().filter(|(i, _)| *i < n) {
        let net_tx = senders.to_owned();
        let net_rx = net_rx.take().unwrap();
        let nat_tx = nat_tx.clone();
        let mut computer = Computer::new(code);
        let mut io = InputOutput::new(network_address as isize, net_tx, net_rx, nat_tx);
        thread::spawn(move || {
            computer.eval(&mut io);
        });
    }
    // for network_address in 0..n {
    //     let net_tx = senders.clone();
    //     let net_rx = receivers[network_address].take().unwrap();
    //     let nat_tx = nat_tx.clone();
    //     let mut computer = Computer::new(code);
    //     let mut io = InputOutput::new(network_address as isize, net_tx, net_rx, nat_tx);
    //     thread::spawn(move || {
    //         computer.eval(&mut io);
    //     });
    // }
}

fn simulate_networked_computers_part_1(code: &str) -> isize {
    let (nat_tx, _) = mpsc::channel::<isize>();

    let (senders, mut receivers) = prepare_channels(256);
    prepare_threads(code, 50, &senders, &mut receivers, &nat_tx);

    let rx_255 = receivers[255].take().unwrap();
    let packet = rx_255.recv().unwrap();
    packet.y
}

fn simulate_networked_computers_part_2(code: &str) -> isize {
    let (nat_tx, nat_rx) = mpsc::channel::<isize>();

    let (senders, mut receivers) = prepare_channels(256);
    prepare_threads(code, 50, &senders, &mut receivers, &nat_tx);

    let port_0 = senders[0].clone();
    let port_255 = receivers[255].take().unwrap();
    let mut nat_buffer = None;
    let mut last_sent_to_0: Option<Packet> = None;
    loop {
        // drain port 255
        while let Ok(packet) = port_255.recv_timeout(Duration::from_millis(1)) {
            nat_buffer = Some(packet);
        }

        // is network idle?
        if nat_rx.recv_timeout(Duration::from_millis(50)).is_ok() {
            continue;
        }

        if nat_buffer.unwrap().y
            == last_sent_to_0
                .unwrap_or_else(|| Packet::new(isize::MIN, isize::MIN))
                .y
        {
            return nat_buffer.unwrap().y;
        }

        last_sent_to_0 = nat_buffer;
        port_0.send(nat_buffer.unwrap()).unwrap_or(()); // ignore send errors
    }
}

#[cfg(test)]
mod tests {
    use crate::{simulate_networked_computers_part_1, simulate_networked_computers_part_2};

    #[test]
    fn test_examples() {}

    #[test]
    fn test_input() {
        let program = std::fs::read_to_string("input/program.txt").unwrap();
        let y = simulate_networked_computers_part_1(&program);

        assert_eq!(y, 24954);
    }

    #[test]
    fn test_input_part2() {
        let program = std::fs::read_to_string("input/program.txt").unwrap();
        let y = simulate_networked_computers_part_2(&program);

        assert_eq!(y, 17091);
    }
}
