use std::collections::HashSet;

use itertools::Itertools;

use crate::solution::{AocError, Solution};

pub struct Day21;

type Coords = (usize, usize);

enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn as_delta(&self) -> (i8, i8) {
        match self {
            Direction::North => (0, -1),
            Direction::East => (1, 0),
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
        }
    }
}

const DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::East,
    Direction::South,
    Direction::West,
];

#[derive(PartialEq)]
enum Tile {
    GardenPlot,
    Rock,
    Start,
}

fn parse(input: &str) -> Result<(Vec<Vec<Tile>>, Coords), AocError> {
    let mut start = None;

    let instructions = input
        .trim()
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, tile)| match tile {
                    '.' => Ok(Tile::GardenPlot),
                    '#' => Ok(Tile::Rock),
                    'S' => {
                        start = Some((x, y));
                        Ok(Tile::Start)
                    }
                    _ => Err(AocError::parse(tile, "Unexpected tile")),
                })
                .try_collect()
        })
        .try_collect()?;

    let start = start.ok_or(AocError::parse(input, "Missing starting position"))?;

    Ok((instructions, start))
}

impl Solution for Day21 {
    type A = usize;
    type B = u64;

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2023/day21.txt")
    }

    fn part_1(&self, input: &str) -> Result<usize, AocError> {
        let (tiles, start) = parse(input)?;
        let count = find_reachable(start, tiles, 64);

        Ok(count)
    }

    fn part_2(&self, _input: &str) -> Result<u64, AocError> {
        todo!();
    }
}

fn find_reachable(start: (usize, usize), tiles: Vec<Vec<Tile>>, steps: usize) -> usize {
    let mut stack = vec![(start, 0)];
    let mut visited: HashSet<(Coords, usize)> = HashSet::new();
    let mut unique: HashSet<Coords> = HashSet::new();

    let width = tiles[0].len();
    let height = tiles.len();

    while let Some(((x, y), distance)) = stack.pop() {
        if !visited.insert(((x, y), distance)) {
            continue;
        }

        if distance == steps {
            unique.insert((x, y));
            continue;
        }

        for (dx, dy) in DIRECTIONS.iter().map(|dir| dir.as_delta()) {
            let (tx, ty) = (x as isize + dx as isize, y as isize + dy as isize);

            if tx >= 0
                && ty >= 0
                && (tx as usize) < width
                && (ty as usize) < height
                && tiles[ty as usize][tx as usize] != Tile::Rock
            {
                stack.push(((tx as usize, ty as usize), distance + 1));
            }
        }
    }

    unique.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip]
    const EXAMPLE_INPUT: &str =
        "...........\n\
         .....###.#.\n\
         .###.##..#.\n\
         ..#.#...#..\n\
         ....#.#....\n\
         .##..S####.\n\
         .##..#...#.\n\
         .......##..\n\
         .##.#.####.\n\
         .##..##.##.\n\
         ...........\n";

    #[test]
    fn it_solves_part1_example() {
        let parsed = parse(EXAMPLE_INPUT);
        assert!(parsed.is_ok());

        let (tiles, start) = parsed.unwrap();

        assert_eq!(find_reachable(start, tiles, 6), 16)
    }
}
