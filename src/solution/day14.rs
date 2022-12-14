use std::collections::{BTreeSet, HashMap};

use itertools::Itertools;

use crate::solution::{AocError, Solution};

pub struct Day14;

impl Day14 {
    fn parse(input: &str) -> Result<HashMap<i32, BTreeSet<i32>>, AocError> {
        let mut world = HashMap::new();

        for line in input.lines() {
            let walls = line
                .split(" -> ")
                .map(|point| {
                    let (start, end) = point.split_once(',').unwrap();
                    (start.parse::<i32>().unwrap(), end.parse::<i32>().unwrap())
                })
                .tuple_windows::<(_, _)>();

            for (start, end) in walls {
                if start.1 == end.1 {
                    for x in i32::min(start.0, end.0)..=i32::max(start.0, end.0) {
                        world.entry(x).or_insert(BTreeSet::new()).insert(start.1);
                    }
                } else {
                    for y in i32::min(start.1, end.1)..=i32::max(start.1, end.1) {
                        world.entry(start.0).or_insert(BTreeSet::new()).insert(y);
                    }
                }
            }
        }

        Ok(world)
    }

    fn simulate(mut world: HashMap<i32, BTreeSet<i32>>) -> usize {
        let mut sand_count = 0;
        let source = (500, 0);

        loop {
            let mut sand = source;
            let mut falling = true;

            while falling {
                if let Some(y) = world
                    .get(&sand.0)
                    .and_then(|column| column.range(sand.1 + 1..).next())
                {
                    sand.1 = y - 1;

                    if !world
                        .get(&(sand.0 - 1))
                        .map(|s| s.contains(&(sand.1 + 1)))
                        .unwrap_or(false)
                    {
                        sand.0 -= 1;
                        sand.1 += 1;
                        continue;
                    }

                    if !world
                        .get(&(sand.0 + 1))
                        .map(|s| s.contains(&(sand.1 + 1)))
                        .unwrap_or(false)
                    {
                        sand.0 += 1;
                        sand.1 += 1;
                        continue;
                    }

                    // Comes to rest
                    world
                        .entry(sand.0)
                        .and_modify(|e| {
                            e.insert(sand.1);
                        })
                        .or_default();

                    sand_count += 1;

                    // Sand blocks the source
                    if sand == source {
                        return sand_count;
                    }

                    falling = false;
                } else {
                    // There's nothing solid in this column, the sand will just fall into the void.
                    return sand_count;
                }
            }
        }
    }
}

impl Solution for Day14 {
    type F = usize;
    type S = usize;

    fn name(&self) -> &'static str {
        "Day 14"
    }

    fn default_input(&self) -> &'static str {
        include_str!("../../inputs/day14.txt")
    }

    fn part_1(&self, input: &str) -> Result<usize, AocError> {
        let world = Day14::parse(input)?;

        Ok(Day14::simulate(world))
    }

    fn part_2(&self, input: &str) -> Result<usize, AocError> {
        let mut world = Day14::parse(input)?;
        let floor_level = 2 + world
            .values()
            .flat_map(|column| column.iter().max())
            .max()
            .ok_or_else(|| AocError::logic("no floor"))?;

        // This should be wide enough
        for x in 0..1000 {
            world
                .entry(x)
                .or_insert(BTreeSet::new())
                .insert(floor_level);
        }

        Ok(Day14::simulate(world))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1() {
        assert_eq!(
            Day14.part_1(
                "498,4 -> 498,6 -> 496,6\n\
                 503,4 -> 502,4 -> 502,9 -> 494,9"
            ),
            Ok(24)
        );
    }

    #[test]
    fn it_solves_part2() {
        assert_eq!(
            Day14.part_2(
                "498,4 -> 498,6 -> 496,6\n\
                 503,4 -> 502,4 -> 502,9 -> 494,9"
            ),
            Ok(93)
        );
    }
}
