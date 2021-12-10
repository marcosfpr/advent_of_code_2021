use std::collections::{BTreeSet, HashSet, VecDeque};

type FrozenSet<T> = BTreeSet<T>;

#[path = "aoc_utils.rs"]
mod aoc_utils;

pub type Graph = Vec<Vec<i32>>;

struct Basin<'a> {
    pub graph: &'a  Graph,
    pub nodes: HashSet<(i32, i32)>,
}

impl<'a> Basin<'a>  {
  fn new(graph: &Graph) -> Basin {
    Basin {
      graph: graph,
      nodes: HashSet::new(),
    }
  }

  fn get_neighbors(&self, node: (i32, i32)) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();

    let (i, j) = node;
    
    let i = i as usize;
    let j = j as usize; 

    if i > 0 { neighbors.push((i - 1, j)); }
    if i < self.graph.len() - 1 { neighbors.push((i + 1, j)); }
    if j > 0 { neighbors.push((i, j - 1)); }
    if j < self.graph[i].len() - 1 { neighbors.push((i, j + 1) ); }

    neighbors
  }

  fn insert_nodes_by_low_point(&mut self, low_point: (i32, i32)) {
    let mut queue = VecDeque::new();
    queue.push_back(low_point);
    while !queue.is_empty() {
      let node = queue.pop_back().unwrap();
      if !self.nodes.contains(&node) {
        self.nodes.insert(node);
        for (i,  j) in self.get_neighbors(node) {
          if self.graph[i][j] != 9 {
            queue.push_back((i as  i32, j as i32));
          }
        }
      }
    }
  }
}

pub fn resolve_part_1(filename: &'static str) -> i32 {
  let heighmap: Vec<String> = aoc_utils::read_file_to_vec(filename).expect("Failed to read file");
  
  let rows = heighmap.len();
  let cols = heighmap[0].len();

  let graph: Graph = build_graph(heighmap, rows, cols);

  let mut total_low_points = 0;

  for i in 0..rows {
    for j in 0..cols {
      // corners  => i == 0 || j == 0 || i == rows - 1 || j == cols - 1
      let height = graph[i][j];

      let up = if i > 0 { graph[i - 1][j] } else { 10 }; // values go up to 9
      let down = if i < rows - 1 { graph[i + 1][j] } else { 10 };
      let left = if j > 0 { graph[i][j - 1] } else { 10 };
      let right = if j < cols - 1 { graph[i][j + 1] } else { 10 };

      if height < up && height < down && height < left && height < right {
        total_low_points += height + 1;
      }
    }
  }

  total_low_points
}


pub fn resolve_part_2(filename: &'static str) -> i32 {
  let heighmap: Vec<String> = aoc_utils::read_file_to_vec(filename).expect("Failed to read file");
  
  let rows = heighmap.len();
  let cols = heighmap[0].len();

  let graph: Graph = build_graph(heighmap, rows, cols);
  let low_points = get_low_points(&graph, rows, cols);
  
  let mut basin_lenghts = VecDeque::new();

  for lp in low_points {
    let mut basin = Basin::new(&graph);
    basin.insert_nodes_by_low_point(lp);
    if basin.nodes.len() > 0 {
      basin_lenghts.push_back(basin.nodes.len());
    }
  }

  let mut basin_lenghts: Vec<usize>  = basin_lenghts.iter().map(|x| x.clone()).collect();

  basin_lenghts.sort_by(|a, b| b.cmp(a));

  (basin_lenghts[0] * basin_lenghts[1] * basin_lenghts[2]) as i32

}

fn get_low_points(graph: &Graph,  rows: usize, cols: usize) -> HashSet<(i32, i32)> {
  let mut low_points = HashSet::new();

  for i in 0..rows {
    for j in 0..cols {
      // corners  => i == 0 || j == 0 || i == rows - 1 || j == cols - 1
      let height = graph[i][j];

      let up = if i > 0 { graph[i - 1][j] } else { 10 }; // values go up to 9
      let down = if i < rows - 1 { graph[i + 1][j] } else { 10 };
      let left = if j > 0 { graph[i][j - 1] } else { 10 };
      let right = if j < cols - 1 { graph[i][j + 1] } else { 10 };

      if height < up && height < down && height < left && height < right {
        low_points.insert((i as i32, j as i32));
      }
    }
  }
  low_points
}


fn build_graph(heighmap: Vec<String>, rows: usize, cols: usize) -> Graph {
  // initialize graph
  let mut graph: Graph = vec![vec![0; cols]; rows];

  for (i, line) in heighmap.iter().enumerate() { 
    for (j, c) in line.chars().enumerate() {
      graph[i][j] = c.to_digit(10).unwrap() as i32;
    }
  }

  graph
}