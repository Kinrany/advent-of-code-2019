use super::wires::{Dir, Wire, WireSegment, Wires};
use anyhow::{anyhow, Result as AnyhowResult};
use itertools::Itertools;

fn parse_wire(input: &str) -> AnyhowResult<Wire> {
  let wire_segments: Result<Vec<WireSegment>, _> = input
    .trim()
    .split(',')
    .map(|s| {
      let dir = match s.chars().nth(0).unwrap() {
        'R' => Dir::Right,
        'L' => Dir::Left,
        'U' => Dir::Up,
        'D' => Dir::Down,
        ch => return Err(anyhow!("Unknown direction: {:?}", ch)),
      };
      let dist = s[1..].parse::<u64>()?;
      Ok(WireSegment { dir, dist })
    })
    .collect();
  Ok(Wire(wire_segments?))
}

pub fn parse_input(input: &str) -> AnyhowResult<Wires> {
  let (line1, line2) = input
    .trim()
    .lines()
    .next_tuple()
    .expect("input to have at least two lines");
  let (wire1, wire2) = (parse_wire(line1)?, parse_wire(line2)?);
  Ok((wire1, wire2))
}
