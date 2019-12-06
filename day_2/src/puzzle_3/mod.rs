mod restore_program;

use anyhow::Result;
use restore_program::restore_program;
use shared::intcode;

pub fn puzzle_3(mut program: intcode::Program) -> Result<usize> {
  restore_program(&mut program);
  intcode::run(&mut program);
  Ok(program[0])
}
