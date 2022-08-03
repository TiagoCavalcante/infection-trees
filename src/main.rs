use crate::path::shortest_path;

mod graph;
mod path;
mod rand;

fn main() {
  let density = 0.01;
  let size = 3000;
  let mut graph = graph::Graph::new(size, density);

  // let mut count = vec![0; size];

  let mut bool_rng = rand::BoolRng::new(0.2);
  let marked = (0..size)
    .filter_map(|i| {
      if bool_rng.sample() {
        Some(i)
      } else {
        None
      }
    })
    .collect::<Vec<_>>();

  // let mut marked_rng = rand::OneOfRng::new(marked.clone());

  let mut path = shortest_path(
    &graph,
    *marked.first().unwrap(),
    *marked.last().unwrap(),
  )
  .unwrap();

  let mut vertices = path
    .iter()
    .map(|vertex| graph.pop_vertex(*vertex))
    .collect::<Vec<_>>();

  for vertex in &marked[1..marked.len() - 1] {
    let mut found = false;

    for (i, start) in path.clone().iter().enumerate() {
      graph.add_edges(*start, &vertices[i]);

      if let Some(new_path) = shortest_path(&graph, *start, *vertex) {
        found = true;
        graph.pop_vertex(*start);
        path.extend(&new_path);

        for vertex in new_path {
          vertices.push(graph.pop_vertex(vertex));
        }

        break;
      }

      graph.pop_vertex(*start);
    }

    if !found {
      panic!("Couldn't find a valid graph");
    }
  }

  println!("We have found a valid graph!");

  // marked.iter().for_each(|this| {
  //   for _ in 0..10 {
  //     let other = marked_rng.sample();

  //     if let Some(path) =
  //       shortest_path(&graph, *this, *other)
  //     {
  //       for vertex in path.iter() {
  //         count[*vertex] += 1;
  //       }
  //     }
  //   }
  // });

  // let mut count = count
  //   .iter()
  //   .enumerate()
  //   .filter(|(_, v)| **v > 0)
  //   .collect::<Vec<_>>();

  // count.sort_by(|(_, r), (_, l)| l.cmp(r));

  // println!(
  //   "{}",
  //   count
  //     .iter()
  //     .map(|(i, v)| format!("{i}: {v}"))
  //     .collect::<Vec<_>>()
  //     .join("\n"),
  // );
}
