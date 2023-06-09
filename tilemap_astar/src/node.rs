use crate::pos::Pos;

#[derive(Eq, Hash, PartialEq, Clone)]
pub(crate) struct HeuristicNode<'a> {
  pub parent: &'a Node,
  pub node: &'a Node,
  pub depth: u64,
  pub cost: u64,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Node {
  pub id: u32,
  pub position: Pos,
  pub(crate) connections: Vec<u32>,
  pub cost: u64,
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

  pub fn score(node_a: &Node, node_b: &Node) -> u64 {
    Pos::score(&node_a.position, &node_b.position) + node_a.cost + node_b.cost
  }
}
