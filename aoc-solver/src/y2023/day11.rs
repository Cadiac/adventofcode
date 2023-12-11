use std::collections::HashSet;

use itertools::Itertools;

use crate::solution::{AocError, Solution};

pub struct Day11;

type Point = (usize, usize);
type Universe = HashSet<Point>;

fn parse(input: &str) -> Result<Universe, AocError> {
    let universe = input
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(y, line)| line.chars().enumerate().filter_map(|(x, c)| {
            if c == '#' {
                return Some((x,y));
            }
            None
        }).collect::<Vec<_>>())
        .collect();

    Ok(universe)
}

fn max_x(map: &Universe) -> usize {
    map
        .iter()
        .max_by(|a, b| a.0.cmp(&b.0)).unwrap_or(&(0,0)).0
}

fn max_y(map: &Universe) -> usize {
    map
        .iter()
        .max_by(|a, b| a.1.cmp(&b.1)).unwrap_or(&(0,0)).1
}

fn is_empty_row(map: &Universe, row: usize) -> bool {
    map.iter().all(|(_x, y)| *y != row)
}

fn is_empty_column(map: &Universe, column: usize) -> bool {
    map.iter().all(|(x, _y)| *x != column)
}

fn expand_universe(mut map: Universe, rate: usize) -> Universe {
    let (mut x, mut y) = (0, 0);

    while y <= max_y(&map) {
        if is_empty_row(&map, y) {
            let mut next_map = HashSet::new();

            for item in map {
                if item.1 > y {
                    next_map.insert((item.0, item.1 + rate - 1));
                } else {
                    next_map.insert(item);
                }
            }

            map = next_map;

            y += rate
        } else {
            y += 1
        }
    }

    while x <= max_x(&map) {
        if is_empty_column(&map, x) {
            let mut next_map = HashSet::new();

            for item in map {
                if item.0 > x {
                    next_map.insert((item.0 + rate - 1, item.1));
                } else {
                    next_map.insert(item);
                }
            }

            map = next_map;

            x += rate
        } else {
            x += 1
        }
    }

    map
}

fn distances(map: &Universe) -> i64 {
    let mut distance = 0;

    for (first, second) in map.iter().tuple_combinations() {
        distance += (first.0 as i64 - second.0 as i64).abs() + (first.1 as i64 - second.1 as i64).abs();
    }

    distance
}

fn solve(input: &str, rate: usize) -> Result<i64, AocError> {
    let universe = parse(input)?;
    let expanded = expand_universe(universe, rate);
    let sum = distances(&expanded);

    Ok(sum)
}

impl Solution for Day11 {
    type A = i64;
    type B = i64;

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2023/day11.txt")
    }

    fn part_1(&self, input: &str) -> Result<i64, AocError> {
        solve(input, 2)
    }

    fn part_2(&self, input: &str) -> Result<i64, AocError> {
        solve(input, 1000000)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_example() {
        assert_eq!(solve(
            "...#......\n\
             .......#..\n\
             #.........\n\
             ..........\n\
             ......#...\n\
             .#........\n\
             .........#\n\
             ..........\n\
             .......#..\n\
             #...#.....
        ", 2), Ok(374));
    }

    #[test]
    fn it_solves_part2_example_1() {
        assert_eq!(solve(
            "...#......\n\
             .......#..\n\
             #.........\n\
             ..........\n\
             ......#...\n\
             .#........\n\
             .........#\n\
             ..........\n\
             .......#..\n\
             #...#.....
        ", 10), Ok(1030));
    }

    #[test]
    fn it_solves_part2_example_2() {
        assert_eq!(solve(
            "...#......\n\
             .......#..\n\
             #.........\n\
             ..........\n\
             ......#...\n\
             .#........\n\
             .........#\n\
             ..........\n\
             .......#..\n\
             #...#.....
        ", 100), Ok(8410));
    }
}
