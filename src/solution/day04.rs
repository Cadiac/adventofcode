use std::{error::Error};

use crate::solution::{Solution, AocError};

pub struct Day04;

fn parse(input: &str) -> ((u32, u32), (u32, u32)) {
    let mut pairs = input.split(",");

    let first: Vec<u32> = pairs
        .next()
        .unwrap()
        .split("-")
        .map(|value| value.parse::<u32>().unwrap())
        .collect();
    let second: Vec<u32> = pairs
        .next()
        .unwrap()
        .split("-")
        .map(|value| value.parse::<u32>().unwrap())
        .collect();

    ((first[0], first[1]), (second[0], second[1]))
}

impl Solution for Day04 {
    type F = usize;
    type S = usize;

    fn name(&self) -> &'static str {
        "Day 04"
    }

    fn default_input(&self) -> &'static str {
        include_str!("../../inputs/day04.txt")
    }

    fn part_1(&self, input: &str) -> Result<usize, AocError> {
        Ok(input
            .lines()
            .map(parse)
            .filter(|(a, b)| a.0 <= b.0 && a.1 >= b.1 || b.0 <= a.0 && b.1 >= a.1)
            .count())
    }

    fn part_2(&self, input: &str) -> Result<usize, AocError> {
        Ok(input
            .lines()
            .map(parse)
            .filter(|(a, b)| u32::max(a.0, b.0) <= u32::min(a.1, b.1))
            .count())
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
                    "2-4,6-8\n\
                    2-3,4-5\n\
                    5-7,7-9\n\
                    2-8,3-7\n\
                    6-6,4-6\n\
                    2-6,4-8"
                )
                .unwrap(),
            2
        );
    }

    #[test]
    fn it_solves_part2_example() {
        assert_eq!(
            Day04
                .part_2(
                    "2-4,6-8\n\
                    2-3,4-5\n\
                    5-7,7-9\n\
                    2-8,3-7\n\
                    6-6,4-6\n\
                    2-6,4-8"
                )
                .unwrap(),
            4
        );
    }
}
