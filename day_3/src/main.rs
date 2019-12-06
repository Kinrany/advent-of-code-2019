mod intersections;
mod parse_input;
mod puzzle_5;
mod puzzle_6;
mod wires;

pub use intersections::intersections;
use parse_input::parse_input;
use puzzle_5::puzzle_5;
use puzzle_6::puzzle_6;
use wires::Wires;

use anyhow::{anyhow, Result};
use shared::{input, print};

fn day_3(puzzle_number: usize, input: Wires) -> Result<u64> {
  match puzzle_number {
    5 => puzzle_5(input),
    6 => puzzle_6(input),
    x => Err(anyhow!("Puzzle number must be 5 or 6, not {:?}.", x)),
  }
}

fn main() {
  print((|| {
    let (puzzle_number, input_string) = input::puzzle_number_and_string()?;
    let input = parse_input(&input_string)?;
    day_3(puzzle_number, input)
  })());
}
