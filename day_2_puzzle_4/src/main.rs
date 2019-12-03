mod run_program_with_input;

use anyhow::{anyhow, Result};
use day_2_puzzle_3::parse_input;
use itertools::iproduct;
use run_program_with_input::run_program_with_input;
use shared::run;

const EXPECTED_OUTPUT: usize = 19690720;

fn day_2_puzzle_4(input_string: String) -> Result<usize> {
  let program = parse_input(&input_string)?;

  for (input1, input2) in iproduct!(1..100, 1..100) {
    let result = run_program_with_input(&program, input1, input2);
    if result == EXPECTED_OUTPUT {
      return Ok(input1 * 100 + input2);
    }
  }

  Err(anyhow!("Failed to find an answer"))
}

fn main() {
  run(day_2_puzzle_4);
}
