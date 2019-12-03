mod fuel_for_module;
mod fuel_for_modules;
mod parse_input;

use anyhow::Result;
use fuel_for_modules::fuel_for_modules;
use parse_input::parse_input;
use shared::run;

fn day_1_puzzle_1(input_string: String) -> Result<i64> {
  let input = parse_input(&input_string)?;
  let output = fuel_for_modules(input);
  Ok(output)
}

fn main() {
  run(day_1_puzzle_1);
}
