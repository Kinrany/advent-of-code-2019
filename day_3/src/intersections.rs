use super::wires::Wire;
use std::collections::HashSet;
use std::iter::FromIterator;

pub fn intersections(wire1: &Wire, wire2: &Wire) -> Vec<(i64, i64)> {
  let locations1 = HashSet::<(i64, i64)>::from_iter(wire1.locations());
  let locations2 = HashSet::from_iter(wire2.locations());
  let intersections = locations1
    .intersection(&locations2)
    .filter(|&&l| l != (0, 0))
    .map(<_>::clone)
    .collect();
  intersections
}
