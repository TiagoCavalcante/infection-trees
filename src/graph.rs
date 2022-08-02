use crate::rand::BoolRng;

pub struct Graph {
  pub size: usize,
  data: Vec<Vec<usize>>,
}

impl Graph {
  fn add_edge(&mut self, a: usize, b: usize) {
    self.data[a].push(b);
    self.data[b].push(a);
  }

  pub fn get_neighbors(&self, i: usize) -> &Vec<usize> {
    &self.data[i]
  }

  fn max_data_density(&self) -> f32 {
    (self.size as f32 - 1.0) / self.size as f32
  }

  fn fill(&mut self, density: f32) {
    let mut bool_rng =
      BoolRng::new(density / self.max_data_density());

    for i in 0..self.size {
      for j in 0..self.size {
        // If i > j it already was added.
        if i < j {
          if bool_rng.sample() {
            self.add_edge(i, j);
          }
        }
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
