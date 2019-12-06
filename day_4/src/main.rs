mod is_valid;

use anyhow::Result;
use is_valid::is_valid;
use shared::print;
use std::iter::Iterator;

const MIN: u32 = 382345;
const MAX: u32 = 843167;

fn day_4_puzzle_7(min: u32, max: u32) -> Result<u32> {
  let count = (min..=max).filter(is_valid).count() as u32;
  Ok(count)
}

fn main() {
  // TODO: add a choice between puzzles 7 and 8
  print(day_4_puzzle_7(MIN, MAX));
}
