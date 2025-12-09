use std::collections::{HashMap, HashSet};

pub type Coords = (usize, usize);

use crate::solution::{AocError, Solution};

pub struct Day07;

type Splitters = HashSet<Coords>;

fn parse(input: &str) -> Result<(Coords, usize, Splitters), AocError> {
    let mut start = None;
    let splitters = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .flat_map(|(x, c)| match c {
                    '^' => Some((x, y)),
                    'S' => {
                        start = Some((x, y));
                        None
                    }
                    _ => None,
                })
                .collect::<Vec<_>>()
        })
        .collect();

    let start = start.ok_or(AocError::parse(input, "missing start"))?;
    let height = input.lines().count();

    Ok((start, height, splitters))
}

fn count_splits(start: (usize, usize), height: usize, splitters: HashSet<(usize, usize)>) -> usize {
    let mut current = HashSet::from([start]);
    let mut splits = 0;

    while !current.is_empty() {
        let mut next = HashSet::new();

        for (x, y) in current {
            let next_y = y + 1;
            if next_y >= height {
                continue;
            }

            if splitters.contains(&(x, next_y)) {
                next.insert((x - 1, next_y));
                next.insert((x + 1, next_y));
                splits += 1;
            } else {
                next.insert((x, next_y));
            }
        }

        current = next;
    }
    splits
}

fn count_timelines(
    memo: &mut HashMap<Coords, usize>,
    current: Coords,
    height: usize,
    splitters: &Splitters,
) -> usize {
    if let Some(cached) = memo.get(&current) {
        return *cached;
    }

    let next = current.1 + 1;
    if next >= height {
        return 1;
    }

    let result = if splitters.contains(&(current.0, next)) {
        let l = count_timelines(memo, (current.0 - 1, next), height, splitters);
        let r = count_timelines(memo, (current.0 + 1, next), height, splitters);

        l + r
    } else {
        count_timelines(memo, (current.0, next), height, splitters)
    };

    memo.insert(current, result);
    result
}

impl Solution for Day07 {
    type Part1 = usize;
    type Part2 = usize;

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2025/day07.txt")
    }

    fn part_1(&self, input: &str) -> Result<usize, AocError> {
        let (start, height, splitters) = parse(input)?;
        let splits = count_splits(start, height, splitters);

        Ok(splits)
    }

    fn part_2(&self, input: &str) -> Result<usize, AocError> {
        let (start, height, splitters) = parse(input)?;
        let timelines = count_timelines(&mut HashMap::new(), start, height, &splitters);

        Ok(timelines)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = ".......S.......\n\
                           ...............\n\
                           .......^.......\n\
                           ...............\n\
                           ......^.^......\n\
                           ...............\n\
                           .....^.^.^.....\n\
                           ...............\n\
                           ....^.^...^....\n\
                           ...............\n\
                           ...^.^...^.^...\n\
                           ...............\n\
                           ..^...^.....^..\n\
                           ...............\n\
                           .^.^.^.^.^...^.\n\
                           ...............";

    const SIMPLE: &str = ".......S.......\n\
                          ...............\n\
                          .......^.......\n\
                          ...............\n\
                          ......^........\n\
                          ...............";

    #[test]
    fn it_solves_part1_example() {
        assert_eq!(Day07.part_1(EXAMPLE), Ok(21));
    }

    #[test]
    fn it_solves_part2_example() {
        assert_eq!(Day07.part_2(EXAMPLE), Ok(40));
    }

    #[test]
    fn it_solves_part2_example2() {
        assert_eq!(Day07.part_2(SIMPLE), Ok(3));
    }
}
