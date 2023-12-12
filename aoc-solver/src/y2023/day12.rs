use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;

use crate::solution::{AocError, Solution};

pub struct Day12;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

type Groups = VecDeque<usize>;
type Springs = VecDeque<Spring>;

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
struct Record {
    groups: Groups,
    springs: Springs,
    contiguous: usize,
    next_has_to_be_operational: bool,
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

            Ok(Record {
                springs,
                groups,
                contiguous: 0,
                next_has_to_be_operational: false,
            })
        })
        .try_collect()
}

fn search(memo: &mut HashMap<Record, usize>, record: &Record) -> usize {
    if let Some(cached) = memo.get(record) {
        return *cached;
    }

    let mut next = record.clone();

    if let Some(spring) = next.springs.pop_front() {
        match spring {
            Spring::Operational => {
                if record.contiguous != 0 {
                    memo.insert(record.clone(), 0);
                    return 0;
                }

                next.next_has_to_be_operational = false;
                let count = search(memo, &next);
                memo.insert(record.clone(), count);
                count
            }
            Spring::Damaged => {
                if next.next_has_to_be_operational {
                    memo.insert(record.clone(), 0);
                    return 0;
                }

                if let Some(group) = next.groups.front() {
                    // We have a group to fill and just hit '#'.
                    // Did we find enough?
                    if next.contiguous + 1 == *group {
                        // we did, so move to next group
                        next.groups.pop_front();
                        next.contiguous = 0;
                        next.next_has_to_be_operational = true;

                        let count = search(memo, &next);
                        memo.insert(record.clone(), count);
                        count
                    } else {
                        next.contiguous += 1;
                        next.next_has_to_be_operational = false;

                        let count = search(memo, &next);
                        memo.insert(record.clone(), count);
                        count
                    }
                } else {
                    // terminate this attempt, its invalid
                    memo.insert(record.clone(), 0);
                    0
                }
            }
            Spring::Unknown => {
                let mut operational = next.clone();
                operational.springs.push_front(Spring::Operational);

                let operational_count = search(memo, &operational);

                let mut damaged = next.clone();
                damaged.springs.push_front(Spring::Damaged);

                let damaged_count = search(memo, &damaged);

                operational_count + damaged_count
            }
        }
    } else if record.groups.is_empty() {
        memo.insert(record.clone(), 1);
        return 1;
    } else {
        memo.insert(record.clone(), 0);
        return 0;
    }
}

impl Solution for Day12 {
    type A = usize;
    type B = usize;

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2023/day12.txt")
    }

    fn part_1(&self, input: &str) -> Result<usize, AocError> {
        let records = parse(input)?;

        let mut memo = HashMap::new();
        let total = records.iter().map(|record| search(&mut memo, record)).sum();

        Ok(total)
    }

    fn part_2(&self, input: &str) -> Result<usize, AocError> {
        let records = parse(input)?;

        let mut memo = HashMap::new();

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

                let unfolded = Record {
                    springs,
                    groups,
                    contiguous: 0,
                    next_has_to_be_operational: false,
                };

                search(&mut memo, &unfolded)
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
