use std::collections::HashMap;

use itertools::Itertools;

use crate::solution::{AocError, Solution};

type Grid = Vec<Vec<u32>>;

struct Search {
    elevation: u32,
    coords: (usize, usize),
}

const DIRECTIONS: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

fn parse(input: &str) -> Result<Grid, AocError> {
    let area = input
        .trim()
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| {
                    c.to_digit(10)
                        .ok_or_else(|| AocError::parse(c.to_string(), "Unexpected character"))
                })
                .collect()
        })
        .try_collect()?;

    Ok(area)
}

fn is_within_bounds(x: isize, y: isize, width: isize, height: isize) -> bool {
    x >= 0 && y >= 0 && x < width && y < height
}

fn search_trails((x, y): (usize, usize), grid: &[Vec<u32>]) -> HashMap<(usize, usize), usize> {
    let mut stack: Vec<Search> = Vec::new();

    let width = grid[0].len() as isize;
    let height = grid.len() as isize;

    stack.push(Search {
        elevation: grid[y][x],
        coords: (x, y),
    });

    let mut trails: HashMap<(usize, usize), usize> = HashMap::new();

    while let Some(Search { coords, elevation }) = stack.pop() {
        if elevation == 9 {
            *trails.entry(coords).or_insert(0) += 1;
            continue;
        }

        for (dx, dy) in DIRECTIONS {
            let next: (isize, isize) = (coords.0 as isize + dx, coords.1 as isize + dy);

            if is_within_bounds(next.0, next.1, width, height) {
                let next_elevation = grid[next.1 as usize][next.0 as usize];

                if next_elevation.saturating_sub(elevation) != 1 {
                    // Trail should always increase by a height of exactly 1 at each step
                    continue;
                }

                stack.push(Search {
                    elevation: next_elevation,
                    coords: (next.0 as usize, next.1 as usize),
                });
            }
        }
    }

    trails
}

pub struct Day10;
impl Solution for Day10 {
    type A = usize;
    type B = usize;

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2024/day10.txt")
    }

    fn part_1(&self, input: &str) -> Result<usize, AocError> {
        let grid = parse(input)?;
        let mut score = 0;

        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                if grid[y][x] == 0 {
                    let peaks = search_trails((x, y), &grid);
                    score += peaks.len()
                }
            }
        }

        Ok(score)
    }

    fn part_2(&self, input: &str) -> Result<usize, AocError> {
        let grid = parse(input)?;
        let mut score = 0;

        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                if grid[y][x] == 0 {
                    let peaks = search_trails((x, y), &grid);
                    let trails: usize = peaks.values().sum();
                    score += trails
                }
            }
        }

        Ok(score)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_example_1() {
        assert_eq!(
            Day10.part_1(
                "89010123\n\
                 78121874\n\
                 87430965\n\
                 96549874\n\
                 45678903\n\
                 32019012\n\
                 01329801\n\
                 10456732"
            ),
            Ok(36)
        );
    }

    #[test]
    fn it_solves_part1_example_2() {
        assert_eq!(
            Day10.part_2(
                "89010123\n\
                 78121874\n\
                 87430965\n\
                 96549874\n\
                 45678903\n\
                 32019012\n\
                 01329801\n\
                 10456732"
            ),
            Ok(81)
        );
    }
}
