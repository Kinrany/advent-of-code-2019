use std::iter::FromIterator;

#[derive(Clone, Debug, PartialEq)]
pub struct Program(Vec<usize>);

#[derive(Debug)]
pub enum Operation {
  Halt,
  Add { in1: usize, in2: usize, out: usize },
  Mul { in1: usize, in2: usize, out: usize },
}

#[derive(Debug)]
pub struct InvalidOperation(usize);

#[derive(Debug)]
pub enum ExecutionResult {
  Halt,
  Continue { ptr_offset: usize },
}

impl Program {
  pub fn legacy_set_input(&mut self, in1: usize, in2: usize) {
    self.0[1] = in1;
    self.0[2] = in2;
  }

  pub fn legacy_get_output(&self) -> usize {
    self.0[0]
  }

  pub fn get_op_at(&self, ptr: usize) -> Result<Operation, InvalidOperation> {
    use Operation::*;

    let program = &self.0;

    match program[ptr] {
      99 => Ok(Halt),
      1 => Ok(Add {
        in1: program[ptr + 1],
        in2: program[ptr + 2],
        out: program[ptr + 3],
      }),
      2 => Ok(Mul {
        in1: program[ptr + 1],
        in2: program[ptr + 2],
        out: program[ptr + 3],
      }),
      opcode => Err(InvalidOperation(opcode)),
    }
  }

  pub fn execute_op(&mut self, op: Operation) -> ExecutionResult {
    use Operation::*;

    let program = &mut self.0;
    match op {
      Halt => ExecutionResult::Halt,
      Add { in1, in2, out } => {
        program[out] = program[in1] + &program[in2];
        ExecutionResult::Continue { ptr_offset: 4 }
      }
      Mul { in1, in2, out } => {
        program[out] = program[in1] * &program[in2];
        ExecutionResult::Continue { ptr_offset: 4 }
      }
    }
  }
}

impl FromIterator<usize> for Program {
  fn from_iter<T>(it: T) -> Self
  where
    T: IntoIterator<Item = usize>,
  {
    Program(it.into_iter().collect())
  }
}

impl From<Vec<usize>> for Program {
  fn from(vec: Vec<usize>) -> Self {
    Program(vec)
  }
}

impl Into<Vec<usize>> for Program {
  fn into(self) -> Vec<usize> {
    self.0
  }
}
