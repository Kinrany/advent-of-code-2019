use day_1_puzzle_1::fuel_for_module as naive_fuel_for_mass;

/// A quote from problem statement:
/// > Fuel itself requires fuel just like a module - take its mass,
/// > divide by three, round down, and subtract 2. However, that fuel
/// > **also** requires fuel, and **that** fuel requires fuel, and so on.
/// > Any mass that would require **negative fuel** should instead be treated
/// > as if it requires **zero fuel**; the remaining mass, if any,
/// > is instead handled by **wishing really hard**, which has no mass
/// > and is outside the scope of this calculation.
/// >
/// > So, for each module mass, calculate its fuel and add it to the total.
/// Then, treat the fuel amount you just calculated as the input mass and
/// repeat the process, continuing until a fuel requirement is zero or negative.
pub fn fuel_for_module(module_mass: i64) -> i64 {
  let mut total_fuel = 0;
  let mut added_mass = module_mass;
  while added_mass > 0 {
    added_mass = naive_fuel_for_mass(added_mass).max(0);
    total_fuel += added_mass;
  }
  total_fuel
}

#[cfg(test)]
mod tests {
  use super::fuel_for_module;

  #[test]
  fn returns_2_for_14() {
    assert_eq!(fuel_for_module(14), 2);
  }
  #[test]
  fn returns_966_for_1969() {
    assert_eq!(fuel_for_module(1969), 966);
  }

  #[test]
  fn returns_50346_for_100756() {
    assert_eq!(fuel_for_module(100756), 50346);
  }
}
