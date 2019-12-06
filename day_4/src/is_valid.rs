use itertools::Itertools;

pub fn has_double_digit(password: &u32) -> bool {
  password
    .to_string()
    .chars()
    .tuple_windows()
    .any(|(a, b)| a == b)
}

pub fn is_non_decreasing(password: &u32) -> bool {
  password
    .to_string()
    .chars()
    .tuple_windows()
    .all(|(a, b)| a <= b)
}

pub fn is_valid(password: &u32) -> bool {
  has_double_digit(password) && is_non_decreasing(password)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn pass_111111_is_valid() {
    assert_eq!(is_valid(&111111), true);
  }

  #[test]
  fn pass_223450_is_not_valid() {
    assert_eq!(is_valid(&223450), false);
  }

  #[test]
  fn pass_123789_is_not_valid() {
    assert_eq!(is_valid(&123789), false);
  }
}
