mod run_program_with_input;

use anyhow::{anyhow, Result};
use itertools::iproduct;
use run_program_with_input::run_program_with_input;
use shared::intcode;

const EXPECTED_OUTPUT: isize = 19690720;

pub fn puzzle_4(program: intcode::Program) -> Result<isize> {
  for (input1, input2) in iproduct!(1..100, 1..100) {
    let result = run_program_with_input(&program, input1, input2);
    if result == EXPECTED_OUTPUT {
      return Ok(input1 * 100 + input2);
    }
  }

  Err(anyhow!("Failed to find an answer"))
}
