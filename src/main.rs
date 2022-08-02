use path::count_vertices;

mod graph;
mod path;
mod rand;

fn main() {
  let density = 0.5;
  let size = 10;
  let max_size = 8;
  let graph = graph::Graph::new(size, density);

  let mut count = vec![0; size];

  let mut bool_rng = rand::BoolRng::new(0.1);
  let mut marked: Vec<usize> = (0..size)
    .filter_map(|i| {
      if bool_rng.sample() {
        Some(i)
      } else {
        None
      }
    })
    .collect::<Vec<_>>();

  if marked.len() == 0 {
    marked.push(0);
  }

  let mut marked_rng = rand::OneOfRng::new(marked.clone());

  marked.iter().for_each(|this| {
    for _ in 0..3 {
      let other = marked_rng.sample();

      count_vertices(
        &graph, *this, *other, max_size, &mut count,
      );
    }
  });

  let mut count = count
    .iter()
    .enumerate()
    .filter(|(_, v)| **v > 0)
    .collect::<Vec<_>>();

  count.sort_by(|(_, r), (_, l)| r.cmp(l));

  println!(
    "{}",
    count
      .iter()
      .map(|(i, v)| format!("{i}: {v}"))
      .collect::<Vec<_>>()
      .join("\n"),
  );
}
