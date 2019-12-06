mod parse_input;
mod puzzle_1;
mod puzzle_2;

use anyhow::{anyhow, Result};
use parse_input::parse_input;
use puzzle_1::puzzle_1;
use puzzle_2::puzzle_2;
use shared::{input, print};

fn day_1(puzzle_number: usize, input: Vec<i64>) -> Result<i64> {
  match puzzle_number {
    1 => puzzle_1(input),
    2 => puzzle_2(input),
    x => Err(anyhow!("Puzzle number must be 1 or 2, not {:?}.", x)),
  }
}

fn main() {
  print((|| {
    let (puzzle_number, input_string) = input::puzzle_number_and_string()?;
    let input = parse_input(&input_string)?;
    day_1(puzzle_number, input)
  })());
}
