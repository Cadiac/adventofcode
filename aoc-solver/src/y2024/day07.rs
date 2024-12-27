use itertools::Itertools;

use crate::solution::{AocError, Solution};

fn parse(input: &str) -> Result<Vec<(u64, Vec<u64>)>, AocError> {
    let equations = input
        .lines()
        .map(|line| {
            let (target, operands) = line
                .split_once(": ")
                .ok_or_else(|| AocError::parse(input, "Invalid input!"))?;

            let target = target
                .parse::<u64>()
                .map_err(|err| AocError::parse(target, err))?;

            let operands: Vec<u64> = operands
                .split_ascii_whitespace()
                .map(|i: &str| i.parse::<u64>().map_err(|err| AocError::parse(i, err)))
                .try_collect()?;

            Ok((target, operands))
        })
        .try_collect()?;

    Ok(equations)
}

fn is_possible(target: u64, current: u64, rest: &[u64], concat: bool) -> bool {
    if rest.is_empty() {
        return current == target;
    }

    current < target && is_possible(target, current + rest[0], &rest[1..], concat)
        || is_possible(target, current * rest[0], &rest[1..], concat)
        || (concat
            && is_possible(
                target,
                current * 10u64.pow(rest[0].ilog10() + 1) + rest[0],
                &rest[1..],
                concat,
            ))
}

pub struct Day07;
impl Solution for Day07 {
    type Part1 = u64;
    type Part2 = u64;

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2024/day07.txt")
    }

    fn part_1(&self, input: &str) -> Result<u64, AocError> {
        let equations = parse(input)?;

        let sum = equations
            .into_iter()
            .filter_map(|(result, operands)| {
                is_possible(result, operands[0], &operands[1..], false).then_some(result)
            })
            .sum();

        Ok(sum)
    }

    fn part_2(&self, input: &str) -> Result<u64, AocError> {
        let equations = parse(input)?;

        let sum = equations
            .into_iter()
            .filter_map(|(result, operands)| {
                is_possible(result, operands[0], &operands[1..], true).then_some(result)
            })
            .sum();

        Ok(sum)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_example() {
        assert_eq!(
            Day07.part_1(
                "190: 10 19\n\
                 3267: 81 40 27\n\
                 83: 17 5\n\
                 156: 15 6\n\
                 7290: 6 8 6 15\n\
                 161011: 16 10 13\n\
                 192: 17 8 14\n\
                 21037: 9 7 18 13\n\
                 292: 11 6 16 20"
            ),
            Ok(3749)
        );
    }

    #[test]
    fn it_solves_part2_example() {
        assert_eq!(
            Day07.part_2(
                "190: 10 19\n\
                 3267: 81 40 27\n\
                 83: 17 5\n\
                 156: 15 6\n\
                 7290: 6 8 6 15\n\
                 161011: 16 10 13\n\
                 192: 17 8 14\n\
                 21037: 9 7 18 13\n\
                 292: 11 6 16 20"
            ),
            Ok(11387)
        );
    }

    #[test]
    fn it_solves_part2_example_1() {
        assert_eq!(Day07.part_2("156: 15 6"), Ok(156));
    }

    #[test]
    fn it_solves_part2_example_2() {
        assert_eq!(Day07.part_2("7290: 6 8 6 15"), Ok(7290));
    }

    #[test]
    fn it_solves_part2_example_3() {
        assert_eq!(Day07.part_2("192: 17 8 14"), Ok(192));
    }
}
