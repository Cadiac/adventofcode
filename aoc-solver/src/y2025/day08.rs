use crate::solution::{AocError, Solution};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

type Coords = (i64, i64, i64);
type Graph = HashMap<Coords, HashSet<Coords>>;

pub struct Day08;

fn parse(input: &str) -> Result<Graph, AocError> {
    let graph = input
        .trim()
        .lines()
        .map(|line| {
            let mut parts = line.split(",");
            match (parts.next(), parts.next(), parts.next()) {
                (Some(x), Some(y), Some(z)) => {
                    let x = x.parse::<i64>().map_err(|err| AocError::parse(x, err))?;
                    let y = y.parse::<i64>().map_err(|err| AocError::parse(y, err))?;
                    let z = z.parse::<i64>().map_err(|err| AocError::parse(z, err))?;

                    Ok(((x, y, z), HashSet::new()))
                }
                _ => Err(AocError::parse(line, "invalid input")),
            }
        })
        .try_collect()?;

    Ok(graph)
}

fn distance(a: Coords, b: Coords) -> f64 {
    f64::sqrt(((a.0 - b.0).pow(2) + (a.1 - b.1).pow(2) + (a.2 - b.2).pow(2)) as f64)
}

fn is_connected(graph: &Graph, start: Coords, target: Coords) -> bool {
    if start == target {
        return true;
    }

    let mut visited = HashSet::new();
    let mut stack = vec![start];

    while let Some(node) = stack.pop() {
        if !visited.insert(node) {
            continue;
        }

        if node == target {
            return true;
        }

        if let Some(neighbors) = graph.get(&node) {
            for &edge in neighbors {
                if !visited.contains(&edge) {
                    stack.push(edge);
                }
            }
        }
    }

    false
}

fn traverse(graph: &Graph, start: Coords, visited: &mut HashSet<Coords>) {
    let mut stack = vec![start];

    while let Some(node) = stack.pop() {
        if !visited.insert(node) {
            continue;
        }

        if let Some(neighbors) = graph.get(&node) {
            for &next in neighbors {
                if !visited.contains(&next) {
                    stack.push(next);
                }
            }
        }
    }
}

fn group_size(graph: &Graph, start: Coords) -> usize {
    let mut visited = HashSet::new();
    traverse(graph, start, &mut visited);
    visited.len()
}

fn group_sizes(graph: &Graph) -> Vec<usize> {
    let mut visited = HashSet::new();
    let mut sizes = Vec::new();

    for &node in graph.keys() {
        if !visited.contains(&node) {
            let before = visited.len();
            traverse(graph, node, &mut visited);
            let after = visited.len();

            sizes.push(after - before);
        }
    }

    sizes
}

fn checksum(graph: &Graph) -> usize {
    group_sizes(graph).iter().sorted().rev().take(3).product()
}

fn find_closest_pairs(graph: &Graph) -> Vec<(Coords, Coords)> {
    graph
        .keys()
        .copied()
        .tuple_combinations()
        .map(|(a, b)| (a, b, distance(a, b)))
        .sorted_by(|(_, _, d1), (_, _, d2)| d1.partial_cmp(d2).unwrap_or(std::cmp::Ordering::Equal))
        .map(|(a, b, _)| (a, b))
        .collect()
}

fn connect_pairs(graph: &mut Graph, count: usize) {
    for (a, b) in find_closest_pairs(graph).into_iter().take(count) {
        if !is_connected(&*graph, a, b) {
            graph.entry(a).or_default().insert(b);
            graph.entry(b).or_default().insert(a);
        }
    }
}

fn connect_all(graph: &mut Graph) -> i64 {
    for (a, b) in find_closest_pairs(graph).into_iter() {
        if !is_connected(&*graph, a, b) {
            graph.entry(a).or_default().insert(b);
            graph.entry(b).or_default().insert(a);

            if group_size(graph, a) == graph.len() {
                return a.0 * b.0;
            }
        }
    }

    0
}

impl Solution for Day08 {
    type Part1 = usize;
    type Part2 = i64;

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2025/day08.txt")
    }

    fn part_1(&self, input: &str) -> Result<usize, AocError> {
        let mut graph = parse(input)?;
        connect_pairs(&mut graph, 1000);

        Ok(checksum(&graph))
    }

    fn part_2(&self, input: &str) -> Result<i64, AocError> {
        let mut graph = parse(input)?;
        let result = connect_all(&mut graph);

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "162,817,812\n\
                           57,618,57\n\
                           906,360,560\n\
                           592,479,940\n\
                           352,342,300\n\
                           466,668,158\n\
                           542,29,236\n\
                           431,825,988\n\
                           739,650,466\n\
                           52,470,668\n\
                           216,146,977\n\
                           819,987,18\n\
                           117,168,530\n\
                           805,96,715\n\
                           346,949,466\n\
                           970,615,88\n\
                           941,993,340\n\
                           862,61,35\n\
                           984,92,344\n\
                           425,690,689";

    #[test]
    fn it_solves_part1_example() {
        let mut graph = parse(EXAMPLE).unwrap();
        connect_pairs(&mut graph, 10);
        assert_eq!(checksum(&graph), 40);
    }

    #[test]
    fn it_solves_part2_example() {
        assert_eq!(Day08.part_2(EXAMPLE), Ok(25272));
    }
}
