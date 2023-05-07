use std::ops::Add;

#[derive(Clone, Eq, PartialEq, Debug, PartialOrd, Hash)]
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
        if x >= 0 && y >= 0 {
          Some(Pos(x as u32, y as u32))
        } else {
          None
        }
      })
      .collect::<Vec<Pos>>()
  }

  pub fn score(pos_a: &Pos, pos_b: &Pos) -> u64 {
    let delta_x = pos_a.0 as f64 - pos_b.0 as f64;
    let delta_y = pos_a.1 as f64 - pos_b.1 as f64;
    let euclidean_distance = (delta_x.powi(2) + delta_y.powi(2)).sqrt() as u64;
    euclidean_distance + 1
  }
}

impl From<(u32, u32)> for Pos {
  fn from(value: (u32, u32)) -> Self {
    Pos(value.0, value.1)
  }
}

#[cfg(test)]
mod tests {
  use crate::pos::Pos;

  #[test]
  fn get_neighbours_non_diagonal() {
    let pos = Pos(0, 0);
    assert_eq!(pos.neighbour_positions(false), vec![Pos(1, 0), Pos(0, 1)]);

    let pos = Pos(1, 1);
    assert_eq!(pos.neighbour_positions(false), vec![Pos(2, 1), Pos(1, 2), Pos(0, 1), Pos(1, 0)]);
  }
}
