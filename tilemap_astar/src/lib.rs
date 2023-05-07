use crate::pos::Pos;

pub mod grid;
pub mod node;
pub mod pos;

pub trait PathSolver {
  fn solve(&self, start: Pos, target: Pos) -> Option<Vec<Pos>>;
}

pub mod prelude {
  pub use crate::grid::*;
  pub use crate::node::*;
  pub use crate::pos::*;
}