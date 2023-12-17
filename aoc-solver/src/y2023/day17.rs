use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

use itertools::Itertools;

use crate::solution::{AocError, Solution};

pub struct Day17;

type Coords = (isize, isize);
type Grid = Vec<Vec<u8>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn possible(&self, consequtive: u8, min_consequtive: u8) -> Vec<Direction> {
        if consequtive < min_consequtive {
            return vec![*self];
        }

        match self {
            Direction::North => vec![Direction::North, Direction::East, Direction::West],
            Direction::East => vec![Direction::East, Direction::North, Direction::South],
            Direction::South => vec![Direction::South, Direction::East, Direction::West],
            Direction::West => vec![Direction::West, Direction::North, Direction::South],
        }
    }

    fn delta(&self) -> (isize, isize) {
        match self {
            Direction::North => (0, -1),
            Direction::East => (1, 0),
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
struct Search {
    heat_loss: u32,
    position: Coords,
    direction: Direction,
    consequtive: u8,
}

impl Ord for Search {
    fn cmp(&self, other: &Self) -> Ordering {
        other.heat_loss.cmp(&self.heat_loss)
    }
}

impl PartialOrd for Search {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse(input: &str) -> Result<Grid, AocError> {
    let grid: Grid = input
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|tile| {
                    let heat_loss = tile
                        .to_digit(10)
                        .ok_or(AocError::parse(tile, "Invalid tile"))?;

                    Ok(heat_loss as u8)
                })
                .try_collect()
        })
        .try_collect()?;

    Ok(grid)
}

fn dijkstra(grid: &Grid, min_consequtive: u8, max_consequtive: u8) -> Result<u32, AocError> {
    let mut heat_losses: HashMap<(Coords, Direction, u8), u32> = HashMap::new();
    let mut heap: BinaryHeap<Search> = BinaryHeap::new();

    let height = grid.len() as isize;
    let width = grid[0].len() as isize;
    let target = (width - 1, height - 1);

    heat_losses.insert(((0, 0), Direction::East, 0), 0);
    heap.push(Search {
        position: (0, 0),
        direction: Direction::East,
        consequtive: 0,
        heat_loss: 0,
    });

    while let Some(Search {
        position,
        direction,
        consequtive,
        heat_loss,
    }) = heap.pop()
    {
        if position == target && consequtive >= min_consequtive {
            return Ok(heat_losses[&(position, direction, consequtive)]);
        }

        if heat_loss
            > *heat_losses
                .get(&(position, direction, consequtive))
                .unwrap_or(&u32::MAX)
        {
            continue;
        }

        for next_direction in direction.possible(consequtive, min_consequtive) {
            let (dx, dy) = next_direction.delta();
            let (x, y) = (position.0 + dx, position.1 + dy);

            let next_consequtive = if direction == next_direction {
                consequtive + 1
            } else {
                1
            };

            if x < 0 || y < 0 || x >= width || y >= height || next_consequtive > max_consequtive {
                continue;
            }

            let next = Search {
                position: (x, y),
                direction: next_direction,
                heat_loss: heat_loss + grid[y as usize][x as usize] as u32,
                consequtive: next_consequtive,
            };

            let best_known = heat_losses
                .entry((next.position, next.direction, next.consequtive))
                .or_insert(u32::MAX);

            if next.heat_loss < *best_known {
                *best_known = next.heat_loss;
                heap.push(next)
            }
        }
    }

    Err(AocError::logic("No path found"))
}

impl Solution for Day17 {
    type A = u32;
    type B = u32;

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2023/day17.txt")
    }

    fn part_1(&self, input: &str) -> Result<u32, AocError> {
        let grid = parse(input)?;
        dijkstra(&grid, 0, 3)
    }

    fn part_2(&self, input: &str) -> Result<u32, AocError> {
        let grid = parse(input)?;
        dijkstra(&grid, 4, 10)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_example() {
        assert_eq!(
            Day17.part_1(
                "2413432311323\n\
                 3215453535623\n\
                 3255245654254\n\
                 3446585845452\n\
                 4546657867536\n\
                 1438598798454\n\
                 4457876987766\n\
                 3637877979653\n\
                 4654967986887\n\
                 4564679986453\n\
                 1224686865563\n\
                 2546548887735\n\
                 4322674655533\n"
            ),
            Ok(102)
        );
    }

    #[test]
    fn it_solves_part2_example_1() {
        assert_eq!(
            Day17.part_2(
                "2413432311323\n\
                 3215453535623\n\
                 3255245654254\n\
                 3446585845452\n\
                 4546657867536\n\
                 1438598798454\n\
                 4457876987766\n\
                 3637877979653\n\
                 4654967986887\n\
                 4564679986453\n\
                 1224686865563\n\
                 2546548887735\n\
                 4322674655533\n"
            ),
            Ok(94)
        );
    }

    #[test]
    fn it_solves_part2_example_2() {
        assert_eq!(
            Day17.part_2(
                "111111111111\n\
                 999999999991\n\
                 999999999991\n\
                 999999999991\n\
                 999999999991\n"
            ),
            Ok(71)
        );
    }
}
