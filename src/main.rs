use crate::path::shortest_path;

mod graph;
mod path;
mod rand;

fn main() {
  let density = 0.01;
  let size = 3000;
  let mut graph = graph::Graph::new(size, density);

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

  let mut path = shortest_path(
    &graph,
    *marked.first().unwrap(),
    *marked.last().unwrap(),
  )
  .unwrap();

  let mut vertices = path
    .iter()
    .map(|vertex| graph.pop_edges(*vertex))
    .collect::<Vec<_>>();

  for vertex in &marked[1..marked.len() - 1] {
    let mut found = false;

    for (i, start) in path.clone().iter().enumerate() {
      graph.add_edges(*start, &vertices[i]);

      if let Some(new_path) =
        shortest_path(&graph, *start, *vertex)
      {
        found = true;
        graph.pop_edges(*start);
        path.extend(&new_path);

        for vertex in new_path {
          vertices.push(graph.pop_edges(vertex));
        }

        break;
      }

      graph.pop_edges(*start);
    }

    if !found {
      panic!("Couldn't find a valid graph");
    }
  }

  println!("We have found a valid graph!");
}
