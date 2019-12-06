mod fuel_for_module;

use anyhow::Result;
pub use fuel_for_module::fuel_for_module;
use std::iter::IntoIterator;

pub fn puzzle_1(masses: Vec<i64>) -> Result<i64> {
  let fuel_for_modules = masses.into_iter().map(fuel_for_module).sum();
  Ok(fuel_for_modules)
}

#[cfg(test)]
mod tests {
  use super::puzzle_1;

  #[test]
  fn returns_658_for_12_14_1969() {
    const OUTPUT_658: i64 = 2 + 2 + 654;
    assert_eq!(puzzle_1(vec![12, 14, 1969]).unwrap(), OUTPUT_658);
  }
}
