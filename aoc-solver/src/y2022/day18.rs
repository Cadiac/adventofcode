use serde_scan::ScanError;
use std::collections::{hash_map::Entry, HashMap, HashSet, VecDeque};

use crate::solution::{AocError, Solution};

type Point = (i32, i32, i32);
type Bounds = ((i32, i32), (i32, i32), (i32, i32));

const NEIGHBOURS: [Point; 6] = [
    (-1, 0, 0),
    (1, 0, 0),
    (0, -1, 0),
    (0, 1, 0),
    (0, 0, -1),
    (0, 0, 1),
];

pub enum Label {
    Exterior,
    Lava,
    Pocket,
}

pub struct Day18;

impl Day18 {
    pub fn parse(input: &str) -> Result<HashSet<Point>, AocError> {
        input
            .lines()
            .map(|line| serde_scan::scan!("{},{},{}" <- line))
            .collect::<Result<_, ScanError>>()
            .map_err(|err| AocError::parse("input", err))
    }

    fn count_exposed_sides(lava_cubes: &HashSet<Point>) -> usize {
        lava_cubes
            .iter()
            .map(|cube| {
                let mut exposed_sides = 0;

                for (x, y, z) in NEIGHBOURS {
                    let neighbour = (cube.0 + x, cube.1 + y, cube.2 + z);

                    if !lava_cubes.contains(&neighbour) {
                        exposed_sides += 1;
                    }
                }

                exposed_sides
            })
            .sum()
    }

    fn bounds(lava_cubes: &HashSet<Point>) -> Bounds {
        let min_x = lava_cubes.iter().map(|(x, _, _)| x).min().unwrap_or(&0) - 1;
        let max_x = lava_cubes.iter().map(|(x, _, _)| x).max().unwrap_or(&0) + 1;

        let min_y = lava_cubes.iter().map(|(_, y, _)| y).min().unwrap_or(&0) - 1;
        let max_y = lava_cubes.iter().map(|(_, y, _)| y).max().unwrap_or(&0) + 1;

        let min_z = lava_cubes.iter().map(|(_, _, z)| z).min().unwrap_or(&0) - 1;
        let max_z = lava_cubes.iter().map(|(_, _, z)| z).max().unwrap_or(&0) + 1;

        ((min_x, max_x), (min_y, max_y), (min_z, max_z))
    }

    fn is_within_bounds((x, y, z): (i32, i32, i32), bounds: Bounds) -> bool {
        x >= bounds.0 .0
            && x <= bounds.0 .1
            && y >= bounds.1 .0
            && y <= bounds.1 .1
            && z >= bounds.2 .0
            && z <= bounds.2 .1
    }

    fn count_exterior(lava_cubes: HashSet<Point>) -> usize {
        let bounds = Day18::bounds(&lava_cubes);

        let mut queue = VecDeque::new();
        let mut exterior: HashSet<Point> = HashSet::new();

        let start = (bounds.0 .0, bounds.1 .0, bounds.2 .0);
        exterior.insert(start);
        queue.push_back(start);

        // BFS the exterior points
        while let Some(v) = queue.pop_front() {
            for (x, y, z) in NEIGHBOURS {
                let neighbour = (v.0 + x, v.1 + y, v.2 + z);

                if Day18::is_within_bounds(neighbour, bounds)
                    && !lava_cubes.contains(&neighbour)
                    && !exterior.contains(&neighbour)
                {
                    exterior.insert(neighbour);
                    queue.push_back(neighbour);
                }
            }
        }

        lava_cubes
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

    // For web visualization
    pub fn label_cubes(lava_cubes: HashSet<Point>) -> (HashMap<Point, Label>, Bounds) {
        let bounds = Day18::bounds(&lava_cubes);
        let mut queue = VecDeque::new();
        let mut labeled: HashMap<Point, Label> = HashMap::new();

        for cube in lava_cubes.iter() {
            labeled.insert(*cube, Label::Lava);
        }

        let start = (bounds.0 .0, bounds.1 .0, bounds.2 .0);
        labeled.insert(start, Label::Exterior);
        queue.push_back(start);

        // BFS the exterior points
        while let Some(v) = queue.pop_front() {
            for (x, y, z) in NEIGHBOURS {
                let neighbour = (v.0 + x, v.1 + y, v.2 + z);

                if Day18::is_within_bounds(neighbour, bounds) {
                    if let Entry::Vacant(e) = labeled.entry(neighbour) {
                        e.insert(Label::Exterior);
                        queue.push_back(neighbour);
                    }
                }
            }
        }

        for z in bounds.2 .0..bounds.2 .1 {
            for y in bounds.1 .0..bounds.1 .1 {
                for x in bounds.0 .0..bounds.0 .1 {
                    labeled.entry((x, y, z)).or_insert(Label::Pocket);
                }
            }
        }

        (labeled, bounds)
    }
}

impl Solution for Day18 {
    type F = usize;
    type S = usize;

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2022/day18.txt")
    }

    fn part_1(&self, input: &str) -> Result<usize, AocError> {
        let lava_cubes = Day18::parse(input)?;

        Ok(Day18::count_exposed_sides(&lava_cubes))
    }

    fn part_2(&self, input: &str) -> Result<usize, AocError> {
        let lava_cubes = Day18::parse(input)?;

        Ok(Day18::count_exterior(lava_cubes))
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
