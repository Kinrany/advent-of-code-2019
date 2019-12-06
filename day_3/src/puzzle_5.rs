use super::{intersections, wires::Wires};
use anyhow::Result;
use std::convert::TryInto;

fn dist((x, y): (i64, i64)) -> u64 {
  (x.abs() + y.abs()).try_into().unwrap()
}

pub fn puzzle_5((wire1, wire2): Wires) -> Result<u64> {
  let intersections = intersections(&wire1, &wire2);
  let closest_intersection = intersections.into_iter().map(dist).min().unwrap();
  Ok(closest_intersection as u64)
}
