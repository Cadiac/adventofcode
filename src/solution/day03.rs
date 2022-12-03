use std::{collections::HashMap, collections::HashSet, error::Error};

use itertools::Itertools;

use crate::solution::Solution;

pub struct Day03;

impl Solution for Day03 {
    type F = u32;
    type S = u32;

    fn name(&self) -> &'static str {
        "Day 03"
    }

    fn default_input(&self) -> &'static str {
        include_str!("../../inputs/day03.txt")
    }

    fn part_1(&self, input: &str) -> Result<u32, Box<dyn Error>> {
        Ok(input
            .lines()
            .map(|line| {
                let (first, second) = line.split_at(line.len() / 2);

                let first: HashSet<char> =
                    first.chars().collect::<Vec<char>>().into_iter().collect();
                let second: HashSet<char> =
                    second.chars().collect::<Vec<char>>().into_iter().collect();

                let shared = first.intersection(&second).next().unwrap();

                if shared.is_lowercase() {
                    *shared as u32 - 'a' as u32 + 1
                } else {
                    *shared as u32 - 'A' as u32 + 27
                }
            })
            .sum())
    }

    fn part_2(&self, input: &str) -> Result<u32, Box<dyn Error>> {
        Ok(input
            .lines()
            .chunks(3)
            .into_iter()
            .map(|group| {
                let unique = group.into_iter().map(|elf| {
                    elf.chars()
                        .collect::<Vec<char>>()
                        .into_iter()
                        .collect::<HashSet<char>>()
                });

                let mut common: HashMap<char, usize> = HashMap::new();

                for chars in unique {
                    for c in chars.iter() {
                        *common.entry(*c).or_insert(0) += 1;
                    }
                }

                let (shared, _) = common.into_iter().find(|(_, count)| *count == 3).unwrap();
                if shared.is_lowercase() {
                    shared as u32 - 'a' as u32 + 1
                } else {
                    shared as u32 - 'A' as u32 + 27
                }
            })
            .sum())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_example() {
        assert_eq!(
            Day03
                .part_1(
                    "vJrwpWtwJgWrhcsFMMfFFhFp\n\
                    jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\n\
                    PmmdzqPrVvPwwTWBwg\n\
                    wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\n\
                    ttgJtRGJQctTZtZT\n\
                    CrZsJsPPZsGzwwsLwLmpwMDw"
                )
                .unwrap(),
            157
        );
    }

    #[test]
    fn it_solves_part2_example() {
        assert_eq!(
            Day03
                .part_2(
                    "vJrwpWtwJgWrhcsFMMfFFhFp\n\
                    jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\n\
                    PmmdzqPrVvPwwTWBwg\n\
                    wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\n\
                    ttgJtRGJQctTZtZT\n\
                    CrZsJsPPZsGzwwsLwLmpwMDw"
                )
                .unwrap(),
            70
        );
    }
}
