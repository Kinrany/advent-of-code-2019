mod parse_input;
mod program;
mod restore_program;
mod run_program;

use anyhow::Result;
use parse_input::parse_input;
use program::Program;
use restore_program::restore_program;
use run_program::run_program;
use shared::run;

fn day_2_puzzle_3(input_string: String) -> Result<usize> {
  let mut program = parse_input(&input_string)?;
  restore_program(&mut program);
  run_program(&mut program);
  Ok(program[0])
}

fn main() {
  run(day_2_puzzle_3);
}
