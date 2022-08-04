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

/// Uniform random number generator
/// ```
/// let mut uniform_rng = BoolRng::new(1, 6);
/// let dice: usize = uniform_rng.sample();
/// ```
pub struct UniformRng {
  uniform_rng: Uniform<usize>,
  rng: ThreadRng,
}

impl UniformRng {
  /// Receives the probability of yielding `true`.
  pub fn new(start: usize, end: usize) -> UniformRng {
    let uniform_rng: Uniform<usize> =
      Uniform::from(start..end);
    let rng: ThreadRng = rand::thread_rng();

    UniformRng { uniform_rng, rng }
  }

  pub fn sample(&mut self) -> usize {
    self.uniform_rng.sample(&mut self.rng)
  }
}
