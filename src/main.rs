use crate::path::shortest_path;

mod graph;
mod path;
mod rand;

fn main() {
  let density = 0.5;
  let size = 300;
  let graph = graph::Graph::new(size, density);

  let mut count = vec![0; size];

  let mut bool_rng = rand::BoolRng::new(0.1);
  let marked: Vec<usize> = (0..size)
    .filter_map(|i| {
      if bool_rng.sample() {
        Some(i)
      } else {
        None
      }
    })
    .collect::<Vec<_>>();

  let mut marked_rng = rand::OneOfRng::new(marked.clone());

  marked.iter().for_each(|this| {
    for _ in 0..10 {
      let other = marked_rng.sample();

      if let Some(path) =
        shortest_path(&graph, *this, *other)
      {
        for vertex in path.iter() {
          count[*vertex] += 1;
        }
      }
    }
  });

  let mut count = count
    .iter()
    .enumerate()
    .filter(|(_, v)| **v > 0)
    .collect::<Vec<_>>();

  count.sort_by(|(_, r), (_, l)| l.cmp(r));

  println!(
    "{}",
    count
      .iter()
      .map(|(i, v)| format!("{i}: {v}"))
      .collect::<Vec<_>>()
      .join("\n"),
  );
}
