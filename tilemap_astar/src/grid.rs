use crate::node::Node;
use crate::pos::Pos;
use std::slice::IterMut;

#[derive(Clone)]
pub struct Grid {
  size: (u32, u32),
  nodes: Vec<Node>,
}

impl Grid {
  pub fn new(size: (u32, u32)) -> Grid {
    let nodes = (0..size.1)
      .into_iter()
      .flat_map(|y| {
        (0..size.0).into_iter().map(move |x| {
          let node_id = (y * size.0) + x;
          Node::new(node_id, Pos(x, y))
        })
      })
      .collect();
    Grid {
      size: size.to_owned(),
      nodes,
    }
  }

  pub fn get_node(&self, pos: &Pos) -> &Node {
    assert!(pos < &self.size.into());

    &self.nodes[self.get_node_idx(pos.0, pos.1)]
  }

  pub fn set_connections(&mut self, node: &Node, connected_nodes: Vec<Node>) {
    self.nodes[node.id as usize] = Node {
      connections: connected_nodes.into_iter().map(|cn| cn.id).collect(),
      ..node.to_owned()
    }
  }

  pub fn iter(&self) -> impl Iterator<Item = &Node> {
    self.nodes.iter()
  }

  fn get_node_idx(&self, x: u32, y: u32) -> usize {
    ((y * self.size.0) + x) as usize
  }
}

#[cfg(test)]
mod tests {
  use crate::grid::Grid;
  use crate::node::Node;
  use crate::pos::Pos;

  impl From<Vec<Vec<&str>>> for Grid {
    fn from(value: Vec<Vec<&str>>) -> Self {
      let mut grid = Grid::new((3, 3));
      let nodes_connections = grid.clone()
        .iter()
        .for_each(|node| {
          let nps = node.position.neighbour_positions(false);

          let connections = nps
            .iter()
            .flat_map(|np| {
              value.get(np.1 as usize).map(|y| {
                y.get(np.0 as usize)
                  .map(|x| x == &".")
                  .and_then(|b| b.then(|| grid.get_node(np).to_owned()))
              })
            })
            .flatten()
            .collect();
          grid.set_connections(node, connections);
        });
      grid
    }
  }

  #[test]
  fn it_works() {
    let map = vec![vec![".", ".", "."], vec!["#", "#", "."], vec![".", ".", "."]];
    let grid: Grid = map.into();
  }
}
