#![allow(dead_code)]

fn main() {
    println!("Advent of Code 2019 - day 05");
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

    fn eval_add(&self, memory: &mut [isize], instruction_pointer: usize) -> usize {
        let param_1 = Self::get_paramter(memory, self.parameter_modes[0], instruction_pointer + 1);
        let param_2 = Self::get_paramter(memory, self.parameter_modes[1], instruction_pointer + 2);
        let target_idx = usize::try_from(memory[instruction_pointer + 3]).unwrap();
        memory[target_idx] = param_1 + param_2;
        instruction_pointer + Mnemonic::Add.instruction_pointer_offset()
    }

    fn eval_mul(&self, memory: &mut [isize], instruction_pointer: usize) -> usize {
        let param_1: isize =
            Self::get_paramter(memory, self.parameter_modes[0], instruction_pointer + 1);
        let param_2 = Self::get_paramter(memory, self.parameter_modes[1], instruction_pointer + 2);
        let target_idx = usize::try_from(memory[instruction_pointer + 3]).unwrap();
        memory[target_idx] = param_1 * param_2;
        instruction_pointer + Mnemonic::Mul.instruction_pointer_offset()
    }

    fn eval_input(
        &self,
        memory: &mut [isize],
        instruction_pointer: usize,
        inputs: &'_ mut Inputs,
    ) -> usize {
        let target_idx = usize::try_from(memory[instruction_pointer + 1]).unwrap();
        memory[target_idx] = inputs.next();
        instruction_pointer + Mnemonic::Input.instruction_pointer_offset()
    }

    fn eval_output(
        &self,
        memory: &mut [isize],
        instruction_pointer: usize,
        outputs: &mut Vec<isize>,
    ) -> usize {
        let param_1: isize =
            Self::get_paramter(memory, self.parameter_modes[0], instruction_pointer + 1);
        outputs.push(param_1);
        instruction_pointer + Mnemonic::Output.instruction_pointer_offset()
    }

    fn eval_jump_if_true(&self, memory: &mut [isize], instruction_pointer: usize) -> usize {
        let param_1 = Self::get_paramter(memory, self.parameter_modes[0], instruction_pointer + 1);
        let param_2 = Self::get_paramter(memory, self.parameter_modes[1], instruction_pointer + 2);

        if param_1 != 0 {
            usize::try_from(param_2).unwrap()
        } else {
            instruction_pointer + Mnemonic::JumpIfTrue.instruction_pointer_offset()
        }
    }

    fn eval_jump_if_false(&self, memory: &mut [isize], instruction_pointer: usize) -> usize {
        let param_1 = Self::get_paramter(memory, self.parameter_modes[0], instruction_pointer + 1);
        let param_2 = Self::get_paramter(memory, self.parameter_modes[1], instruction_pointer + 2);

        if param_1 == 0 {
            usize::try_from(param_2).unwrap()
        } else {
            instruction_pointer + Mnemonic::JumpIfTrue.instruction_pointer_offset()
        }
    }

    fn eval_less_than(&self, memory: &mut [isize], instruction_pointer: usize) -> usize {
        let param_1 = Self::get_paramter(memory, self.parameter_modes[0], instruction_pointer + 1);
        let param_2 = Self::get_paramter(memory, self.parameter_modes[1], instruction_pointer + 2);
        let target_idx = usize::try_from(memory[instruction_pointer + 3]).unwrap();
        memory[target_idx] = if param_1 < param_2 { 1 } else { 0 };
        instruction_pointer + Mnemonic::LessThan.instruction_pointer_offset()
    }

    fn eval_equals(&self, memory: &mut [isize], instruction_pointer: usize) -> usize {
        let param_1 = Self::get_paramter(memory, self.parameter_modes[0], instruction_pointer + 1);
        let param_2 = Self::get_paramter(memory, self.parameter_modes[1], instruction_pointer + 2);
        let target_idx = usize::try_from(memory[instruction_pointer + 3]).unwrap();
        memory[target_idx] = if param_1 == param_2 { 1 } else { 0 };
        instruction_pointer + Mnemonic::Equals.instruction_pointer_offset()
    }

    fn eval(
        &self,
        memory: &mut [isize],
        instruction_pointer: usize,
        inputs: &'_ mut Inputs,
        outputs: &mut Vec<isize>,
    ) -> usize {
        match self.mnemonic {
            Mnemonic::Add => self.eval_add(memory, instruction_pointer),
            Mnemonic::Mul => self.eval_mul(memory, instruction_pointer),
            Mnemonic::Input => self.eval_input(memory, instruction_pointer, inputs),
            Mnemonic::Output => self.eval_output(memory, instruction_pointer, outputs),
            Mnemonic::JumpIfTrue => self.eval_jump_if_true(memory, instruction_pointer),
            Mnemonic::JumpIfFalse => self.eval_jump_if_false(memory, instruction_pointer),
            Mnemonic::LessThan => self.eval_less_than(memory, instruction_pointer),
            Mnemonic::Equals => self.eval_equals(memory, instruction_pointer),
            Mnemonic::Halt => instruction_pointer,
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

struct Inputs<'a> {
    data: &'a [isize],
}

impl<'a> Inputs<'a> {
    fn new(data: &'a [isize]) -> Self {
        Self { data }
    }

    fn next(&mut self) -> isize {
        if self.data.is_empty() {
            panic!("Input exhausted!");
        }
        let v = self.data[0];
        self.data = &self.data[1..];
        v
    }
}

struct Computer {
    memory: Vec<isize>,
    instruction_pointer: usize,
}

impl From<&str> for Computer {
    fn from(value: &str) -> Self {
        let memory = value.split(',').map(|s| s.parse().unwrap()).collect();
        let instruction_pointer = 0;

        Self {
            memory,
            instruction_pointer,
        }
    }
}

impl Computer {
    fn eval(&mut self, inputs: &[isize]) -> Vec<isize> {
        let mut inputs = Inputs::new(inputs);
        let mut outputs = Vec::new();

        let mut opcode = OpCode::from(self.memory[self.instruction_pointer]);

        while opcode.mnemonic != Mnemonic::Halt {
            self.instruction_pointer = opcode.eval(
                &mut self.memory,
                self.instruction_pointer,
                &mut inputs,
                &mut outputs,
            );
            opcode = OpCode::from(self.memory[self.instruction_pointer]);
        }

        outputs
    }
}

#[cfg(test)]
mod tests {
    use crate::Computer;

    #[test]
    fn test_examples() {
        let program = "1002,4,3,4,33";
        let mut computer = Computer::from(program);
        let output = computer.eval(&[]);
        assert_eq!(output, vec![]);
        assert_eq!(computer.memory[4], 99);
        assert_eq!(computer.instruction_pointer, 4);

        let program = "1101,100,-1,4,0";
        let mut computer = Computer::from(program);
        let output = computer.eval(&[]);
        assert_eq!(output, vec![]);
        assert_eq!(computer.memory[4], 99);
        assert_eq!(computer.instruction_pointer, 4);
    }

    #[test]
    fn test_examples_part2() {
        let program = "3,9,8,9,10,9,4,9,99,-1,8";
        let mut computer = Computer::from(program);
        let output = computer.eval(&[7]);
        assert_eq!(*output.last().unwrap(), 0);
        let mut computer = Computer::from(program);
        let output = computer.eval(&[8]);
        assert_eq!(*output.last().unwrap(), 1);
        let mut computer = Computer::from(program);
        let output = computer.eval(&[9]);
        assert_eq!(*output.last().unwrap(), 0);

        let program = "3,9,7,9,10,9,4,9,99,-1,8";
        let mut computer = Computer::from(program);
        let output = computer.eval(&[7]);
        assert_eq!(*output.last().unwrap(), 1);
        let mut computer = Computer::from(program);
        let output = computer.eval(&[8]);
        assert_eq!(*output.last().unwrap(), 0);
        let mut computer = Computer::from(program);
        let output = computer.eval(&[9]);
        assert_eq!(*output.last().unwrap(), 0);

        let program = "3,3,1108,-1,8,3,4,3,99";
        let mut computer = Computer::from(program);
        let output = computer.eval(&[7]);
        assert_eq!(*output.last().unwrap(), 0);
        let mut computer = Computer::from(program);
        let output = computer.eval(&[8]);
        assert_eq!(*output.last().unwrap(), 1);
        let mut computer = Computer::from(program);
        let output = computer.eval(&[9]);
        assert_eq!(*output.last().unwrap(), 0);

        let program = "3,3,1107,-1,8,3,4,3,99";
        let mut computer = Computer::from(program);
        let output = computer.eval(&[7]);
        assert_eq!(*output.last().unwrap(), 1);
        let mut computer = Computer::from(program);
        let output = computer.eval(&[8]);
        assert_eq!(*output.last().unwrap(), 0);
        let mut computer = Computer::from(program);
        let output = computer.eval(&[9]);
        assert_eq!(*output.last().unwrap(), 0);

        let program = "3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9";
        let mut computer = Computer::from(program);
        let output = computer.eval(&[0]);
        assert_eq!(*output.last().unwrap(), 0);
        let mut computer = Computer::from(program);
        let output = computer.eval(&[8]);
        assert_eq!(*output.last().unwrap(), 1);

        let program = "3,3,1105,-1,9,1101,0,0,12,4,12,99,1";
        let mut computer = Computer::from(program);
        let output = computer.eval(&[0]);
        assert_eq!(*output.last().unwrap(), 0);
        let mut computer = Computer::from(program);
        let output = computer.eval(&[8]);
        assert_eq!(*output.last().unwrap(), 1);

        let program = "\
            3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,\
            1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,\
            999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99\
        ";
        let mut computer = Computer::from(program);
        let output = computer.eval(&[6]);
        assert_eq!(*output.last().unwrap(), 999);
        let mut computer = Computer::from(program);
        let output = computer.eval(&[8]);
        assert_eq!(*output.last().unwrap(), 1000);
        let mut computer = Computer::from(program);
        let output = computer.eval(&[10]);
        assert_eq!(*output.last().unwrap(), 1001);
    }

    #[test]
    fn test_input() {
        let program = std::fs::read_to_string("input/program.txt").unwrap();
        let mut computer = Computer::from(program.as_str());
        let output = computer.eval(&[1]);
        assert_eq!(*output.last().unwrap(), 11049715);
    }

    #[test]
    fn test_input_part2() {
        let program = std::fs::read_to_string("input/program.txt").unwrap();
        let mut computer = Computer::from(program.as_str());
        let output = computer.eval(&[5]);
        assert_eq!(output.len(), 1);
        assert_eq!(*output.last().unwrap(), 2140710);
    }
}
