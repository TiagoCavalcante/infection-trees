use crate::graph::Graph;

fn _count_vertices(
  graph: &Graph,
  start: usize,
  end: usize,
  max_size: usize,
  path: &mut Vec<usize>,
  remaining_vertices: &mut Vec<usize>,
  count: &mut Vec<usize>,
) {
  let last = remaining_vertices.len() - 1;

  for i in 0..=last {
    let vertex = remaining_vertices[i];

    if graph.get(start, vertex) {
      path.push(vertex);

      if vertex == end {
        path.iter().for_each(|i| count[*i] += 1);
      } else if max_size > 0 {
        // Swap the current element with the last and
        // decrement the length.
        remaining_vertices.swap_remove(i);

        _count_vertices(
          graph,
          start,
          end,
          max_size - 1,
          path,
          remaining_vertices,
          count,
        );

        // Revert our changes.
        remaining_vertices.push(vertex);
        remaining_vertices.swap(i, last);
      }

      path.pop();
    }
  }
}

/// Count the number of times each vertex is used in all
/// paths from the start vertex to the end vertex with a
/// given max_size.
/// If you are using this function to measure the
/// probability of a vertex be used in a valid graph don't
/// forget to give all tries the same max_size.
pub fn count_vertices(
  graph: &Graph,
  start: usize,
  end: usize,
  max_size: usize,
  count: &mut Vec<usize>
) {
  let mut remaining_vertices = (0..graph.size)
    .filter(|i| *i != start)
    .collect::<Vec<_>>();

  let mut path = vec![start];

  _count_vertices(
    graph,
    start,
    end,
    max_size,
    &mut path,
    &mut remaining_vertices,
    count,
  );
}
