use crate::rand::UniformRng;

pub struct Graph {
  pub size: usize,
  pub data: Vec<Vec<usize>>,
}

impl Graph {
  pub fn add_edge(&mut self, a: usize, b: usize) {
    self.data[a].push(b);
    self.data[b].push(a);
  }

  pub fn get_neighbors(
    &self,
    vertex: usize,
  ) -> &Vec<usize> {
    &self.data[vertex]
  }

  /// Returns the neighbors of `vertex` and remove the edges
  /// between `vertex` and its neighbors.
  pub fn pop_edges(&mut self, vertex: usize) -> Vec<usize> {
    let neighbors = self.data[vertex].clone();

    for neighbor in &neighbors {
      let position = self.data[*neighbor]
        .iter()
        .position(|v| *v == vertex)
        .unwrap();

      self.data[*neighbor].swap_remove(position);
    }

    self.data[vertex].clear();

    neighbors
  }

  /// Add edges between `vertex` and each neighbor of
  /// `neighbors`, it is usually used in conjunction with
  /// `pop_edges`.
  /// ```
  /// let neighbors = graph.pop_edges(vertex);
  /// let path_without_vertex =
  ///   path::shortest_path(&graph, a, b);
  /// // Restore the edges.
  /// graph.add_edges(vertex, neighbors);
  /// ```
  pub fn add_edges(
    &mut self,
    vertex: usize,
    neighbors: &Vec<usize>,
  ) {
    for neighbor in neighbors {
      self.add_edge(vertex, *neighbor);
    }
  }

  fn max_data_density(&self) -> f32 {
    (self.size as f32 - 1.0) / self.size as f32
  }

  pub fn fill(&mut self, density: f32) {
    let real_density = density / self.max_data_density();

    let marked = (real_density
      // This is squared because we need to "throw the coin"
      // for each pair of vertices.
      * self.size.pow(2) as f32
      // And divided by 2 because when we add a connection
      // we add 2 edges, as the graph is undirected.
      * 0.5) as usize;

    let mut vertex_rng = UniformRng::new(0, self.size);

    for _ in 0..marked {
      let a = vertex_rng.sample();
      let b = vertex_rng.sample();

      if a != b {
        self.add_edge(a, b);
      }
    }
  }

  pub fn new(size: usize, density: f32) -> Graph {
    let mut graph = Graph {
      size,
      data: vec![vec![]; size],
    };

    graph.fill(density);

    graph
  }
}
