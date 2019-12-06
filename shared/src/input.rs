use anyhow::{anyhow, Result};
use std::{env::args, fs::read_to_string};

fn get_nth_arg(n: usize) -> Result<String> {
  args()
    .skip(1)
    .nth(n)
    .ok_or(anyhow!("Input file path not specified"))
}

pub fn puzzle_number() -> Result<usize> {
  Ok(get_nth_arg(0)?.parse::<usize>()?)
}

pub fn string() -> Result<String> {
  Ok(read_to_string(get_nth_arg(0)?)?)
}

pub fn puzzle_number_and_string() -> Result<(usize, String)> {
  let puzzle_number = get_nth_arg(0)?.parse::<usize>()?;
  let input = read_to_string(get_nth_arg(1)?)?;
  Ok((puzzle_number, input))
}
