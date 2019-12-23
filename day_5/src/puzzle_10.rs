use anyhow::{anyhow, Result};
use shared::intcode;

pub fn puzzle_10(mut program: intcode::Program) -> Result<isize> {
  let input = vec![5];
  let output = intcode::run_with_input(&mut program, &mut input.into_iter());

  match output.len() {
    0 => Err(anyhow!("Output is empty")),
    1 => Ok(output[0]),
    x => Err(anyhow!("Output has {:?} elements instead of one", x)),
  }
}
