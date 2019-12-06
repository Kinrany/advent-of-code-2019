use super::{
  intersections,
  wires::{Wire, WireSegment, Wires},
};
use anyhow::Result;

fn is_between(a: i64, x: i64, b: i64) -> bool {
  (a <= x && x <= b) || (b <= x && x <= a)
}

fn dist(wire: &Wire, (target_x, target_y): (i64, i64)) -> u64 {
  let mut total_dist: u64 = 0;
  let (mut coords_x, mut coords_y): (i64, i64) = (0, 0);
  for WireSegment { dir, dist } in wire.0.iter() {
    let (dx, dy) = dir.to_dxdy();
    let new_coords_x = coords_x + dx * *dist as i64;
    let new_coords_y = coords_y + dy * *dist as i64;

    if target_x == coords_x && target_x == new_coords_x {
      if is_between(coords_y, target_y, new_coords_y) {
        return total_dist + (target_y - coords_y).abs() as u64;
      }
    }
    if target_y == coords_y && target_y == new_coords_y {
      if is_between(coords_x, target_x, new_coords_x) {
        return total_dist + (target_x - coords_x).abs() as u64;
      }
    }

    total_dist += dist;
    coords_x = new_coords_x;
    coords_y = new_coords_y;
  }
  total_dist
}

pub fn puzzle_6((wire1, wire2): Wires) -> Result<u64> {
  let intersections = intersections(&wire1, &wire2);
  let closest_intersection = intersections
    .into_iter()
    .map(|coords| dist(&wire1, coords) + dist(&wire2, coords))
    .min()
    .unwrap();
  Ok(closest_intersection)
}
