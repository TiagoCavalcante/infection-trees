use rand::distributions::{Distribution, Uniform};
use rand::rngs::ThreadRng;

/// Random boolean generator
/// ```
/// let mut bool_rng = BoolRng::new(0.5);
/// let is_true: bool = bool_rng.sample();
/// ```
pub struct BoolRng {
  uniform_rng: Uniform<usize>,
  rng: ThreadRng,
  threshold: usize,
}

impl BoolRng {
  /// Receives the probability of yielding `true`.
  pub fn new(probability: f32) -> BoolRng {
    let uniform_rng: Uniform<usize> =
      Uniform::from(0..usize::MAX);
    let rng: ThreadRng = rand::thread_rng();

    BoolRng {
      uniform_rng,
      rng,
      threshold: (probability * usize::MAX as f32) as usize,
    }
  }

  pub fn sample(&mut self) -> bool {
    self.uniform_rng.sample(&mut self.rng) < self.threshold
  }
}

/// # Example
/// ```
/// let mut vertex_rng = OneOfRng::new(vec![-7, 0, 10, 14]);
/// ```
pub struct _OneOfRng<T> {
  uniform_rng: Uniform<usize>,
  rng: ThreadRng,
  possible: Vec<T>,
}

impl<T> _OneOfRng<T> {
  pub fn _new(possible: Vec<T>) -> _OneOfRng<T> {
    let uniform_rng: Uniform<usize> =
      Uniform::from(0..possible.len());
    let rng: ThreadRng = rand::thread_rng();

    _OneOfRng {
      uniform_rng,
      rng,
      possible,
    }
  }

  pub fn _sample(&mut self) -> &T {
    &self.possible[self.uniform_rng.sample(&mut self.rng)]
  }
}
