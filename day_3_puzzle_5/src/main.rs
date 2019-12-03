mod parse_input;
mod wires;

use anyhow::Result;
use parse_input::parse_input;
use shared::run;
use std::collections::HashSet;
use std::iter::FromIterator;

fn day_3_puzzle_5(input_string: String) -> Result<u64> {
  let (wire1, wire2) = parse_input(&input_string)?;
  let locations1 = HashSet::<(i64, i64)>::from_iter(wire1.locations());
  let locations2 = HashSet::from_iter(wire2.locations());
  let intersections = locations1
    .intersection(&locations2)
    .filter(|&&l| l != (0, 0));
  let closest_intersection = intersections.map(|(x, y)| x.abs() + y.abs()).min().unwrap();
  Ok(closest_intersection as u64)
}

fn main() {
  run(day_3_puzzle_5);
}
