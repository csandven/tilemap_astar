use crate::node::{HeuristicNode, Node};
use crate::pos::Pos;

use queue::Queue;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Grid {
  size: (u32, u32),
  nodes: Vec<Node>,
}

impl Grid {
  pub fn new(size: (u32, u32)) -> Grid {
    let nodes = (0..size.1)
      .flat_map(|y| {
        (0..size.0).map(move |x| {
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

  pub fn update_node(&mut self, node: &Node) {
    self.nodes[node.id as usize] = node.to_owned();
  }

  pub fn iter(&self) -> impl Iterator<Item = &Node> {
    self.nodes.iter()
  }

  fn get_node_idx(&self, x: u32, y: u32) -> usize {
    ((y * self.size.0) + x) as usize
  }

  fn find_path(&self, start: Pos, target: Pos) -> Option<Vec<Pos>> {
    let mut solved_grid = vec![];
    let mut open_nodes = Queue::new();
    let mut working_set = HashMap::new();
    let start = self.get_node(&start);
    let finish = self.get_node(&target);
    let heuristic = Node::score(start, finish);
    let node = HeuristicNode {
      parent: start,
      node: start,
      depth: 0,
      cost: heuristic,
    };
    working_set.insert(start.position.to_owned(), start.to_owned());
    open_nodes.queue(node).unwrap();

    while let Some(current_node) = open_nodes.dequeue() {
      if current_node.node.position == finish.position {
        let mut current = current_node.parent;
        let mut parent = working_set.get(&current.position);
        while Some(current) != parent {
          solved_grid.push(current.position.to_owned());
          current = working_set.get(&current.position).unwrap();
          parent = working_set.get(&current.position);
        }
        solved_grid.reverse();
        solved_grid.push(finish.position.to_owned());
        return Some(solved_grid);
      } else {
        let n_pos = current_node
          .node
          .connections
          .iter()
          .flat_map(|c| self.nodes.iter().find(|n| &n.id == c))
          .collect::<Vec<&Node>>();
        for node in n_pos {
          if !working_set.contains_key(&node.position) {
            let score = Node::score(node, finish);
            let depth = current_node.depth + 1;
            let h_node = HeuristicNode {
              parent: current_node.node,
              node: self.get_node(&node.position),
              depth,
              cost: score,
            };
            working_set.insert(node.position.clone(), current_node.node.to_owned());
            open_nodes.queue(h_node).unwrap();
          }
        }
      }
    }

    None
  }
}

#[cfg(test)]
mod tests {
  use crate::grid::Grid;

  use crate::pos::Pos;

  impl From<Vec<Vec<&str>>> for Grid {
    fn from(value: Vec<Vec<&str>>) -> Self {
      let mut grid = Grid::new((3, 3));
      grid.clone().iter().for_each(|node| {
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
    let p = grid.find_path(Pos(0, 0), Pos(0, 2));
    assert!(p.is_some());
    assert_eq!(p.unwrap(), vec![Pos(1, 0), Pos(2, 0), Pos(2, 1), Pos(2, 2), Pos(1, 2), Pos(0, 2)]);
  }
}
