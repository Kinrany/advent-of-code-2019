mod fuel_for_module;
mod fuel_for_modules;
mod get_filepath;
mod parse_input;

use anyhow::Result;
use fuel_for_modules::fuel_for_modules;
use get_filepath::get_filepath;
use parse_input::parse_input;

fn day_1_puzzle_1() -> Result<i64> {
  let filepath = get_filepath()?;
  let input_string = std::fs::read_to_string(&filepath)?;
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
