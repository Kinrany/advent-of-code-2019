mod validation;

use anyhow::{anyhow, Result};
use shared::{get_first_arg, print};
use std::iter::Iterator;
use validation::{has_double_digit, has_exactly_double_digit, is_non_decreasing};

const MIN: u32 = 382345;
const MAX: u32 = 843167;

fn is_valid_7(password: &u32) -> bool {
  has_double_digit(password) && is_non_decreasing(password)
}

fn is_valid_8(password: &u32) -> bool {
  has_exactly_double_digit(password) && is_non_decreasing(password)
}

fn day_4(min: u32, max: u32) -> Result<u32> {
  let puzzle_number = get_first_arg()?.parse::<u32>()?;
  let is_valid = match puzzle_number {
    7 => is_valid_7,
    8 => is_valid_8,
    x => {
      return Err(anyhow!(
        "Expected puzzle number to be 7 or 8. Found {:?}.",
        x
      ))
    }
  };
  let count = (min..=max).filter(is_valid).count() as u32;
  Ok(count)
}

fn main() {
  print(day_4(MIN, MAX));
}
