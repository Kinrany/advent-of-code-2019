use super::fuel_for_module::fuel_for_module;
use std::iter::IntoIterator;

pub fn fuel_for_modules<T>(masses: T) -> i64
where
  T: IntoIterator<Item = i64>,
{
  masses.into_iter().map(fuel_for_module).sum()
}

#[cfg(test)]
mod tests {
  use super::fuel_for_modules;

  #[test]
  fn returns_658_for_12_14_1969() {
    const OUTPUT_658: i64 = 2 + 2 + 654;
    assert_eq!(fuel_for_modules(vec![12, 14, 1969]), OUTPUT_658);
  }
}
