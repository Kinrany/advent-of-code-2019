use anyhow::Result;
use shared::print;

const MIN: u32 = 382345;
const MAX: u32 = 843167;

fn is_valid(password: u32) -> bool {
  let pass_str = password.to_string();
  let mut prev_digit = pass_str.chars().nth(0).unwrap().to_digit(10).unwrap();
  let mut double = false;
  for ch in pass_str.chars().skip(1) {
    let digit = ch.to_digit(10).unwrap();
    if prev_digit == digit {
      double = true;
    } else if prev_digit > digit {
      return false;
    }
    prev_digit = digit
  }

  // valid if has a double digit and no decreasing digits
  double
}

fn day_4_puzzle_7(min: u32, max: u32) -> Result<u32> {
  let mut count = 0;
  for password in min..=max {
    if is_valid(password) {
      count += 1;
    }
  }
  Ok(count)
}

fn main() {
  // TODO: add a choice between puzzles 7 and 8
  print(day_4_puzzle_7(MIN, MAX));
}
