use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
};

use itertools::Itertools;

use crate::solution::{AocError, Solution};

pub struct Day23;

type Coords = (isize, isize);
type Grid = Vec<Vec<Tile>>;
type Graph = Vec<Vec<(usize, u32)>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

const DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::East,
    Direction::South,
    Direction::West,
];

impl Direction {
    fn neighbours(
        grid: &Grid,
        position: Coords,
        width: isize,
        height: isize,
        slippery: bool,
    ) -> Vec<Coords> {
        DIRECTIONS
            .iter()
            .filter_map(|direction| {
                let (dx, dy) = direction.as_delta();
                let (x, y) = (position.0 + dx, position.1 + dy);

                if x < 0 || y < 0 || x >= width || y >= height {
                    return None;
                }

                match grid[y as usize][x as usize] {
                    Tile::Forest => None,
                    Tile::Slope(slope_direction) if slippery && slope_direction != *direction => {
                        None
                    }
                    _ => Some((x, y)),
                }
            })
            .collect()
    }

    fn as_delta(&self) -> (isize, isize) {
        match self {
            Direction::North => (0, -1),
            Direction::East => (1, 0),
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Tile {
    Path,
    Forest,
    Slope(Direction),
}

fn parse(input: &str) -> Result<Grid, AocError> {
    let grid: Grid = input
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|tile| {
                    let tile = match tile {
                        '.' => Tile::Path,
                        '#' => Tile::Forest,
                        '^' => Tile::Slope(Direction::North),
                        '>' => Tile::Slope(Direction::East),
                        'v' => Tile::Slope(Direction::South),
                        '<' => Tile::Slope(Direction::West),
                        _ => return Err(AocError::parse(tile, "Unknown tile")),
                    };

                    Ok(tile)
                })
                .try_collect()
        })
        .try_collect()?;

    Ok(grid)
}

fn find_edges(grid: &Grid, current: Coords) -> HashMap<Coords, u32> {
    let (_, target, height, width) = bounds(grid);

    let mut connected = HashMap::new();

    if current == target {
        return connected;
    }

    for start in Direction::neighbours(grid, current, width, height, false) {
        let mut stack = vec![(start, 1)];
        let mut visited = HashSet::from([current]);

        while let Some((position, distance)) = stack.pop() {
            visited.insert(position);

            if position == target {
                connected.insert(position, distance);
                break;
            }

            // Count how many neighbours the current position has.
            // If it has more than one consider it as a connected vertex.
            let neighbours: Vec<Coords> =
                Direction::neighbours(grid, position, width, height, false)
                    .iter()
                    .filter(|neighbour| !visited.contains(neighbour))
                    .copied()
                    .collect();

            if neighbours.len() > 1 {
                // If a longer way to target vertex has been found keep using it
                let longest_dist = connected.entry(position).or_default();
                *longest_dist = u32::max(*longest_dist, distance);
            } else {
                stack.extend(
                    neighbours
                        .into_iter()
                        .map(|neighbour| (neighbour, distance + 1)),
                );
            }
        }
    }

    connected
}

fn simplify_graph(grid: &Grid) -> (Graph, usize, usize, u32) {
    let start = (1, 0);

    let mut graph = HashMap::new();
    let mut stack = vec![start];

    while let Some(current_node) = stack.pop() {
        let edges = find_edges(grid, current_node);
        let new_vertices = edges.keys().filter(|vertex| !graph.contains_key(*vertex));

        stack.extend(new_vertices);
        graph.insert(current_node, edges);
    }

    let (start, mut target, _, _) = bounds(grid);
    let mut distance = 0;

    // Trim the end of the graph to the last junction that leads to the target
    loop {
        let nodes_leading_to_end: Vec<_> = graph
            .iter()
            .filter_map(|(key, edges)| {
                if edges.contains_key(&target) {
                    Some(*key)
                } else {
                    None
                }
            })
            .collect();

        if nodes_leading_to_end.len() != 1 {
            break;
        }

        let previous = nodes_leading_to_end[0];

        graph.remove(&target);

        let edge_distance = graph[&previous][&target];
        distance += edge_distance;

        graph.entry(previous).or_default().remove(&target);

        target = previous;
    }

    // Convert the graph from hashmap to vector
    let mapping: Vec<Coords> = graph.keys().copied().collect();
    let graph: Graph = mapping
        .iter()
        .map(|coords| {
            graph[coords]
                .iter()
                .map(|(edge, edge_distance)| {
                    let index = mapping.iter().position(|m| m == edge).unwrap();

                    (index, *edge_distance)
                })
                .collect()
        })
        .collect();

    let start = mapping.iter().position(|m| *m == start).unwrap();
    let target = mapping.iter().position(|m| *m == target).unwrap();

    (graph, start, target, distance)
}

fn bounds(grid: &Grid) -> (Coords, Coords, isize, isize) {
    let height = grid.len() as isize;
    let width = grid[0].len() as isize;

    let start = (1, 0);
    let target = (width - 2, height - 1);

    (start, target, height, width)
}

#[derive(Clone, Eq, PartialEq)]
struct Search {
    distance: i32,
    position: Coords,
    visited: HashSet<Coords>,
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

// At part 1 use dijkstra that instead of increasing the distance reduces it as
// the distance grows. It then tries to maximize minimize this, finding the longest
// path in the directed acyclic graph (DAG). This was based on this SO thread:
// https://stackoverflow.com/questions/8027180/dijkstra-for-longest-path-in-a-dag
fn dijkstra(grid: &Grid) -> u32 {
    let (start, target, height, width) = bounds(grid);

    let mut dist: HashMap<Coords, i32> = HashMap::new();
    let mut heap: BinaryHeap<Search> = BinaryHeap::new();

    heap.push(Search {
        distance: 0,
        position: start,
        visited: HashSet::new(),
    });

    let mut max_distance = 0;

    while let Some(Search {
        position,
        distance,
        mut visited,
    }) = heap.pop()
    {
        if position == target {
            max_distance = max_distance.max(-dist[&position] as u32);
            continue;
        }

        if !visited.insert(position) {
            continue;
        }

        if distance > *dist.get(&position).unwrap_or(&i32::MAX) {
            continue;
        }

        for (x, y) in Direction::neighbours(grid, position, width, height, true) {
            let next = Search {
                position: (x, y),
                distance: distance - 1,
                visited: visited.clone(),
            };

            let longest_known = dist.entry(next.position).or_insert(i32::MAX);

            if next.distance < *longest_known {
                *longest_known = next.distance;
                heap.push(next)
            }
        }
    }

    max_distance
}

fn dfs(
    current: usize,
    target: usize,
    distance: u32,
    graph: &[Vec<(usize, u32)>],
    visited: &mut Vec<bool>,
    max_distance: &mut u32,
) {
    visited[current] = true;

    if current == target {
        *max_distance = (*max_distance).max(distance);
    }

    for (vertex, edge_distance) in graph[current].iter() {
        if !visited[*vertex] {
            dfs(
                *vertex,
                target,
                distance + edge_distance,
                graph,
                visited,
                max_distance,
            );
        }
    }

    visited[current] = false;
}

impl Solution for Day23 {
    type A = u32;
    type B = u32;

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2023/day23.txt")
    }

    fn part_1(&self, input: &str) -> Result<u32, AocError> {
        let grid = parse(input)?;
        let longest = dijkstra(&grid);

        Ok(longest)
    }

    fn part_2(&self, input: &str) -> Result<u32, AocError> {
        let grid = parse(input)?;
        let (graph, start, target, distance) = simplify_graph(&grid);

        let mut max_distance = 0;
        let mut visited = vec![false; graph.len()];

        dfs(
            start,
            target,
            distance,
            &graph,
            &mut visited,
            &mut max_distance,
        );

        Ok(max_distance)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip]
    const EXAMPLE_INPUT: &str =
        "#.#####################\n\
         #.......#########...###\n\
         #######.#########.#.###\n\
         ###.....#.>.>.###.#.###\n\
         ###v#####.#v#.###.#.###\n\
         ###.>...#.#.#.....#...#\n\
         ###v###.#.#.#########.#\n\
         ###...#.#.#.......#...#\n\
         #####.#.#.#######.#.###\n\
         #.....#.#.#.......#...#\n\
         #.#####.#.#.#########v#\n\
         #.#...#...#...###...>.#\n\
         #.#.#v#######v###.###v#\n\
         #...#.>.#...>.>.#.###.#\n\
         #####v#.#.###v#.#.###.#\n\
         #.....#...#...#.#.#...#\n\
         #.#########.###.#.#.###\n\
         #...###...#...#...#.###\n\
         ###.###.#.###v#####v###\n\
         #...#...#.#.>.>.#.>.###\n\
         #.###.###.#.###.#.#v###\n\
         #.....###...###...#...#\n\
         #####################.#\n";

    #[test]
    fn it_solves_part1_example() {
        assert_eq!(Day23.part_1(EXAMPLE_INPUT), Ok(94));
    }

    #[test]
    fn it_solves_part2_example() {
        assert_eq!(Day23.part_2(EXAMPLE_INPUT), Ok(154));
    }
}
