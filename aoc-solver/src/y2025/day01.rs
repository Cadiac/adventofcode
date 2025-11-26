use itertools::Itertools;

use crate::solution::{AocError, Solution};

pub struct Day01;

fn parse(input: &str) -> Result<(Vec<u32>, Vec<u32>), AocError> {
    let result: Vec<(u32, u32)> = input
        .trim()
        .lines()
        .map(|line| {
            let (left_input, right_input) = line
                .trim()
                .split_once("   ")
                .ok_or(AocError::parse(input, "Invalid input"))?;

            let left = left_input
                .parse::<u32>()
                .map_err(|err| AocError::parse(left_input, err))?;
            let right = right_input
                .parse::<u32>()
                .map_err(|err| AocError::parse(right_input, err))?;

            Ok((left, right))
        })
        .try_collect()?;

    Ok(result.into_iter().unzip())
}

impl Solution for Day01 {
    type Part1 = u32;
    type Part2 = u32;

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2025/day01.txt")
    }

    fn part_1(&self, input: &str) -> Result<u32, AocError> {
        let (mut left, mut right) = parse(input)?;

        unimplemented!();
    }

    fn part_2(&self, input: &str) -> Result<u32, AocError> {
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_example() {
        assert_eq!(
            Day01.part_1(
                "3   4\n\
                 4   3\n\
                 2   5\n\
                 1   3\n\
                 3   9\n\
                 3   3"
            ),
            Ok(11)
        );
    }

    #[test]
    fn it_solves_part2_example() {
        assert_eq!(
            Day01.part_2(
                "3   4\n\
                 4   3\n\
                 2   5\n\
                 1   3\n\
                 3   9\n\
                 3   3"
            ),
            Ok(31)
        );
    }
}
