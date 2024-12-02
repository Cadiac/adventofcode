use itertools::Itertools;

use crate::solution::{AocError, Solution};

pub struct Day02;

fn parse(input: &str) -> Result<Vec<Vec<i32>>, AocError> {
    let reports: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            let levels: Vec<i32> = line
                .split_whitespace()
                .map(|i| i.parse::<i32>().map_err(|err| AocError::parse(i, err)))
                .try_collect()?;

            Ok(levels)
        })
        .try_collect()?;

    Ok(reports)
}

fn is_safe(levels: &[i32]) -> bool {
    let change_sign = (levels[1] - levels[0]).signum();

    levels.windows(2).all(|pair| {
        pair[0].abs_diff(pair[1]) <= 3
            && pair[0].abs_diff(pair[1]) >= 1
            && (pair[1] - pair[0]).signum() == change_sign
    })
}

impl Solution for Day02 {
    type A = usize;
    type B = usize;

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2024/day02.txt")
    }

    fn part_1(&self, input: &str) -> Result<usize, AocError> {
        let safe = parse(input)?
            .into_iter()
            .filter(|levels| is_safe(levels))
            .count();

        Ok(safe)
    }

    fn part_2(&self, input: &str) -> Result<usize, AocError> {
        let safe = parse(input)?
            .iter()
            .filter(|levels| {
                is_safe(levels)
                    || levels
                        .iter()
                        .combinations(levels.len() - 1)
                        .any(|dampened| {
                            is_safe(&dampened.into_iter().copied().collect::<Vec<i32>>())
                        })
            })
            .count();

        Ok(safe)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_example() {
        assert_eq!(
            Day02.part_1(
                "7 6 4 2 1\n\
                 1 2 7 8 9\n\
                 9 7 6 2 1\n\
                 1 3 2 4 5\n\
                 8 6 4 4 1\n\
                 1 3 6 7 9"
            ),
            Ok(2)
        );
    }

    #[test]
    fn it_solves_part2_example() {
        assert_eq!(
            Day02.part_2(
                "7 6 4 2 1\n\
                 1 2 7 8 9\n\
                 9 7 6 2 1\n\
                 1 3 2 4 5\n\
                 8 6 4 4 1\n\
                 1 3 6 7 9"
            ),
            Ok(4)
        );
    }

    #[test]
    fn it_solves_part2_report_1() {
        assert_eq!(Day02.part_2("7 6 4 2 1"), Ok(1));
    }

    #[test]
    fn it_solves_part2_report_2() {
        assert_eq!(Day02.part_2("1 2 7 8 9"), Ok(0));
    }

    #[test]
    fn it_solves_part2_report_3() {
        assert_eq!(Day02.part_2("9 7 6 2 1"), Ok(0));
    }

    #[test]
    fn it_solves_part2_report_4() {
        assert_eq!(Day02.part_2("1 3 2 4 5"), Ok(1));
    }

    #[test]
    fn it_solves_part2_report_5() {
        assert_eq!(Day02.part_2("8 6 4 4 1"), Ok(1));
    }

    #[test]
    fn it_solves_part2_report_6() {
        assert_eq!(Day02.part_2("1 3 6 7 9"), Ok(1));
    }
}
