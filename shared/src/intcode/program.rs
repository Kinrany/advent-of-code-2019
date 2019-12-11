use std::{convert::TryFrom, iter::FromIterator};

#[derive(Clone, Debug, PartialEq)]
pub struct Program(Vec<isize>);

#[derive(Clone, Debug)]
pub enum Parameter {
  Position(usize),
  Immediate(isize),
}

impl From<(isize, isize)> for Parameter {
  fn from((value, mode): (isize, isize)) -> Self {
    match mode {
      0 => Parameter::Position(usize::try_from(value).expect("Pointers must be non-negative")),
      1 => Parameter::Immediate(value),
      x => panic!("Mode must be 0 or 1, found {:?}", x),
    }
  }
}

#[derive(Debug)]
pub enum Instruction {
  Halt,
  Add {
    in1: Parameter,
    in2: Parameter,
    out: Parameter,
  },
  Mul {
    in1: Parameter,
    in2: Parameter,
    out: Parameter,
  },
  Input {
    address: Parameter,
  },
  Output {
    address: Parameter,
  },
}

#[derive(Debug)]
pub struct InvalidInstruction(isize);

#[derive(Debug)]
pub enum ExecutionResult {
  Halt,
  Continue { ptr_offset: usize },
  Output { ptr_offset: usize, value: isize },
}

impl Program {
  pub fn legacy_set_input(&mut self, in1: isize, in2: isize) {
    self.0[1] = in1;
    self.0[2] = in2;
  }

  pub fn legacy_get_output(&self) -> isize {
    self.0[0]
  }

  pub fn get_value(&self, ptr: Parameter) -> isize {
    match ptr {
      Parameter::Position(x) => self.0[x],
      Parameter::Immediate(x) => x,
    }
  }

  pub fn set_value(&mut self, ptr: Parameter, value: isize) {
    let address = match ptr {
      Parameter::Position(x) => x,
      Parameter::Immediate(_) => panic!("Unexpected immediate mode parameter"),
    };
    self.0[address] = value;
  }

  pub fn get_op_at(&self, ptr: usize) -> Result<Instruction, InvalidInstruction> {
    use Instruction::*;

    let program = &self.0;

    let instruction = program[ptr];
    let opcode = instruction % 100;
    let mode = instruction / 100;

    match opcode {
      99 => Ok(Halt),
      1 => Ok(Add {
        in1: (program[ptr + 1], mode % 10).into(),
        in2: (program[ptr + 2], mode / 10 % 10).into(),
        out: (program[ptr + 3], mode / 100 % 10).into(),
      }),
      2 => Ok(Mul {
        in1: (program[ptr + 1], mode % 10).into(),
        in2: (program[ptr + 2], mode / 10 % 10).into(),
        out: (program[ptr + 3], mode / 100 % 10).into(),
      }),
      3 => Ok(Input {
        address: (program[ptr + 1], mode % 10).into(),
      }),
      4 => Ok(Output {
        address: (program[ptr + 1], mode % 10).into(),
      }),
      _ => Err(InvalidInstruction(instruction)),
    }
  }

  pub fn execute_op<T>(&mut self, op: Instruction, input: &mut T) -> ExecutionResult
  where
    T: Iterator<Item = isize>,
  {
    use Instruction::*;

    match op {
      Halt => ExecutionResult::Halt,
      Add { in1, in2, out } => {
        self.set_value(out, self.get_value(in1) + self.get_value(in2));
        ExecutionResult::Continue { ptr_offset: 4 }
      }
      Mul { in1, in2, out } => {
        self.set_value(out, self.get_value(in1) * self.get_value(in2));
        ExecutionResult::Continue { ptr_offset: 4 }
      }
      Input { address } => {
        self.set_value(address, input.next().unwrap());
        ExecutionResult::Continue { ptr_offset: 2 }
      }
      Output { address } => ExecutionResult::Output {
        ptr_offset: 2,
        value: self.get_value(address),
      },
    }
  }
}

impl FromIterator<isize> for Program {
  fn from_iter<T>(it: T) -> Self
  where
    T: IntoIterator<Item = isize>,
  {
    Program(it.into_iter().collect())
  }
}

impl From<Vec<isize>> for Program {
  fn from(vec: Vec<isize>) -> Self {
    Program(vec)
  }
}

impl Into<Vec<isize>> for Program {
  fn into(self) -> Vec<isize> {
    self.0
  }
}
