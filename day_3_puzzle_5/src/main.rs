mod intersections;
mod parse_input;
mod wires;

use anyhow::Result;
use intersections::intersections;
use parse_input::parse_input;
use shared::run;
use std::convert::TryInto;

fn dist((x, y): (i64, i64)) -> u64 {
  (x.abs() + y.abs()).try_into().unwrap()
}

fn day_3_puzzle_5(input_string: String) -> Result<u64> {
  let (wire1, wire2) = parse_input(&input_string)?;
  let intersections = intersections(&wire1, &wire2);
  let closest_intersection = intersections.into_iter().map(dist).min().unwrap();
  Ok(closest_intersection as u64)
}

fn main() {
  run(day_3_puzzle_5);
}
