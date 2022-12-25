use std::collections::{BinaryHeap, HashMap};
use std::cmp::{PartialOrd, Ordering};

const START: u32 = 'S' as u32;
const END: u32 = 'E' as u32;

#[derive(Hash, PartialEq, Eq)]
 struct Vertex {
    dst: i32,
    val: u32,
    pos: (usize, usize),
}

impl Ord for Vertex {
    fn cmp(&self, other: &Vertex) -> Ordering {
        (-self.dst).cmp(&-other.dst)
    }
}

impl PartialOrd for Vertex {
    fn partial_cmp(&self, other: &Vertex) -> Option<Ordering> {
        (-self.dst).partial_cmp(&-other.dst)
    }
}
/*
impl PartialEq for Vertex {
    fn eq(&self, other: &Vertex) -> bool {
        self.dst.eq(&other.dst)
    }
}
*/

fn generate_heightmap(input: &str) -> Vec<Vec<u32>> {
    input
        .split_terminator('\n')
        .map(|line| line.chars().map(|c| c as u32).collect())
        .collect()
}

/*
function Dijkstra(Graph, source):
2      dist[source] ← 0                           // Initialization
3
4      create vertex priority queue Q
5
6      for each vertex v in Graph.Vertices:
7          if v ≠ source
8              dist[v] ← INFINITY                 // Unknown distance from source to v
9              prev[v] ← UNDEFINED                // Predecessor of v
10
11         Q.add_with_priority(v, dist[v])
12
13
14     while Q is not empty:                      // The main loop
15         u ← Q.extract_min()                    // Remove and return best vertex
16         for each neighbor v of u:              // Go through all v neighbors of u
17             alt ← dist[u] + Graph.Edges(u, v)
18             if alt < dist[v]:
19                 dist[v] ← alt
20                 prev[v] ← u
21                 Q.decrease_priority(v, alt)
22
23     return dist, prev
 */


pub fn part_one(input: &str) -> Option<u32> {
    let heightmap = generate_heightmap(input);
    let nrows = heightmap.len();
    let ncols = heightmap[0].len();
    let mut vertices: HashMap<(usize, usize), Vertex> = HashMap::with_capacity(nrows*ncols);
    for i in 0..nrows {
        for j in 0..ncols {
            let dst = if (i, j) == (0, 0) { 0 } else {i32::MAX};
            vertices.insert((i, j), Vertex {dst: dst, val: heightmap[i][j], pos: (i, j)});
        }
    }

    // initialization
    let mut queue: BinaryHeap<&Vertex> = BinaryHeap::new();
    let mut dist: HashMap<&Vertex, u32> = HashMap::new();
    let mut prev: HashMap<&Vertex, Option<&Vertex>> = HashMap::new();
    for vertex in vertices.values() {
        if vertex.val != START {
            dist.insert(vertex, u32::MAX);
            prev.insert(vertex, None);
        }

        queue.push(vertex);
    }

    let get_neighbors = |(i, j)| [
        (i + 1, j),
        (i - 1, j),
        (i, j + 1),
        (i, j - 1)].iter()
        .filter(|(i, j)| (0 <= *i) && (*i <= nrows as i32) && (0 <= *j) && (*j<= ncols as i32))
        .map(|(i, j)| (*i as usize, *j as usize))
        .collect::<Vec<(usize, usize)>>();

    while !queue.is_empty() {
        let u = queue.pop().unwrap();
        for (i, j) in get_neighbors((u.pos.0 as i32, u.pos.1 as i32)) {
            let v = vertices.get_mut(&(i, j)).unwrap();
            let alt = u.dst + (v.val - u.val) as i32;

            // TODO: In progress
            if alt < v.dst {
                v.dst = alt;
                prev.insert(&v, Some(u));
            }
        }
    }
    

    None
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), None);
    }
}
