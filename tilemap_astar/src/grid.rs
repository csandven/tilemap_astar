use crate::node::{HeuristicNode, Node, NodeConnection};
use crate::pos::Pos;

use std::collections::{HashMap, VecDeque};

#[derive(Clone)]
pub struct CachedPath {
  start_idx: usize,
  target_idx: usize,
  path: Vec<Pos>,
}

#[derive(Clone)]
pub struct CachedGrid {
  pub grid: Grid,
  pub cache_size: usize,
  pub cache: VecDeque<CachedPath>,
}

impl CachedGrid {
  pub fn new(grid: Grid, cache_size: usize) -> CachedGrid {
    CachedGrid {
      grid,
      cache_size,
      cache: VecDeque::new(),
    }
  }

  pub fn find_path(&mut self, start: Pos, target: Pos) -> Option<Vec<Pos>> {
    let s_idx = self.grid.get_node_idx(start.0, start.1);
    let t_idx = self.grid.get_node_idx(target.0, target.1);
    if let Some(cached_path) = self
      .cache
      .iter()
      .find(|p| p.start_idx == s_idx && p.target_idx == t_idx)
    {
      Some(cached_path.path.clone())
    } else if let Some(path) = self.grid.find_path(start, target) {
      self.put_cache(&(s_idx, t_idx), path.to_owned());
      Some(path)
    } else {
      None
    }
  }

  pub fn remove_connections_for_node(&mut self, node: &Node) {
    self.grid.remove_connections_for_node(node);
    let valid_cached_paths = self
      .cache
      .clone()
      .into_iter()
      .filter(|cp| !cp.path.contains(&node.position))
      .collect();
    self.cache = valid_cached_paths;
  }

  pub fn put_cache(&mut self, key: &(usize, usize), value: Vec<Pos>) {
    if self.cache.len() == self.cache_size {
      self.cache.pop_front();
    }
    self.cache.push_back(CachedPath {
      start_idx: key.0,
      target_idx: key.1,
      path: value,
    })
  }
}

#[derive(Clone)]
pub struct Grid {
  size: (u32, u32),
  nodes: Vec<Node>,
  connections: Vec<NodeConnection>,
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
      connections: vec![],
    }
  }

  pub fn fully_connected(size: (u32, u32)) -> Grid {
    let mut new_grid = Grid::new(size);
    new_grid.clone().iter().for_each(|node| {
      let connections = node
        .position
        .neighbour_positions(false)
        .iter()
        .flat_map(|n_pos| new_grid.get_node(n_pos).to_owned())
        .map(|n| n.to_owned())
        .collect::<Vec<Node>>();
      new_grid.set_connections(node, connections);
    });
    new_grid
  }

  pub fn not_out_of_bounds(&self, pos: &Pos) -> bool {
    pos < &self.size.into()
  }

  pub fn get_node(&self, pos: &Pos) -> Option<&Node> {
    self.nodes.iter().find(|n| &n.position == pos)
  }

  pub fn node_at(&self, pos: &Pos) -> &Node {
    assert!(pos < &self.size.into());
    &self.nodes[self.get_node_idx(pos.0, pos.1)]
  }

  pub fn set_connections(&mut self, node: &Node, connected_nodes: Vec<Node>) {
    let mut connections = connected_nodes
      .iter()
      .map(|cn| NodeConnection(node.id, cn.id))
      .filter(|nc| !self.connections.contains(nc))
      .collect::<Vec<NodeConnection>>();
    self.connections.append(&mut connections);
  }

  pub fn remove_connections_for_node(&mut self, node: &Node) {
    let updated_connections = self
      .connections
      .clone()
      .into_iter()
      .filter(|nc| !(nc.0 != node.id || nc.1 != node.id))
      .collect();
    self.connections = updated_connections;
  }

  pub fn get_connections(&self, node: &Node) -> Vec<&NodeConnection> {
    self
      .connections
      .iter()
      .filter(|con| con.0 == node.id || con.1 == node.id)
      .collect()
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
    let mut open_nodes = VecDeque::new();
    let mut working_set = HashMap::new();
    let start = self.node_at(&start);
    let finish = self.node_at(&target);
    let heuristic = Node::score(start, finish);
    let node = HeuristicNode {
      parent: start,
      node: start,
      depth: 0,
      cost: heuristic,
    };
    working_set.insert(start.position.to_owned(), start.to_owned());
    open_nodes.push_back(node);

    while let Some(current_node) = open_nodes.pop_back() {
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
        let connected_neighbours = self
          .get_connections(current_node.node)
          .iter()
          .flat_map(|NodeConnection(c1, c2)| self.nodes.iter().filter(|n| n.id == *c1 || n.id == *c2))
          .filter(|node| node != &current_node.node)
          .collect::<Vec<&Node>>();

        for node in connected_neighbours {
          if !working_set.contains_key(&node.position) {
            let score = Node::score(node, finish);
            let depth = current_node.depth + 1;
            let h_node = HeuristicNode {
              parent: current_node.node,
              node: self.node_at(&node.position),
              depth,
              cost: score,
            };
            working_set.insert(node.position.clone(), current_node.node.to_owned());
            open_nodes.push_back(h_node);
          }
        }
      }
    }

    None
  }
}

#[cfg(test)]
mod tests {
  use crate::grid::{CachedGrid, Grid};
  use crate::node::NodeConnection;

  use crate::pos::Pos;

  impl From<Vec<&str>> for Grid {
    fn from(value: Vec<&str>) -> Self {
      let mut grid = Grid::new((3, 3));
      grid
        .clone()
        .iter()
        .filter(|n| {
          let idx = ((n.position.1 * 3) + n.position.0) as usize;
          value.get(idx) == Some(&".")
        })
        .for_each(|node| {
          let nps = node.position.neighbour_positions(false);
          let connections = nps
            .iter()
            .flat_map(|np| {
              let idx = ((np.1 * 3) + np.0) as usize;
              match value.get(idx) {
                Some(v) if v == &"." => grid.get_node(np),
                _ => None,
              }
            })
            .map(|n| n.to_owned())
            .collect();
          grid.set_connections(node, connections);
        });
      grid
    }
  }

  #[test]
  fn it_works_fully_connected() {
    let grid: Grid = Grid::fully_connected((3, 3));
    assert_eq!(
      grid.connections,
      vec![
        NodeConnection(0, 1),
        NodeConnection(0, 3),
        NodeConnection(1, 2),
        NodeConnection(1, 4),
        NodeConnection(2, 5),
        NodeConnection(3, 4),
        NodeConnection(3, 6),
        NodeConnection(4, 5),
        NodeConnection(4, 7),
        NodeConnection(5, 8),
        NodeConnection(6, 7),
        NodeConnection(7, 8)
      ]
    )
  }

  #[test]
  fn it_works_without_cache() {
    let map = vec![".", ".", ".", "#", "#", ".", ".", ".", "."];
    let grid: Grid = map.into();
    let p = grid.find_path(Pos(0, 0), Pos(0, 2));
    assert!(p.is_some());
    assert_eq!(p.unwrap(), vec![Pos(1, 0), Pos(2, 0), Pos(2, 1), Pos(2, 2), Pos(1, 2), Pos(0, 2)]);
  }

  #[test]
  fn it_works_with_cache() {
    let map = vec![".", ".", ".", "#", "#", ".", ".", ".", "."];
    let grid: Grid = map.into();
    let mut cached_grid = CachedGrid::new(grid, 10);
    let p = cached_grid.find_path(Pos(0, 0), Pos(0, 2));
    assert!(p.is_some());
    let target_path = vec![Pos(1, 0), Pos(2, 0), Pos(2, 1), Pos(2, 2), Pos(1, 2), Pos(0, 2)];
    assert_eq!(p.unwrap(), target_path);
    assert_eq!(cached_grid.cache.get(0).map(|d| d.path.to_owned()), Some(target_path))
  }

  #[test]
  fn cache_limit_is_handled() {
    let map = vec![".", ".", ".", "#", "#", ".", ".", ".", "."];
    let grid: Grid = map.into();
    let mut cached_grid = CachedGrid::new(grid, 1);
    let p1 = cached_grid.find_path(Pos(0, 0), Pos(0, 2));
    assert!(p1.is_some());
    let first_target_path = vec![Pos(1, 0), Pos(2, 0), Pos(2, 1), Pos(2, 2), Pos(1, 2), Pos(0, 2)];
    assert_eq!(p1.unwrap(), first_target_path);
    assert_eq!(cached_grid.cache.get(0).map(|d| d.path.to_owned()), Some(first_target_path));

    let p2 = cached_grid.find_path(Pos(1, 0), Pos(2, 1));
    assert!(p2.is_some());
    let second_target_path = vec![Pos(2, 0), Pos(2, 1)];
    assert_eq!(p2.unwrap(), second_target_path);
    assert_eq!(cached_grid.cache.len(), 1);
    assert_eq!(cached_grid.cache.get(0).map(|d| d.path.to_owned()), Some(second_target_path));
  }
}
