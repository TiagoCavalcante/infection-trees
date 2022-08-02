use crate::graph::Graph;

/// Breadth First Search
fn bfs(
  graph: &Graph,
  start: usize,
  end: usize,
  predecessor: &mut Vec<usize>,
  distance: &mut Vec<usize>,
) -> bool {
  // A queue to maintain the vertices whose adjacency list
  // is to be scanned as per normal DFS algorithm.
  let mut queue = std::collections::VecDeque::new();

  // Stores the information whether ith vertex is reached at
  // least once in the Breadth First Search.
  let mut visited = vec![false; graph.size];

  // The start is the first to be visited and the distance
  // from the start to itself is 0.
  visited[start] = true;
  distance[start] = 0;
  queue.push_back(start);

  // Standard BFS algorithm
  // See https://en.wikipedia.org/wiki/Breadth-first_search.
  while let Some(current) = queue.pop_front() {
    for vertex in graph.get_neighbors(current) {
      if !visited[*vertex] {
        visited[*vertex] = true;
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
  // Here usize::MAX is used to indicate infinite distance.
  let mut distance = vec![usize::MAX; graph.size];

  if bfs(graph, start, end, &mut predecessor, &mut distance)
  {
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
