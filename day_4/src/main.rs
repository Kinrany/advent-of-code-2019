mod validation;

use anyhow::{anyhow, Result};
use shared::{input, print};
use std::iter::Iterator;
use validation::{has_double_digit, has_exactly_double_digit, is_non_decreasing};

fn count_valid_passwords(is_valid: fn(&u32) -> bool) -> Result<u32> {
  Ok((MIN..=MAX).filter(is_valid).count() as u32)
}

fn puzzle_7() -> Result<u32> {
  count_valid_passwords(|password| has_double_digit(password) && is_non_decreasing(password))
}

fn puzzle_8() -> Result<u32> {
  count_valid_passwords(|password| {
    has_exactly_double_digit(password) && is_non_decreasing(password)
  })
}

const MIN: u32 = 382345;
const MAX: u32 = 843167;

fn day_4(puzzle_number: usize) -> Result<u32> {
  match puzzle_number {
    7 => puzzle_7(),
    8 => puzzle_8(),
    x => Err(anyhow!("Puzzle number must be 7 or 8, not {:?}.", x)),
  }
}

fn main() {
  print((|| {
    let puzzle_number = input::puzzle_number()?;
    day_4(puzzle_number)
  })());
}
