use itertools::Itertools;
use std::iter;

/// A quote from problem statement:
/// > Two adjacent digits are the same (like `22` in `122345`)
pub fn has_double_digit(password: &u32) -> bool {
  password
    .to_string()
    .chars()
    .tuple_windows()
    .any(|(a, b)| a == b)
}

/// A quote from problem statement:
/// > Two adjacent digits are the same (like `22` in `122345`).
/// >
/// > The two adjacent matching digits are not part
/// > of a larger group of matching digits.
pub fn has_exactly_double_digit(password: &u32) -> bool {
  let pass_str = password.to_string();
  iter::once('_')
    .chain(pass_str.chars())
    .chain(iter::once('_'))
    .tuple_windows()
    .any(|(a, b, c, d)| a != b && b == c && c != d)
}

/// A quote from problem statement:
/// > Going from left to right, the digits never decrease; they only ever
/// > increase or stay the same (like `111123` or `135679`).
pub fn is_non_decreasing(password: &u32) -> bool {
  password
    .to_string()
    .chars()
    .tuple_windows()
    .all(|(a, b)| a <= b)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn pass_111111_has_double_digit() {
    assert_eq!(has_double_digit(&111111), true);
  }

  #[test]
  fn pass_223450_has_double_digit() {
    assert_eq!(has_double_digit(&223450), true);
  }

  #[test]
  fn pass_123789_does_not_have_double_digit() {
    assert_eq!(has_double_digit(&123789), false);
  }

  #[test]
  fn pass_111111_is_non_decreasing() {
    assert_eq!(is_non_decreasing(&111111), true);
  }

  #[test]
  fn pass_223450_is_not_non_decreasing() {
    assert_eq!(is_non_decreasing(&223450), false);
  }

  #[test]
  fn pass_123789_is_non_decreasing() {
    assert_eq!(is_non_decreasing(&123789), true);
  }

  #[test]
  fn pass_111111_has_exactly_double_digit() {
    assert_eq!(has_exactly_double_digit(&111111), false);
  }

  #[test]
  fn pass_223450_has_exactly_double_digit() {
    assert_eq!(has_exactly_double_digit(&223450), true);
  }

  #[test]
  fn pass_123789_has_exactly_double_digit() {
    assert_eq!(has_exactly_double_digit(&123789), false);
  }
}
