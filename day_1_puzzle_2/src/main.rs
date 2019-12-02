mod fuel_for_module;
mod fuel_for_modules;

use anyhow::{anyhow, Result};
use day_1_puzzle_1::{get_filepath, parse_input};
use fuel_for_modules::fuel_for_modules;

fn day_1_puzzle_2() -> Result<i64> {
  let filepath = get_filepath()?;
  let input_string = std::fs::read_to_string(&filepath)
    .or_else(|e| Err(anyhow!("Failed to open input file {:?}. {:?}", filepath, e)))?;
  let input = parse_input(&input_string)?;
  let additional_fuel = fuel_for_modules(input);
  Ok(additional_fuel)
}

fn main() {
  match day_1_puzzle_2() {
    Ok(result) => println!("Answer: {:?}", result),
    Err(error) => {
      eprintln!("Error: {:?}", error);
      std::process::exit(1);
    }
  }
}
