use std::collections::HashMap;

use itertools::Itertools;

use crate::solution::{AocError, Solution};

pub struct Day12;

#[derive(Debug, Clone, Copy)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

#[derive(Debug)]
struct Record {
    groups: Vec<usize>,
    springs: Vec<Spring>,
}

fn parse(input: &str) -> Result<Vec<Record>, AocError> {
    input
        .trim()
        .lines()
        .map(|line| {
            let (springs_str, groups_str) = line
                .split_once(' ')
                .ok_or(AocError::parse(line, "Missing springs or groups"))?;

            let springs = springs_str
                .chars()
                .map(|spring| match spring {
                    '.' => Ok(Spring::Operational),
                    '#' => Ok(Spring::Damaged),
                    '?' => Ok(Spring::Unknown),
                    _ => Err(AocError::parse(spring, "Unexpected symbol")),
                })
                .try_collect()?;

            let groups = groups_str
                .split(',')
                .map(|number| {
                    number
                        .parse::<usize>()
                        .map_err(|_| AocError::parse(number, "Error parsing number"))
                })
                .try_collect()?;

            Ok(Record { springs, groups })
        })
        .try_collect()
}

fn find_possible(
    memo: &mut HashMap<(usize, usize, usize), usize>,
    record: &Record,
    spring_index: usize,
    group_index: usize,
    contiguous: usize,
) -> usize {
    if let Some(cached) = memo.get(&(spring_index, group_index, contiguous)) {
        return *cached;
    }

    if spring_index == record.springs.len() {
        let is_at_end_done = group_index == record.groups.len() && contiguous == 0;
        let is_at_last_just_finishing =
            group_index == record.groups.len() - 1 && record.groups[group_index] == contiguous;

        if is_at_end_done || is_at_last_just_finishing {
            return 1;
        }

        return 0;
    }

    let mut possible = 0;

    if matches!(
        record.springs[spring_index],
        Spring::Operational | Spring::Unknown,
    ) {
        if contiguous == 0 {
            // Currently not gathering a group, move to next spring
            possible += find_possible(memo, record, spring_index + 1, group_index, 0)
        } else if *record.groups.get(group_index).unwrap_or(&0) == contiguous {
            // A group ended just before this spring
            possible += find_possible(memo, record, spring_index + 1, group_index + 1, 0)
        }
    }

    if matches!(
        record.springs[spring_index],
        Spring::Damaged | Spring::Unknown,
    ) {
        possible += find_possible(memo, record, spring_index + 1, group_index, contiguous + 1)
    }

    memo.insert((spring_index, group_index, contiguous), possible);
    possible
}

impl Solution for Day12 {
    type A = usize;
    type B = usize;

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2023/day12.txt")
    }

    fn part_1(&self, input: &str) -> Result<usize, AocError> {
        let records = parse(input)?;

        let total = records
            .iter()
            .map(|record| find_possible(&mut HashMap::new(), record, 0, 0, 0))
            .sum();

        Ok(total)
    }

    fn part_2(&self, input: &str) -> Result<usize, AocError> {
        let records = parse(input)?;

        let total = records
            .iter()
            .map(|record| {
                let springs = record
                    .springs
                    .iter()
                    .copied()
                    .chain([Spring::Unknown])
                    .cycle()
                    .take(5 * record.springs.len() + 4)
                    .collect();

                let groups = record
                    .groups
                    .iter()
                    .copied()
                    .cycle()
                    .take(5 * record.groups.len())
                    .collect();

                find_possible(&mut HashMap::new(), &Record { springs, groups }, 0, 0, 0)
            })
            .sum();

        Ok(total)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_individual() {
        assert_eq!(Day12.part_1("???.### 1,1,3"), Ok(1));
        assert_eq!(Day12.part_1(".??..??...?##. 1,1,3"), Ok(4));
        assert_eq!(Day12.part_1("?#?#?#?#?#?#?#? 1,3,1,6"), Ok(1));
        assert_eq!(Day12.part_1("????.#...#... 4,1,1"), Ok(1));
        assert_eq!(Day12.part_1("????.######..#####. 1,6,5"), Ok(4));
        assert_eq!(Day12.part_1("?###???????? 3,2,1"), Ok(10));
    }

    #[test]
    fn it_solves_part1_full_example() {
        assert_eq!(
            Day12.part_1(
                "???.### 1,1,3\n\
                 .??..??...?##. 1,1,3\n\
                 ?#?#?#?#?#?#?#? 1,3,1,6\n\
                 ????.#...#... 4,1,1\n\
                 ????.######..#####. 1,6,5\n\
                 ?###???????? 3,2,1\n"
            ),
            Ok(21)
        );
    }

    #[test]
    fn it_solves_part2_example() {
        assert_eq!(
            Day12.part_2(
                "???.### 1,1,3\n\
                 .??..??...?##. 1,1,3\n\
                 ?#?#?#?#?#?#?#? 1,3,1,6\n\
                 ????.#...#... 4,1,1\n\
                 ????.######..#####. 1,6,5\n\
                 ?###???????? 3,2,1\n"
            ),
            Ok(525152)
        );
    }

    #[test]
    fn it_solves_part2_example_hard() {
        assert_eq!(Day12.part_2("?###???????? 3,2,1"), Ok(506250));
    }
}
