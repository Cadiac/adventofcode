use itertools::Itertools;
use std::iter::FromIterator;
use std::{collections::HashSet, error::Error};

use crate::solution::Solution;

pub struct Day03;

fn as_priority(item: u8) -> u32 {
    let priority = if item.is_ascii_lowercase() {
        item - b'a' + 1
    } else {
        item - b'A' + 27
    };

    priority as u32
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

                let first: HashSet<u8> = HashSet::from_iter(first.bytes());
                let second: HashSet<u8> = HashSet::from_iter(second.bytes());
                let shared = first.intersection(&second).next().unwrap();

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
                let mut unique = group
                    .into_iter()
                    .map(|elf| HashSet::<u8>::from_iter(elf.bytes()));

                let mut common = unique.next().unwrap();
                for another in unique {
                    common.retain(|item| another.contains(item));
                }

                let shared = common.into_iter().next().unwrap();

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
