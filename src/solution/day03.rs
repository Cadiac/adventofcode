use std::{collections::HashSet, error::Error};

use itertools::Itertools;

use crate::solution::Solution;

pub struct Day03;

fn as_priority(item: char) -> u32 {
    if item.is_lowercase() {
        item as u32 - 'a' as u32 + 1
    } else {
        item as u32 - 'A' as u32 + 27
    }
}

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

                let shared = first.intersection(&second).next().expect("shared item");

                as_priority(*shared)
            })
            .sum())
    }

    fn part_2(&self, input: &str) -> Result<u32, Box<dyn Error>> {
        Ok(input
            .lines()
            .chunks(3)
            .into_iter()
            .map(|group| {
                let shared = group
                    .into_iter()
                    .map(|elf| {
                        elf.chars()
                            .collect::<Vec<char>>()
                            .into_iter()
                            .collect::<HashSet<char>>()
                    })
                    .reduce(|acc, unique| acc.intersection(&unique).copied().collect())
                    .expect("non empty group")
                    .into_iter()
                    .next()
                    .expect("shared item");

                as_priority(shared)
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
