use anyhow::{anyhow, Result};
use shared::intcode;

trait OptionOrOk<T1> {
  fn or_ok<T2>(self, ok: T2) -> Result<T2, T1>;
}

impl<T1> OptionOrOk<T1> for Option<T1> {
  fn or_ok<T2>(self, ok: T2) -> Result<T2, T1> {
    match self {
      Some(x) => Err(x),
      None => Ok(ok),
    }
  }
}

pub fn puzzle_9(mut program: intcode::Program) -> Result<isize> {
  let output = intcode::run_with_input(&mut program, &mut vec![1].into_iter());
  let (&diagnostic_code, error_outputs) = output.split_last().ok_or(anyhow!("Empty output"))?;

  error_outputs
    .iter()
    .enumerate()
    .find(|(_i, &x)| x != 0)
    .map(|(i, &x)| anyhow!("Test {:?} returned {:?}", i, x))
    .or_ok(diagnostic_code)
}
