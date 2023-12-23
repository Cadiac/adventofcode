use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;

use crate::solution::{AocError, Solution};

pub struct Day22;

#[derive(Debug, Clone)]
struct Coords {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Debug, Clone)]
struct Brick {
    id: usize,
    left: Coords,
    right: Coords,
}

fn parse(input: &str) -> Result<Vec<Brick>, AocError> {
    let bricks = input
        .trim()
        .lines()
        .enumerate()
        .map(|(id, line)| {
            let (left, right) = line
                .split_once('~')
                .ok_or(AocError::parse(line, "Missing ~ split"))?;

            let left = parse_coordinates(left)?;
            let right = parse_coordinates(right)?;

            Ok(Brick { id, left, right })
        })
        .try_collect()?;

    Ok(bricks)
}

fn parse_coordinates(input: &str) -> Result<Coords, AocError> {
    let coords = match input.split(',').collect_tuple() {
        Some((x, y, z)) => Coords {
            x: x.parse::<i32>().map_err(|err| AocError::parse(x, err))?,
            y: y.parse::<i32>().map_err(|err| AocError::parse(y, err))?,
            z: z.parse::<i32>().map_err(|err| AocError::parse(z, err))?,
        },
        None => return Err(AocError::parse(input, "Wrong amount of coordinates")),
    };

    Ok(coords)
}

fn is_supporting(a: &Brick, b: &Brick) -> bool {
    if a.id == b.id || a.right.z + 1 != b.left.z {
        return false;
    }

    is_overlapping(a, b)
}

fn is_overlapping(a: &Brick, b: &Brick) -> bool {
    let horizontal = a.left.x <= b.right.x && a.right.x >= b.left.x;
    let vertical = a.left.y <= b.right.y && a.right.y >= b.left.y;

    horizontal && vertical
}

fn find_supports(dropped: Vec<Brick>) -> HashMap<usize, HashSet<usize>> {
    let bricks_with_supports = dropped
        .iter()
        .map(|brick| {
            let supporting_bricks: HashSet<usize> = dropped
                .iter()
                .filter(|potential_support| is_supporting(potential_support, brick))
                .map(|support| support.id)
                .collect();

            (brick.id, supporting_bricks)
        })
        .collect();

    bricks_with_supports
}

fn apply_gravity(mut bricks: Vec<Brick>) -> Vec<Brick> {
    bricks.sort_by(|a, b| a.left.z.cmp(&b.left.z));

    let mut queue = VecDeque::from(bricks);
    let mut dropped = Vec::new();

    while let Some(mut brick) = queue.pop_front() {
        let resting_height = dropped
            .iter()
            .filter(|stationary| is_overlapping(&brick, stationary))
            .map(|b| b.right.z)
            .max()
            .unwrap_or(0)
            + 1;

        brick.right.z = resting_height + brick.right.z - brick.left.z;
        brick.left.z = resting_height;

        dropped.push(brick);
    }

    dropped
}

impl Solution for Day22 {
    type A = u32;
    type B = u32;

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2023/day22.txt")
    }

    fn part_1(&self, input: &str) -> Result<u32, AocError> {
        let bricks = parse(input)?;

        let dropped = apply_gravity(bricks);
        let bricks_with_supports = find_supports(dropped);

        let safe_to_disintegrate = bricks_with_supports
            .keys()
            .filter(|id| {
                !bricks_with_supports
                    .values()
                    .any(|supports| supports.len() == 1 && supports.contains(id))
            })
            .count() as u32;

        Ok(safe_to_disintegrate)
    }

    fn part_2(&self, input: &str) -> Result<u32, AocError> {
        let bricks = parse(input)?;

        let dropped = apply_gravity(bricks);
        let bricks_with_supports = find_supports(dropped);

        let mut memo: HashMap<usize, HashSet<usize>> = HashMap::new();

        let total_falls: u32 = bricks_with_supports
            .keys()
            .map(|disintegrated| {
                let mut dropped_supports = HashSet::from([*disintegrated]);

                loop {
                    let mut fallen_bricks = HashSet::new();

                    for (brick, supports) in bricks_with_supports.iter() {
                        if !dropped_supports.contains(brick)
                            && !supports.is_empty()
                            && supports.is_subset(&dropped_supports)
                        {
                            fallen_bricks.insert(*brick);

                            // If we've previously dropped a long chain that starts from
                            // this brick falling reading that from the cache will result
                            // in a nice +200% performance gain. But on smaller inputs this
                            // often just returns the same brick that we just inserted.
                            if let Some(cached_chain_reaction) = memo.get(brick) {
                                fallen_bricks.extend(cached_chain_reaction);
                            }
                        }
                    }

                    if fallen_bricks.is_empty() {
                        // Nothing moved so the chain reaction stopped.
                        // Don't count the disintegrated brick that started the reaction
                        let other_fallen_bricks = dropped_supports.len() as u32 - 1;
                        memo.insert(*disintegrated, dropped_supports);

                        return other_fallen_bricks;
                    }

                    // To avoid checking bricks we've fully tested already
                    dropped_supports.extend(fallen_bricks);
                }
            })
            .sum();

        Ok(total_falls as u32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_example() {
        assert_eq!(
            Day22.part_1(
                "1,0,1~1,2,1\n\
                 0,0,2~2,0,2\n\
                 0,2,3~2,2,3\n\
                 0,0,4~0,2,4\n\
                 2,0,5~2,2,5\n\
                 0,1,6~2,1,6\n\
                 1,1,8~1,1,9\n"
            ),
            Ok(5)
        );
    }

    #[test]
    fn it_solves_part2_example() {
        assert_eq!(
            Day22.part_2(
                "1,0,1~1,2,1\n\
                 0,0,2~2,0,2\n\
                 0,2,3~2,2,3\n\
                 0,0,4~0,2,4\n\
                 2,0,5~2,2,5\n\
                 0,1,6~2,1,6\n\
                 1,1,8~1,1,9\n"
            ),
            Ok(7)
        );
    }

    #[test]
    fn it_solves_part2_real() {
        assert_eq!(Day22.part_2(Day22.default_input()), Ok(67468));
    }
}
