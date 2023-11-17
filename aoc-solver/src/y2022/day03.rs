use itertools::Itertools;
use std::collections::HashSet;
use std::iter::FromIterator;

use crate::solution::{AocError, Solution};

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

    fn meta(&self) -> (u32, u32) {
        (3, 2022)
    }

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2022/day03.txt")
    }

    fn part_1(&self, input: &str) -> Result<u32, AocError> {
        let mut total = 0;
        for line in input.lines() {
            let (first, second) = line.split_at(line.len() / 2);

            let first: HashSet<u8> = HashSet::from_iter(first.bytes());
            let second: HashSet<u8> = HashSet::from_iter(second.bytes());
            let shared = first
                .intersection(&second)
                .next()
                .ok_or_else(|| AocError::logic("no intersection found"))?;

            total += as_priority(*shared)
        }

        Ok(total)
    }

    fn part_2(&self, input: &str) -> Result<u32, AocError> {
        let mut total = 0;

        for group in input.lines().chunks(3).into_iter() {
            let mut unique = group
                .into_iter()
                .map(|elf| HashSet::<u8>::from_iter(elf.bytes()));

            let mut common = unique.next().ok_or_else(|| AocError::logic("no groups"))?;

            for another in unique {
                common.retain(|item| another.contains(item));
            }

            let shared = common
                .into_iter()
                .next()
                .ok_or_else(|| AocError::logic("no shared value found"))?;

            total += as_priority(shared)
        }

        Ok(total)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_example() {
        assert_eq!(
            Day03.part_1(
                "vJrwpWtwJgWrhcsFMMfFFhFp\n\
                jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\n\
                PmmdzqPrVvPwwTWBwg\n\
                wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\n\
                ttgJtRGJQctTZtZT\n\
                CrZsJsPPZsGzwwsLwLmpwMDw"
            ),
            Ok(157)
        );
    }

    #[test]
    fn it_solves_part2_example() {
        assert_eq!(
            Day03.part_2(
                "vJrwpWtwJgWrhcsFMMfFFhFp\n\
                jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\n\
                PmmdzqPrVvPwwTWBwg\n\
                wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\n\
                ttgJtRGJQctTZtZT\n\
                CrZsJsPPZsGzwwsLwLmpwMDw"
            ),
            Ok(70)
        );
    }
}
