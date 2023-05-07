use crate::node::{HeuristicNode, Node};
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
    let mut open_nodes = VecDeque::new();
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
        let connected_neighbours = current_node
          .node
          .connections
          .iter()
          .flat_map(|c| self.nodes.iter().find(|n| &n.id == c))
          .collect::<Vec<&Node>>();

        for node in connected_neighbours {
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
  fn it_works_without_cache() {
    let map = vec![vec![".", ".", "."], vec!["#", "#", "."], vec![".", ".", "."]];
    let grid: Grid = map.into();
    let p = grid.find_path(Pos(0, 0), Pos(0, 2));
    assert!(p.is_some());
    assert_eq!(p.unwrap(), vec![Pos(1, 0), Pos(2, 0), Pos(2, 1), Pos(2, 2), Pos(1, 2), Pos(0, 2)]);
  }

  #[test]
  fn it_works_with_cache() {
    let map = vec![vec![".", ".", "."], vec!["#", "#", "."], vec![".", ".", "."]];
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
    let map = vec![vec![".", ".", "."], vec!["#", "#", "."], vec![".", ".", "."]];
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
