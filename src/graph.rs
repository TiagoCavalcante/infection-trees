use crate::rand::BoolRng;

pub struct Graph {
  size: usize,
  data: Vec<bool>,
}

impl Graph {
  pub fn get(&self, row: usize, col: usize) -> bool {
    self.data[row * self.size + col]
  }

  pub fn set(
    &mut self,
    row: usize,
    col: usize,
    value: bool,
  ) {
    self.data[row * self.size + col] = value
  }

  fn max_data_density(&self) -> f32 {
    (self.size as f32 - 1.0) / self.size as f32
  }

  fn fill(&mut self, density: f32) {
    let mut bool_rng =
      BoolRng::new(density / self.max_data_density());

    for i in 0..self.size {
      for j in 0..self.size {
        if i < j {
          self.set(i, j, bool_rng.sample());
        } else if i == j {
          self.set(i, j, false);
        } else {
          self.set(i, j, self.get(j, i));
        }
      }
    }
  }

  pub fn new(size: usize, density: f32) -> Graph {
    let mut graph = Graph {
      size,
      data: vec![false; size * size],
    };

    graph.fill(density);

    graph
  }
}
