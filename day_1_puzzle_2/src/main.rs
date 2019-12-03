mod fuel_for_module;
mod fuel_for_modules;

use anyhow::Result;
use day_1_puzzle_1::parse_input;
use fuel_for_modules::fuel_for_modules;
use shared::run;

fn day_1_puzzle_2(input_string: String) -> Result<i64> {
  let input = parse_input(&input_string)?;
  let additional_fuel = fuel_for_modules(input);
  Ok(additional_fuel)
}

fn main() {
  run(day_1_puzzle_2);
}
