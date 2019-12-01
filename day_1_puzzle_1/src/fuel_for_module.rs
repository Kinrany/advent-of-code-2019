/// A quote from problem statement:
/// > Fuel required to launch a given module is based on its mass.
/// > Specifically, to find the fuel required for a module, take its mass,
/// > divide by three, round down, and subtract 2.
pub fn fuel_for_module(module_mass: i64) -> i64 {
  module_mass / 3 - 2
}

#[cfg(test)]
mod tests {
  use super::fuel_for_module;

  #[test]
  fn returns_2_for_12() {
    assert_eq!(fuel_for_module(12), 2);
  }

  #[test]
  fn returns_2_for_14() {
    assert_eq!(fuel_for_module(14), 2);
  }

  #[test]
  fn returns_654_for_1969() {
    assert_eq!(fuel_for_module(1969), 654);
  }

  #[test]
  fn returns_33583_for_12() {
    assert_eq!(fuel_for_module(100756), 33583);
  }
}
