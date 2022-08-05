use crate::graph::Graph;

fn in_path(
  predecessor: &Vec<usize>,
  to: usize,
  vertex: usize,
) -> bool {
  let mut current = to;

  while predecessor[current] != usize::MAX {
    current = predecessor[current];
    if current == vertex {
      return true;
    }
  }

  return false;
}

pub fn fixed_length_bfs(
  graph: &Graph,
  start: usize,
  end: usize,
  length: usize,
) -> Option<Vec<usize>> {
  let distance = length - 1;

  let mut predecessor_from_start =
    vec![usize::MAX; graph.size];
  let mut distance_to_start = vec![usize::MAX; graph.size];

  let mut predecessor_from_end =
    vec![usize::MAX; graph.size];
  let mut distance_to_end = vec![usize::MAX; graph.size];

  // A queue to maintain the vertices whose adjacency list
  // is to be scanned as per normal DFS algorithm.
  let mut queue = std::collections::VecDeque::new();

  // The distance from the start to itself is 0.
  distance_to_start[start] = 0;
  queue.push_front(start);

  // Standard BFS algorithm
  // See https://en.wikipedia.org/wiki/Breadth-first_search.
  // Note that in the BFS algorithm the queue must be
  // first in last out.
  while let Some(current) = queue.pop_front() {
    for vertex in graph.get_neighbors(current) {
      // If the distance is usize::MAX then that vertex was
      // never reached before.
      if distance_to_start[*vertex] == usize::MAX {
        distance_to_start[*vertex] =
          distance_to_start[current] + 1;
        predecessor_from_start[*vertex] = current;
        queue.push_back(*vertex);
      }
    }
  }

  // Here we are starting from the end and going to the
  // start.
  // The distance from the start to itself is 0.
  distance_to_end[end] = 0;
  queue.push_back(end);

  // Here the magic happens.
  // Instead of finding the smallest path we are trying to
  // find the biggest path that is no bigger than the
  // length.
  // We want it to be exactly equal to the length, but we
  // won't get there so easy.
  // Contrary to BFS, here the queue must be first in first
  // out, otherwise it could (and that almost always happen)
  // change the path to a vertex without updating its
  // distance, so when it finds a path with the correct
  // length, the predecessor array would have changed and
  // a path with a bigger length would be returned instead.
  while let Some(current) = queue.pop_back() {
    for vertex in graph.get_neighbors(current) {
      // If we never visited this vertex or the size of the
      // path is bigger than the last path but still not
      // bigger than the length and that neighbor is not in
      // the path to the current vertex.
      // Note: if the distance is usize::MAX then that
      // vertex was never reached before.
      if distance_to_end[*vertex] == usize::MAX
        || (distance_to_end[current] + 1
          > distance_to_end[*vertex]
          // If the sum of both is less than length, then
          // their sum + 1 won't be bigger than length.
          && distance_to_end[current]
            + distance_to_start[*vertex]
            < distance
          // If it is already in path then we won't go to
          // this neighbor as we can't use any vertex more
          // than once.
          && !in_path(
            &predecessor_from_end,
            current,
            *vertex,
          ))
      {
        distance_to_end[*vertex] =
          distance_to_end[current] + 1;
        predecessor_from_end[*vertex] = current;

        if distance_to_start[*vertex]
          + distance_to_end[*vertex]
          == distance
        {
          // First find the path between the first vertex
          // and the current.
          let mut path = vec![current];
          let mut current = *vertex;

          while predecessor_from_start[current]
            != usize::MAX
          {
            path.push(predecessor_from_start[current]);
            current = predecessor_from_start[current];
          }

          path.reverse();

          // Then append the path between the current vertex
          // and the last.
          current = *vertex;

          while predecessor_from_end[current] != usize::MAX
          {
            path.push(predecessor_from_end[current]);
            current = predecessor_from_end[current];
          }

          return Some(path);
        }

        queue.push_back(*vertex);
      }
    }
  }

  return None;
}
