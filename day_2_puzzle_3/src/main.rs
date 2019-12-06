mod restore_program;

use anyhow::Result;
use restore_program::restore_program;
use shared::{intcode, run};

fn day_2_puzzle_3(input_string: String) -> Result<usize> {
  let mut program = intcode::parse(&input_string)?;
  restore_program(&mut program);
  intcode::run(&mut program);
  Ok(program[0])
}

fn main() {
  run(day_2_puzzle_3);
}
