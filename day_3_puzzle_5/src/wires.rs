use std::convert::TryFrom;

pub enum Dir {
  Left,
  Right,
  Up,
  Down,
}

impl Dir {
  pub fn to_dxdy(&self) -> (i64, i64) {
    match self {
      Self::Left => (-1, 0),
      Self::Right => (1, 0),
      Self::Up => (0, 1),
      Self::Down => (0, -1),
    }
  }
}

pub struct WireSegment {
  pub dir: Dir,
  pub dist: u64,
}

pub struct Wire(pub Vec<WireSegment>);

impl Wire {
  pub fn locations(&self) -> Vec<(i64, i64)> {
    let mut locations = vec![(0, 0)];
    for wire_segment in &self.0 {
      let dist = i64::try_from(wire_segment.dist).unwrap();
      let (x, y) = locations.last().unwrap().clone();
      let (dx, dy) = wire_segment.dir.to_dxdy();

      locations.extend((1..=dist).map(|d| (x + dx * d, y + dy * d)));
    }
    locations
  }
}

pub type Wires = (Wire, Wire);
