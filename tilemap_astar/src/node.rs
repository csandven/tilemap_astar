use crate::pos::Pos;
use std::ops::Deref;

#[derive(Clone, Debug)]
pub struct Node {
  pub id: u32,
  pub position: Pos,
  pub(crate) connections: Vec<u32>,
  pub cost: u32,
}

impl Node {
  pub fn new(id: u32, position: Pos) -> Node {
    Node {
      id,
      position,
      connections: vec![],
      cost: 0,
    }
  }
}
