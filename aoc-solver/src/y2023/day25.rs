use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
};

use crate::solution::{AocError, Solution};

pub struct Day25;

type Graph = HashMap<String, HashSet<String>>;

fn parse(input: &str) -> Result<Graph, AocError> {
    let mut graph: Graph = HashMap::new();

    for line in input.trim().lines() {
        let (name, edges) = line
            .split_once(": ")
            .ok_or(AocError::parse(line, "Invalid node"))?;

        for edge in edges.split_ascii_whitespace() {
            (*graph.entry(name.to_owned()).or_default()).insert(edge.to_owned());
            (*graph.entry(edge.to_owned()).or_default()).insert(name.to_owned());
        }
    }

    Ok(graph)
}

#[derive(Clone, Eq, PartialEq)]
struct Search {
    distance: u32,
    position: String,
    visited: HashSet<String>,
}

impl Ord for Search {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl PartialOrd for Search {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra(graph: &Graph, start: &str, target: &str) -> Option<u32> {
    let mut dist: HashMap<String, u32> = HashMap::new();
    let mut heap: BinaryHeap<Search> = BinaryHeap::new();

    heap.push(Search {
        distance: 0,
        position: start.to_owned(),
        visited: HashSet::new(),
    });

    while let Some(Search {
        position,
        distance,
        mut visited,
    }) = heap.pop()
    {
        if position == target {
            return Some(dist[&position]);
        }

        if !visited.insert(position.clone()) {
            continue;
        }

        if distance > *dist.get(&position).unwrap_or(&u32::MAX) {
            continue;
        }

        for edge in graph[&position].iter() {
            if edge == target && position == start {
                // We've cut the connection from start to target
                continue;
            }

            let next = Search {
                position: edge.clone(),
                distance: distance + 1,
                visited: visited.clone(),
            };

            let best_known = dist.entry(next.position.clone()).or_insert(u32::MAX);

            if next.distance < *best_known {
                *best_known = next.distance;
                heap.push(next)
            }
        }
    }

    None
}

fn traverse(current: String, graph: &Graph, visited: &mut HashSet<String>) {
    if !visited.insert(current.clone()) {
        return;
    }

    for edge in graph[&current].iter() {
        traverse(edge.clone(), graph, visited)
    }
}

impl Solution for Day25 {
    type A = u32;
    type B = String;

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2023/day25.txt")
    }

    fn part_1(&self, input: &str) -> Result<u32, AocError> {
        let mut graph = parse(input)?;

        loop {
            let starts: Vec<String> = graph.keys().cloned().collect();

            let mut max = (0, String::new(), String::new());

            for start in starts {
                // Check what the distance to the current's neighbours would be if we cut the connection
                let edges = graph.get(&start).cloned().unwrap();

                for edge in edges.iter() {
                    // TODO: If we've already calculated A -> B, don't calculate B -> A
                    // TODO: Cache the distances we see while searching for the distance
                    let distance = dijkstra(&graph, &start, edge);

                    match distance {
                        Some(distance) if distance > max.0 => {
                            max = (distance, start.clone(), edge.clone())
                        }
                        None => {
                            // We're done and the graph is now split into two groups!
                            // Determine the sizes of these groups by traversing one of them

                            // Cut the max connection permanently
                            graph.entry(start.clone()).or_default().remove(edge);
                            graph.entry(edge.clone()).or_default().remove(&start);

                            let mut visited = HashSet::new();

                            traverse(start, &graph, &mut visited);

                            let size_1 = visited.len();
                            let size_2 = graph.len() - size_1;

                            return Ok((size_1 * size_2) as u32);
                        }
                        _ => {}
                    }
                }

                *graph.entry(start).or_default() = edges.clone();
            }

            // The edge that leads to max distance to its neighbour is
            // along separation line, cut that connection permanently
            graph.entry(max.1.clone()).or_default().remove(&max.2);
            graph.entry(max.2).or_default().remove(&max.1);
        }
    }

    fn part_2(&self, _input: &str) -> Result<String, AocError> {
        Ok([
            "",
            "                               ",
            "               *               ",
            "               ^^              ",
            "              ^^o              ",
            "              o^^              ",
            "              ^^o^             ",
            "             o^^^^o            ",
            "             ^^o^^^^           ",
            "        _______||_______       ",
            "            AoC 2023           ",
        ]
        .join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip]
    const EXAMPLE_INPUT: &str =
        "jqt: rhn xhk nvd\n\
         rsh: frs pzl lsr\n\
         xhk: hfx\n\
         cmg: qnr nvd lhk bvb\n\
         rhn: xhk bvb hfx\n\
         bvb: xhk hfx\n\
         pzl: lsr hfx nvd\n\
         qnr: nvd\n\
         ntq: jqt hfx bvb xhk\n\
         nvd: lhk\n\
         lsr: lhk\n\
         rzs: qnr cmg lsr rsh\n\
         frs: qnr lhk lsr";

    #[test]
    fn it_solves_part1_example() {
        assert_eq!(Day25.part_1(EXAMPLE_INPUT), Ok(54));
    }
}
