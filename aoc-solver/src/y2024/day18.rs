use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
};

use itertools::Itertools;

use crate::solution::{AocError, Solution};

type Coords = (usize, usize);

const DIRECTIONS: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

#[derive(Clone, Eq, PartialEq)]
struct Search {
    steps: u32,
    position: Coords,
}

impl Ord for Search {
    fn cmp(&self, other: &Self) -> Ordering {
        other.steps.cmp(&self.steps)
    }
}

impl PartialOrd for Search {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse(input: &str) -> Result<Vec<Coords>, AocError> {
    let bytes = input
        .lines()
        .map(|line| {
            line.split_once(",")
                .and_then(|(x, y)| {
                    let pos = (x.parse::<usize>().ok()?, y.parse::<usize>().ok()?);

                    Some(pos)
                })
                .ok_or_else(|| AocError::parse(input, "Invalid input"))
        })
        .try_collect()?;

    Ok(bytes)
}

// Manhattan distance as the heuristic function
fn heuristic(a: Coords, b: Coords) -> u32 {
    (a.0.abs_diff(b.0) + a.1.abs_diff(b.1)) as u32
}

pub fn a_star(corrupted: &[Coords], start: Coords, end: Coords) -> Option<u32> {
    let corrupted: HashSet<&Coords> = corrupted.iter().collect();
    let width = end.0 as isize + 1;
    let height = end.1 as isize + 1;

    if corrupted.contains(&start) || corrupted.contains(&end) {
        return None;
    }

    let mut g = HashMap::new();
    let mut f = HashMap::new();
    let mut open_set = BinaryHeap::new();
    let mut closed_set = HashSet::new();

    g.insert(start, 0);
    f.insert(start, heuristic(start, end));

    open_set.push(Search {
        steps: f[&start],
        position: start,
    });

    while let Some(Search { position, .. }) = open_set.pop() {
        if position == end {
            return g.get(&position).cloned();
        }

        if closed_set.contains(&position) {
            continue;
        }

        closed_set.insert(position);

        for (dx, dy) in DIRECTIONS {
            let (x, y) = (position.0 as isize + dx, position.1 as isize + dy);
            if x < 0 || y < 0 || x >= width || y >= height {
                continue;
            }

            let neighbour = (x as usize, y as usize);

            if corrupted.contains(&neighbour) || closed_set.contains(&neighbour) {
                continue;
            }

            let tentative_g = g.get(&position).unwrap_or(&0) + 1;
            let existing_g = g.get(&neighbour).unwrap_or(&u32::MAX);

            if tentative_g < *existing_g {
                g.insert(neighbour, tentative_g);
                f.insert(neighbour, tentative_g + heuristic(neighbour, end));

                open_set.push(Search {
                    steps: tentative_g + heuristic(neighbour, end),
                    position: neighbour,
                });
            }
        }
    }

    None
}

pub struct Day18;
impl Solution for Day18 {
    type Part1 = u32;
    type Part2 = String;

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2024/day18.txt")
    }

    fn part_1(&self, input: &str) -> Result<u32, AocError> {
        let corrupted = parse(input)?;

        let start = (0, 0);
        let end = (70, 70);
        let min_distance = a_star(&corrupted[0..1024], start, end)
            .ok_or_else(|| AocError::logic("No path found"))?;

        Ok(min_distance)
    }

    fn part_2(&self, input: &str) -> Result<String, AocError> {
        let corrupted = parse(input)?;

        let start = (0, 0);
        let end = (70, 70);

        let mut low = 1024;
        let mut high = corrupted.len();

        while low < high {
            let mid = (low + high) / 2;

            if a_star(&corrupted[0..mid], start, end).is_some() {
                low = mid + 1;
            } else {
                high = mid;
            }
        }

        Ok(format!("{},{}", corrupted[low - 1].0, corrupted[low - 1].1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "5,4\n\
        4,2\n\
        4,5\n\
        3,0\n\
        2,1\n\
        6,3\n\
        2,4\n\
        1,5\n\
        0,6\n\
        3,3\n\
        2,6\n\
        5,1\n\
        1,2\n\
        5,5\n\
        2,5\n\
        6,5\n\
        1,4\n\
        0,4\n\
        6,4\n\
        1,1\n\
        6,1\n\
        1,0\n\
        0,5\n\
        1,6\n\
        2,0";

    #[test]
    fn it_solves_part1_example() {
        let bytes = parse(TEST_INPUT).unwrap();

        let min_distance = a_star(&bytes[0..12], (0, 0), (6, 6));

        assert_eq!(min_distance, Some(22));
    }

    #[test]
    fn it_solves_part2_example() {
        let bytes = parse(TEST_INPUT).unwrap();

        let mut t = 12;

        while a_star(&bytes[0..t], (0, 0), (6, 6)).is_some() {
            t += 1;
        }

        assert_eq!(t, 21);
    }
}
