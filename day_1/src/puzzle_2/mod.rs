mod fuel_for_module;

use anyhow::Result;
use fuel_for_module::fuel_for_module;
use std::iter::IntoIterator;

pub fn puzzle_2(masses: Vec<i64>) -> Result<i64> {
  let fuel_for_modules = masses.into_iter().map(fuel_for_module).sum();
  Ok(fuel_for_modules)
}
