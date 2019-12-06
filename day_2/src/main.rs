mod puzzle_3;
mod puzzle_4;

use anyhow::{anyhow, Result};
use puzzle_3::puzzle_3;
use puzzle_4::puzzle_4;
use shared::{input, intcode, print};

fn day_2(puzzle_number: usize, input: intcode::Program) -> Result<usize> {
  match puzzle_number {
    3 => puzzle_3(input),
    4 => puzzle_4(input),
    x => Err(anyhow!("Puzzle number must be 3 or 4, not {:?}.", x)),
  }
}

fn main() {
  print((|| {
    let (puzzle_number, input_string) = input::puzzle_number_and_string()?;
    let program = intcode::parse(&input_string)?;
    day_2(puzzle_number, program)
  })());
}
