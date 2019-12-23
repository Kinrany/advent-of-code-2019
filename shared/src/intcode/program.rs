use anyhow::{anyhow, Result};
use std::{convert::TryFrom, iter::FromIterator};

#[derive(Clone, Debug, PartialEq)]
pub struct Program(Vec<isize>);

#[derive(Clone, Debug)]
pub enum Parameter {
  Position(usize),
  Immediate(isize),
}

impl Parameter {
  pub fn as_ptr(self) -> Result<usize> {
    match self {
      Parameter::Position(x) => Ok(x),
      Parameter::Immediate(_) => Err(anyhow!("Immediate mode cannot be used for pointers")),
    }
  }
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
  JumpIfTrue {
    in1: Parameter,
    address: Parameter,
  },
  JumpIfFalse {
    in1: Parameter,
    address: Parameter,
  },
  LessThan {
    in1: Parameter,
    in2: Parameter,
    out: Parameter,
  },
  Equals {
    in1: Parameter,
    in2: Parameter,
    out: Parameter,
  },
}

#[derive(Debug)]
pub struct InvalidInstruction(isize);

pub enum ExecutionResult {
  Halt,
  Continue {
    ptr_set: Box<dyn Fn(usize) -> usize>,
  },
  Output {
    ptr_set: Box<dyn Fn(usize) -> usize>,
    value: isize,
  },
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

  pub fn set_value(&mut self, ptr: usize, value: isize) {
    self.0[ptr] = value;
  }

  pub fn get_op_at(&self, ptr: usize) -> Result<Instruction, InvalidInstruction> {
    use Instruction::*;

    let program = &self.0;

    let instruction = program[ptr];
    let opcode = instruction % 100;
    let mode = instruction / 100;

    let instruction = match opcode {
      99 => Halt,
      1 => Add {
        in1: (program[ptr + 1], mode % 10).into(),
        in2: (program[ptr + 2], mode / 10 % 10).into(),
        out: (program[ptr + 3], mode / 100 % 10).into(),
      },
      2 => Mul {
        in1: (program[ptr + 1], mode % 10).into(),
        in2: (program[ptr + 2], mode / 10 % 10).into(),
        out: (program[ptr + 3], mode / 100 % 10).into(),
      },
      3 => Input {
        address: (program[ptr + 1], mode % 10).into(),
      },
      4 => Output {
        address: (program[ptr + 1], mode % 10).into(),
      },
      5 => JumpIfTrue {
        in1: (program[ptr + 1], mode % 10).into(),
        address: (program[ptr + 2], mode / 10 % 10).into(),
      },
      6 => JumpIfFalse {
        in1: (program[ptr + 1], mode % 10).into(),
        address: (program[ptr + 2], mode / 10 % 10).into(),
      },
      7 => LessThan {
        in1: (program[ptr + 1], mode % 10).into(),
        in2: (program[ptr + 2], mode / 10 % 10).into(),
        out: (program[ptr + 3], mode / 100 % 10).into(),
      },
      8 => Equals {
        in1: (program[ptr + 1], mode % 10).into(),
        in2: (program[ptr + 2], mode / 10 % 10).into(),
        out: (program[ptr + 3], mode / 100 % 10).into(),
      },
      _ => Err(InvalidInstruction(instruction))?,
    };

    Ok(instruction)
  }

  pub fn execute_op<T>(&mut self, op: Instruction, input: &mut T) -> ExecutionResult
  where
    T: Iterator<Item = isize>,
  {
    use Instruction::*;

    match op {
      Halt => ExecutionResult::Halt,
      Add { in1, in2, out } => {
        let in1 = self.get_value(in1);
        let in2 = self.get_value(in2);
        let out = out.as_ptr().unwrap();

        self.set_value(out, in1 + in2);

        ExecutionResult::Continue {
          ptr_set: Box::new(|ptr| ptr + 4),
        }
      }
      Mul { in1, in2, out } => {
        let in1 = self.get_value(in1);
        let in2 = self.get_value(in2);
        let out = out.as_ptr().unwrap();

        self.set_value(out, in1 * in2);

        ExecutionResult::Continue {
          ptr_set: Box::new(|ptr| ptr + 4),
        }
      }
      Input { address } => {
        let input = input.next().unwrap();
        let address = address.as_ptr().unwrap();

        self.set_value(address, input);

        ExecutionResult::Continue {
          ptr_set: Box::new(|ptr| ptr + 2),
        }
      }
      Output { address } => ExecutionResult::Output {
        ptr_set: Box::new(|ptr| ptr + 2),
        value: self.get_value(address),
      },
      JumpIfTrue { in1, address } => {
        let in1 = self.get_value(in1);
        let address = self.get_value(address) as usize;

        ExecutionResult::Continue {
          ptr_set: Box::new(move |ptr| if in1 != 0 { address } else { ptr + 3 }),
        }
      }
      JumpIfFalse { in1, address } => {
        let in1 = self.get_value(in1);
        let address = self.get_value(address) as usize;

        ExecutionResult::Continue {
          ptr_set: Box::new(move |ptr| if in1 == 0 { address } else { ptr + 3 }),
        }
      }
      LessThan { in1, in2, out } => {
        let in1 = self.get_value(in1);
        let in2 = self.get_value(in2);
        let out = out.as_ptr().unwrap();

        self.set_value(out, if in1 < in2 { 1 } else { 0 });

        ExecutionResult::Continue {
          ptr_set: Box::new(|ptr| ptr + 4),
        }
      }
      Equals { in1, in2, out } => {
        let in1 = self.get_value(in1);
        let in2 = self.get_value(in2);
        let out = out.as_ptr().unwrap();

        self.set_value(out, if in1 == in2 { 1 } else { 0 });

        ExecutionResult::Continue {
          ptr_set: Box::new(|ptr| ptr + 4),
        }
      }
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
