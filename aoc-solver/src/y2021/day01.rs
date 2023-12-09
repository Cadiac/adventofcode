use std::fmt::Debug;
use std::str::FromStr;

use crate::solution::{AocError, Solution};

pub struct Day01;

pub fn parse_from_str<T>(input: &str) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    input
        .lines()
        .map(|depth| depth.parse::<T>().unwrap())
        .collect()
}

impl Solution for Day01 {
    type A = usize;
    type B = usize;

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2021/day01.txt")
    }

    fn part_1(&self, input: &str) -> Result<usize, AocError> {
        let count = parse_from_str::<u32>(input)
            .windows(2)
            .filter(|depths| depths[0] < depths[1])
            .count();

        Ok(count)
    }

    fn part_2(&self, input: &str) -> Result<usize, AocError> {
        let count = parse_from_str(input)
            .windows(3)
            .map(|depths| depths.iter().sum())
            .collect::<Vec<u32>>()
            .windows(2)
            .filter(|depths| depths[0] < depths[1])
            .count();

        Ok(count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_example() {
        assert_eq!(
            Day01.part_1(
                "199\n\
                    200\n\
                    208\n\
                    210\n\
                    200\n\
                    207\n\
                    240\n\
                    269\n\
                    260\n\
                    263"
            ),
            Ok(7)
        );
    }

    #[test]
    fn it_solves_part2_example() {
        assert_eq!(
            Day01.part_2(
                "199\n\
                    200\n\
                    208\n\
                    210\n\
                    200\n\
                    207\n\
                    240\n\
                    269\n\
                    260\n\
                    263"
            ),
            Ok(5)
        );
    }
}
