use itertools::Itertools;

use crate::solution::{AocError, Solution};

pub struct Day01;

fn parse(input: &str) -> Result<Vec<i32>, AocError> {
    input
        .trim()
        .lines()
        .map(|line| {
            let (direction, value) = line.split_at(1);

            let turn = value
                .parse::<i32>()
                .map_err(|err| AocError::parse(line, err))?;

            match direction {
                "L" => Ok(-turn),
                "R" => Ok(turn),
                _ => Err(AocError::parse(line, "unknown direction")),
            }
        })
        .try_collect()
}

impl Solution for Day01 {
    type Part1 = u32;
    type Part2 = u32;

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2025/day01.txt")
    }

    fn part_1(&self, input: &str) -> Result<u32, AocError> {
        let turns = parse(input)?;
        let mut zeroes = 0;

        turns.into_iter().fold(50, |position, turn| {
            let next = (position + turn).rem_euclid(100);

            if next == 0 {
                zeroes += 1;
            }

            next
        });

        Ok(zeroes)
    }

    fn part_2(&self, input: &str) -> Result<u32, AocError> {
        let turns = parse(input)?;
        let mut zeroes = 0;

        turns.into_iter().fold(50, |prev, turn| {
            let next = (prev + turn).rem_euclid(100);
            let full_rotations = turn.unsigned_abs() / 100;

            zeroes += full_rotations;

            if next == 0 || (prev != 0 && (turn < 0 && next > prev) || (turn > 0 && next < prev)) {
                zeroes += 1;
            }

            next
        });

        Ok(zeroes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_example() {
        assert_eq!(
            Day01.part_1(
                "L68\n\
                 L30\n\
                 R48\n\
                 L5\n\
                 R60\n\
                 L55\n\
                 L1\n\
                 L99\n\
                 R14\n\
                 L82"
            ),
            Ok(3)
        );
    }

    #[test]
    fn it_solves_part2_example() {
        assert_eq!(
            Day01.part_2(
                "L68\n\
                 L30\n\
                 R48\n\
                 L5\n\
                 R60\n\
                 L55\n\
                 L1\n\
                 L99\n\
                 R14\n\
                 L82"
            ),
            Ok(6)
        );
    }
}
