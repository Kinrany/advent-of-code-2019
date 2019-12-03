mod parse_input;
mod program;
mod restore_program;
mod run_program;

use anyhow::Result;
use day_1_puzzle_1::get_filepath;
use parse_input::parse_input;
use program::Program;
use restore_program::restore_program;
use run_program::run_program;

fn day_2_puzzle_3() -> Result<usize> {
  let filepath = get_filepath()?;
  let input_string = std::fs::read_to_string(&filepath)?;
  let mut program = parse_input(&input_string)?;
  restore_program(&mut program);
  run_program(&mut program);
  Ok(program[0])
}

fn main() {
  match day_2_puzzle_3() {
    Ok(result) => println!("Answer: {:?}", result),
    Err(error) => {
      eprintln!("Error: {:?}", error);
      std::process::exit(1);
    }
  }
}
