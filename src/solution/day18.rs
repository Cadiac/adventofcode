use serde_scan::ScanError;
use std::collections::{HashSet, VecDeque};

use crate::solution::{AocError, Solution};

type Point = (i32, i32, i32);

const NEIGHBOURS: [Point; 6] = [
    (-1, 0, 0),
    (1, 0, 0),
    (0, -1, 0),
    (0, 1, 0),
    (0, 0, -1),
    (0, 0, 1),
];

pub struct Day18;

impl Day18 {
    fn parse(input: &str) -> Result<HashSet<Point>, AocError> {
        input
            .lines()
            .map(|line| serde_scan::scan!("{},{},{}" <- line))
            .collect::<Result<_, ScanError>>()
            .map_err(|err| AocError::parse("input", err))
    }

    fn count_exposed_sides(cubes: &HashSet<Point>) -> usize {
        cubes
            .iter()
            .map(|cube| {
                let mut exposed_sides = 0;

                for (x, y, z) in NEIGHBOURS {
                    let neighbour = (cube.0 + x, cube.1 + y, cube.2 + z);

                    if !cubes.contains(&neighbour) {
                        exposed_sides += 1;
                    }
                }

                exposed_sides
            })
            .sum()
    }

    fn count_exterior(cubes: HashSet<Point>) -> usize {
        let min_x = cubes.iter().map(|(x, _, _)| x).min().unwrap_or(&0) - 1;
        let max_x = cubes.iter().map(|(x, _, _)| x).max().unwrap_or(&0) + 1;

        let min_y = cubes.iter().map(|(_, y, _)| y).min().unwrap_or(&0) - 1;
        let max_y = cubes.iter().map(|(_, y, _)| y).max().unwrap_or(&0) + 1;

        let min_z = cubes.iter().map(|(_, _, z)| z).min().unwrap_or(&0) - 1;
        let max_z = cubes.iter().map(|(_, _, z)| z).max().unwrap_or(&0) + 1;

        let mut queue = VecDeque::new();
        let mut exterior: HashSet<Point> = HashSet::new();

        let start = (min_x, min_y, min_z);
        exterior.insert(start);
        queue.push_back(start);

        // BFS the exterior points
        while let Some(v) = queue.pop_front() {
            for (x, y, z) in NEIGHBOURS {
                let neighbour = (v.0 + x, v.1 + y, v.2 + z);

                if neighbour.0 >= min_x
                    && neighbour.0 <= max_x
                    && neighbour.1 >= min_y
                    && neighbour.1 <= max_y
                    && neighbour.2 >= min_z
                    && neighbour.2 <= max_z
                {
                    if !cubes.contains(&neighbour) && !exterior.contains(&neighbour) {
                        exterior.insert(neighbour);
                        queue.push_back(neighbour);
                    }
                }
            }
        }

        cubes
            .iter()
            .map(|cube| {
                let mut exposed_sides = 0;

                for (x, y, z) in NEIGHBOURS {
                    let neighbour = (cube.0 + x, cube.1 + y, cube.2 + z);

                    if exterior.contains(&neighbour) {
                        exposed_sides += 1;
                    }
                }

                exposed_sides
            })
            .sum()
    }
}

impl Solution for Day18 {
    type F = usize;
    type S = usize;

    fn name(&self) -> &'static str {
        "Day 18"
    }

    fn default_input(&self) -> &'static str {
        include_str!("../../inputs/day18.txt")
    }

    fn part_1(&self, input: &str) -> Result<usize, AocError> {
        let cubes = Day18::parse(input)?;

        Ok(Day18::count_exposed_sides(&cubes))
    }

    fn part_2(&self, input: &str) -> Result<usize, AocError> {
        let cubes = Day18::parse(input)?;

        Ok(Day18::count_exterior(cubes))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2,2,2\n\
        1,2,2\n\
        3,2,2\n\
        2,1,2\n\
        2,3,2\n\
        2,2,1\n\
        2,2,3\n\
        2,2,4\n\
        2,2,6\n\
        1,2,5\n\
        3,2,5\n\
        2,1,5\n\
        2,3,5";

    #[test]
    fn it_solves_part1_small() {
        assert_eq!(Day18.part_1("1,1,1\n2,1,1"), Ok(10));
    }

    #[test]
    fn it_solves_part1_large() {
        assert_eq!(Day18.part_1(INPUT), Ok(64));
    }

    #[test]
    fn it_solves_part2() {
        assert_eq!(Day18.part_2(INPUT), Ok(58));
    }
}
