use super::fuel_for_module::fuel_for_module;
use std::iter::IntoIterator;

pub fn fuel_for_modules<T>(masses: T) -> i64
where
  T: IntoIterator<Item = i64>,
{
  masses.into_iter().map(fuel_for_module).sum()
}
