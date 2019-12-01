mod fuel_for_module;
mod fuel_for_modules;
mod parse_input;

use anyhow::{anyhow, Result};
use fuel_for_modules::fuel_for_modules;
use parse_input::parse_input;

fn day_1_puzzle_1() -> Result<i64> {
  let filepath = std::env::args()
    .skip(1)
    .next()
    .ok_or(anyhow!("Input file path not specified"))?;
  let input_string = std::fs::read_to_string(&filepath)
    .or_else(|e| Err(anyhow!("Failed to open input file {:?}. {:?}", filepath, e)))?;
  let input = parse_input(&input_string)?;
  let output = fuel_for_modules(input);
  Ok(output)
}

fn main() {
  match day_1_puzzle_1() {
    Ok(result) => println!("Answer: {:?}", result),
    Err(error) => {
      eprintln!("Error: {:?}", error);
      std::process::exit(1);
    }
  }
}
