use crate::graph::Graph;

/// Implementation of the Breadth First Search.
/// See https://en.wikipedia.org/wiki/Breadth-first_search
/// This scans the graph level by level, passing through
/// each vertex at most once.
/// It stops when the end vertex is reached.
/// The distances to each vertex from the start are stored
/// in the `distance` vector, that is initialized to
/// `usize::MAX`.
/// The predecessors of each vertex are stored in the
/// `predecessor` vector, that is also initilized to
/// `usize::MAX`, so if the predecessor of a node is
/// `usize::MAX` then that node was never reached.
/// This modifies the `distance` and `predecessor` vectors.
/// Returns `true` if `start` and `end` are connected and
/// `false` otherwise.
fn bfs(
  graph: &Graph,
  start: usize,
  end: usize,
  predecessor: &mut Vec<usize>,
) -> bool {
  // A queue to maintain the vertices whose adjacency list
  // is to be scanned as per normal DFS algorithm.
  let mut queue = std::collections::VecDeque::new();

  // Here usize::MAX is used to indicate infinite distance.
  let mut distance = vec![usize::MAX; graph.size];

  // The distance from the start to itself is 0.
  distance[start] = 0;
  queue.push_back(start);

  // Standard BFS algorithm.
  while let Some(current) = queue.pop_front() {
    for vertex in graph.get_neighbors(current) {
      // If it wasn't visited.
      if distance[*vertex] == usize::MAX {
        distance[*vertex] = distance[current] + 1;
        predecessor[*vertex] = current;
        queue.push_back(*vertex);

        // We stop the BFS when we find the destination.
        if *vertex == end {
          return true;
        }
      }
    }
  }

  return false;
}

/// Returns the shortest path between `start` and `end`.
/// Returns `None` if no path exists.
/// ```
/// let graph = graph::Graph::new(300, 0.01);
/// let path =
///   path::shortest_path(&graph, 0, 299).unwrap_or(vec![]);
/// println!("{:?}", path);
/// ```
pub fn shortest_path(
  graph: &Graph,
  start: usize,
  end: usize,
) -> Option<Vec<usize>> {
  // Here usize::MAX is used to indicate that there is no
  // predecessor.
  let mut predecessor = vec![usize::MAX; graph.size];

  if bfs(graph, start, end, &mut predecessor) {
    let mut path = vec![end];
    let mut current = end;
    while predecessor[current] != usize::MAX {
      path.push(predecessor[current]);
      current = predecessor[current];
    }

    path.reverse();

    Some(path)
  } else {
    // Source and destination are not connected.
    None
  }
}

fn equal_paths(a: &Vec<usize>, b: &Vec<usize>) -> bool {
  a.iter().zip(b).all(|(a, b)| *a == *b)
}

/// Use Yen algorithm for find a path with length `length`.
/// Returns None if this path doesn't exist.
pub fn yen(
  graph: &mut Graph,
  start: usize,
  end: usize,
  length: usize,
) -> Option<Vec<usize>> {
  if let Some(shortest) = shortest_path(&graph, start, end)
  {
    let mut paths = vec![shortest];
    let mut b: Vec<Vec<usize>> = vec![];

    for k in 1..=graph.size - length {
      // The spur node ranges from the first node to the
      // next to last node in the previous k-shortest path.
      let last_length = paths[k - 1].len();

      for i in 0..last_length - 2 {
        // Spur node is retrieved from the previous
        // k-shortest path, k − 1.
        let spur_node = paths[k - 1][i];
        // The sequence of nodes from the source to the spur
        // node of the previous k-shortest path.
        let root_path = paths[k - 1][0..i].to_vec();

        let mut edges = vec![];
        let mut nodes = vec![];

        for p in paths.iter() {
          if p.len() > i + 1
            && equal_paths(&root_path, &p[0..i].to_vec())
          {
            // Remove the links that are part of the
            // previous shortest paths which share
            // the same root path.
            if graph.has_edge(p[i], p[i + 1]) {
              graph.remove_edge(p[i], p[i + 1]);
              edges.push((p[i], p[i + 1]));
            }
          }
        }

        for node in &root_path {
          if *node != spur_node {
            nodes.push(graph.pop_edges(*node));
          }
        }

        // Calculate the spur path from the spur node to the
        // end.
        // Consider also checking if any spur_path found.
        if let Some(spur_path) =
          shortest_path(graph, spur_node, end)
        {
          // Entire path is made up of the root path and
          // spur path.
          let mut total_path = root_path.clone();
          total_path.extend(spur_path);

          // Add the potential k-shortest path to the heap.
          if b
            .iter()
            .find(|path| equal_paths(path, &total_path))
            .is_none()
          {
            b.push(total_path);
          }

          // Add back the edges and nodes that were removed
          // from the graph.
          for (a, b) in edges {
            graph.add_edge(a, b);
          }

          for (node, neighbors) in nodes.iter().enumerate()
          {
            graph.add_edges(node, neighbors)
          }
        }
      }

      if b.is_empty() {
        // This handles the case of there being no spur
        // paths, or no spur paths left.
        // This could happen if the spur paths have already
        // been exhausted (added to paths),
        // or there are no spur paths at all - such as when
        // both the source and sink vertices
        // lie along a "dead end".
        break;
      }

      b.sort();
      if b[0].len() == length {
        return Some(b[0].clone());
      }
      // Add the lowest cost path becomes the k-shortest
      // path.
      paths.push(b[0].clone());
      b.swap_remove(0);
    }

    None
  } else {
    None
  }
}
