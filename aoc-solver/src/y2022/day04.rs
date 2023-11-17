use crate::solution::{AocError, Solution};
use std::num::ParseIntError;

type Pairs = ((u32, u32), (u32, u32));

pub struct Day04;

fn parse(input: &str) -> Result<Pairs, AocError> {
    let mut pairs = input.split(',');

    let first = pairs
        .next()
        .ok_or_else(|| AocError::parse("first", "no pair"))?
        .split('-')
        .map(|value| value.parse::<u32>())
        .collect::<Result<Vec<u32>, ParseIntError>>()
        .map_err(|err| AocError::parse("first", err))?;

    let second = pairs
        .next()
        .ok_or_else(|| AocError::parse("second", "no pair"))?
        .split('-')
        .map(|value| value.parse::<u32>())
        .collect::<Result<Vec<u32>, ParseIntError>>()
        .map_err(|err| AocError::parse("second", err))?;

    Ok(((first[0], first[1]), (second[0], second[1])))
}

impl Solution for Day04 {
    type F = usize;
    type S = usize;

    fn meta(&self) -> (u32, u32) {
        (4, 2022)
    }

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2022/day04.txt")
    }

    fn part_1(&self, input: &str) -> Result<usize, AocError> {
        let mut count = 0;

        for line in input.lines() {
            let (a, b) = parse(line)?;
            if a.0 <= b.0 && a.1 >= b.1 || b.0 <= a.0 && b.1 >= a.1 {
                count += 1;
            }
        }

        Ok(count)
    }

    fn part_2(&self, input: &str) -> Result<usize, AocError> {
        let mut count = 0;

        for line in input.lines() {
            let (a, b) = parse(line)?;
            if u32::max(a.0, b.0) <= u32::min(a.1, b.1) {
                count += 1;
            }
        }

        Ok(count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_example() {
        assert_eq!(
            Day04.part_1(
                "2-4,6-8\n\
                2-3,4-5\n\
                5-7,7-9\n\
                2-8,3-7\n\
                6-6,4-6\n\
                2-6,4-8"
            ),
            Ok(2)
        );
    }

    #[test]
    fn it_solves_part2_example() {
        assert_eq!(
            Day04.part_2(
                "2-4,6-8\n\
                2-3,4-5\n\
                5-7,7-9\n\
                2-8,3-7\n\
                6-6,4-6\n\
                2-6,4-8"
            ),
            Ok(4)
        );
    }
}
