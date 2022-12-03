use itertools::Itertools;
use std::iter::FromIterator;
use std::{collections::HashSet, error::Error};

use crate::solution::Solution;

pub struct Day04;

impl Solution for Day04 {
    type F = u32;
    type S = u32;

    fn name(&self) -> &'static str {
        "Day 04"
    }

    fn default_input(&self) -> &'static str {
        include_str!("../../inputs/day04.txt")
    }

    fn part_1(&self, input: &str) -> Result<u32, Box<dyn Error>> {
        unimplemented!()
    }

    fn part_2(&self, input: &str) -> Result<u32, Box<dyn Error>> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_example() {
        assert_eq!(
            Day04
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
        unimplemented!();
    }
}
