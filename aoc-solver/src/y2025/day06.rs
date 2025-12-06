use itertools::Itertools;

use crate::solution::{AocError, Solution};

pub struct Day06;

enum Operator {
    Add,
    Multiply,
}

type Operands = Vec<Vec<u64>>;
type Operators = Vec<(usize, Operator)>;
type Grid = Vec<Vec<char>>;

fn parse(input: &str) -> Result<(Operands, Operators, Grid), AocError> {
    let mut lines = input.lines();

    let operators_str = lines
        .next_back()
        .ok_or(AocError::parse(input, "missing operators"))?;

    let operators: Vec<(usize, Operator)> = operators_str
        .chars()
        .enumerate()
        .flat_map(|(column, operator)| match operator {
            '+' => Some((column, Operator::Add)),
            '*' => Some((column, Operator::Multiply)),
            _ => None,
        })
        .collect();

    let operands: Vec<Vec<u64>> = lines
        .clone()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|operand| {
                    operand
                        .parse::<u64>()
                        .map_err(|err| AocError::parse(operand, err))
                })
                .try_collect()
        })
        .try_collect()?;

    let grid: Vec<Vec<char>> = lines.map(|line| line.chars().collect()).collect();

    Ok((operands, operators, grid))
}

impl Solution for Day06 {
    type Part1 = u64;
    type Part2 = u64;

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2025/day06.txt")
    }

    fn part_1(&self, input: &str) -> Result<u64, AocError> {
        let (operands, operators, _) = parse(input)?;

        let result = operators
            .iter()
            .enumerate()
            .map(|(i, (_, operator))| {
                let iter = (0..operands.len()).map(|j| operands[j][i]);
                match operator {
                    Operator::Add => iter.sum::<u64>(),
                    Operator::Multiply => iter.product(),
                }
            })
            .sum();

        Ok(result)
    }

    fn part_2(&self, input: &str) -> Result<u64, AocError> {
        let (_, operators, grid) = parse(input)?;

        let result = operators
            .iter()
            .map(|(start, operator)| {
                let mut values = Vec::new();
                let mut offset = 0;

                loop {
                    let mut value = 0;
                    for row in &grid {
                        if let Some(digit) = row.get(start + offset).and_then(|c| c.to_digit(10)) {
                            value = value * 10 + digit;
                        }
                    }

                    if value == 0 {
                        break;
                    }

                    values.push(value as u64);
                    offset += 1;
                }

                match operator {
                    Operator::Add => values.iter().sum::<u64>(),
                    Operator::Multiply => values.iter().product(),
                }
            })
            .sum();

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_example() {
        assert_eq!(
            Day06.part_1(
                [
                    "123 328  51 64 ",
                    " 45 64  387 23 ",
                    "  6 98  215 314",
                    "*   +   *   +  "
                ]
                .join("\n")
                .as_str()
            ),
            Ok(4277556)
        );
    }

    #[test]
    fn it_solves_part2_example() {
        assert_eq!(
            Day06.part_2(
                [
                    "123 328  51 64 ",
                    " 45 64  387 23 ",
                    "  6 98  215 314",
                    "*   +   *   +  "
                ]
                .join("\n")
                .as_str()
            ),
            Ok(3263827)
        );
    }
}
