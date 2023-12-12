use std::collections::{HashSet, VecDeque};

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

fn parse(input: &str) -> Result<Vec<(Springs, Groups)>, AocError> {
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

            Ok((springs, groups))
        })
        .try_collect()
}

fn bfs(springs: Springs, groups: Groups) -> usize {
    let mut queue = VecDeque::new();
    let mut found: HashSet<Springs> = HashSet::new();

    let start = (springs, 0, 0, 0, 0, false);
    queue.push_back(start);

    let total_damaged: usize = groups.iter().sum();

    while let Some((
        springs,
        spring_index,
        group_index,
        contiguous,
        mut damaged_count,
        has_to_be_operational,
    )) = queue.pop_back()
    {
        // if we already n damaged, where N >= groups, just stop?
        if damaged_count > total_damaged {
            // Did we reach the end filling all the groups?
            continue;
        }

        if let Some(spring) = springs.get(spring_index) {
            match spring {
                Spring::Operational => {
                    if contiguous == 0 {
                        queue.push_back((
                            springs,
                            spring_index + 1,
                            group_index,
                            0,
                            damaged_count,
                            false,
                        ));
                    }
                }
                Spring::Damaged => {
                    if has_to_be_operational {
                        continue;
                    }

                    damaged_count += 1;

                    if let Some(group) = groups.get(group_index) {
                        // We have a group to fill and just hit '#'.
                        // Did we find enough?
                        if contiguous + 1 == *group {
                            // we did, so move to next group
                            queue.push_back((
                                springs,
                                spring_index + 1,
                                group_index + 1,
                                0,
                                damaged_count,
                                true,
                            ));
                        } else {
                            queue.push_back((
                                springs,
                                spring_index + 1,
                                group_index,
                                contiguous + 1,
                                damaged_count,
                                false,
                            ));
                        }
                    } else {
                        // terminate this attempt, its invalid
                        continue;
                    }
                }
                Spring::Unknown => {
                    let mut operational = springs.clone();
                    operational[spring_index] = Spring::Operational;

                    queue.push_back((
                        operational,
                        spring_index,
                        group_index,
                        contiguous,
                        damaged_count,
                        has_to_be_operational,
                    ));

                    let mut damaged = springs.clone();
                    damaged[spring_index] = Spring::Damaged;

                    queue.push_back((
                        damaged,
                        spring_index,
                        group_index,
                        contiguous,
                        damaged_count,
                        has_to_be_operational,
                    ));
                }
            }
        } else if group_index == groups.len() {
            // Did we reach the end filling all the groups?
            found.insert(springs);
        }
    }

    found.len()
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
            .into_iter()
            .map(|(springs, groups)| {
                // Plan: BFS the all possibilities, terminating the searches as soon as we know its impossible
                bfs(springs, groups)
            })
            .sum();

        Ok(total)
    }

    fn part_2(&self, input: &str) -> Result<usize, AocError> {
        let records = parse(input)?;

        let total = records
            .into_iter()
            .enumerate()
            .map(|(index, (springs, groups))| {
                println!("Index {index}");
                let mut unfolded_springs = springs.clone();
                let mut unfolded_groups = groups.clone();

                for _ in 0..4 {
                    unfolded_springs.push_back(Spring::Unknown);
                    unfolded_springs.append(&mut springs.clone());
                    unfolded_groups.append(&mut groups.clone());
                }

                bfs(unfolded_springs, unfolded_groups)
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
}
