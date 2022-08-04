mod graph;
mod path;
mod rand;

fn main() {
  let density = 0.1;
  let size = 300;
  let mut graph = graph::Graph::new(size, density);

  let mut size_rng = rand::UniformRng::new(3, 6);

  let mut bool_rng = rand::BoolRng::new(0.1);
  let marked = (0..size)
    .filter_map(|i| {
      if bool_rng.sample() {
        Some((i, size_rng.sample()))
      } else {
        None
      }
    })
    .collect::<Vec<_>>();
  println!("number of marked = {}", marked.len());

  let mut path = path::yen(
    &mut graph,
    marked.first().unwrap().0,
    marked.last().unwrap().0,
    marked.last().unwrap().1,
  )
  .unwrap();

  // Collect the edges of the path between the first and the
  // last marked vertices.
  // We are removing those edges because to build a valid
  // tree, a vertex can only be used once.
  let mut edges = path
    .iter()
    .map(|vertex| graph.pop_edges(*vertex))
    .collect::<Vec<_>>();

  // Iterate over all marked vertices except the first and
  // the last.
  for vertex in &marked[1..marked.len() - 1] {
    let mut found = false;

    for (index, start) in path.clone().iter().enumerate() {
      // Restore the edges between the current vertex and
      // its neighbors as a path can't start at a
      // unconnected vertex.
      // This vertex is still going to be used once because
      // a vertex can have more than 1 child in the tree.
      graph.add_edges(*start, &edges[index]);

      match path::yen(
        &mut graph, *start, vertex.0, vertex.1,
      ) {
        Some(new_path) => {
          found = true;

          path.extend(&new_path);

          for vertex in new_path {
            edges.push(graph.pop_edges(vertex));
          }
        }
        None => continue,
      }

      // Remove the edges again.
      graph.pop_edges(*start);
      // If we reached here is because a path was found.
      break;
    }

    if !found {
      panic!("Couldn't find a valid graph");
    }
  }

  println!("We have found a valid graph!");
}
