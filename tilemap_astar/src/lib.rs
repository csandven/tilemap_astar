use crate::pos::Pos;

mod grid;
mod node;
mod pos;

pub trait PathSolver {
  fn solve(&self, start: Pos, target: Pos) -> Option<Vec<Pos>>;
}
