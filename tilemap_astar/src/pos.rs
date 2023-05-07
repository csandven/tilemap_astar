use std::ops::Add;

#[derive(Clone, Eq, PartialEq, Debug, PartialOrd)]
pub struct Pos(pub u32, pub u32);

impl Add for Pos {
  type Output = Pos;

  fn add(self, rhs: Self) -> Self::Output {
    Pos(self.0 + rhs.0, self.1 + rhs.1)
  }
}

impl Pos {
  pub fn neighbour_positions(&self, allow_diagonal: bool) -> Vec<Pos> {
    let mut offsets: Vec<(i32, i32)> = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];

    if allow_diagonal {
      offsets.append(&mut vec![(-1, -1), (1, 1), (-1, 1), (1, -1)]);
    }

    offsets
      .into_iter()
      .flat_map(|offset| {
        let x = self.0 as i32 + offset.0;
        let y = self.1 as i32 + offset.1;
        if x > 0 && y > 0 {
          Some(Pos(x as u32, y as u32))
        } else {
          None
        }
      })
      .collect::<Vec<Pos>>()
  }
}

impl From<(u32, u32)> for Pos {
  fn from(value: (u32, u32)) -> Self {
    Pos(value.0, value.1)
  }
}
