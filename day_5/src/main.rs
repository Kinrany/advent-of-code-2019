mod puzzle_10;
mod puzzle_9;

use anyhow::{anyhow, Result};
use puzzle_10::puzzle_10;
use puzzle_9::puzzle_9;
use shared::{input, intcode, print};

fn day_5(puzzle_number: usize, program: intcode::Program) -> Result<isize> {
  match puzzle_number {
    9 => puzzle_9(program),
    10 => puzzle_10(program),
    x => Err(anyhow!("Puzzle number must be 9 or 10, not {:?}.", x)),
  }
}

fn main() {
  print((|| {
    let (puzzle_number, input_string) = input::puzzle_number_and_string()?;
    let program = intcode::parse(&input_string)?;
    day_5(puzzle_number, program)
  })());
}
