use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, char, digit1},
    combinator::{map, map_res},
    multi::many0,
    sequence::{delimited, preceded, separated_pair},
    IResult,
};

use crate::solution::{AocError, Solution};

#[derive(PartialEq, Debug)]
enum Instruction {
    Mul(u32, u32),
    Enable,
    Disable,
}

fn parse_mul(input: &str) -> IResult<&str, (u32, u32)> {
    preceded(
        tag("mul"),
        delimited(
            char('('),
            separated_pair(
                map_res(digit1, |i: &str| i.parse::<u32>()),
                char(','),
                map_res(digit1, |i: &str| i.parse::<u32>()),
            ),
            char(')'),
        ),
    )(input)
}

fn parse_other(input: &str) -> IResult<&str, ()> {
    let (unhandled, _ignored) = anychar(input)?;
    Ok((unhandled, ()))
}

fn parse_instruction(input: &str) -> IResult<&str, Option<Instruction>> {
    alt((
        map(tag("do()"), |_| Some(Instruction::Enable)),
        map(tag("don't()"), |_| Some(Instruction::Disable)),
        map(parse_mul, |(a, b)| Some(Instruction::Mul(a, b))),
        map(parse_other, |_| None),
    ))(input)
}

fn parse(input: &str) -> Result<Vec<Instruction>, AocError> {
    let (_unhandled, parsed) =
        many0(parse_instruction)(input).map_err(|err| AocError::parse(input, err))?;

    let instructions = parsed.into_iter().flatten().collect();

    Ok(instructions)
}

pub struct Day03;
impl Solution for Day03 {
    type A = u32;
    type B = u32;

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2024/day03.txt")
    }

    fn part_1(&self, input: &str) -> Result<u32, AocError> {
        let sum = parse(input)?
            .iter()
            .map(|instruction| match instruction {
                Instruction::Mul(a, b) => a * b,
                _ => 0,
            })
            .sum();

        Ok(sum)
    }

    fn part_2(&self, input: &str) -> Result<u32, AocError> {
        let mut enabled = true;
        let sum = parse(input)?
            .iter()
            .map(|instruction| match instruction {
                Instruction::Mul(a, b) if enabled => a * b,
                Instruction::Enable => {
                    enabled = true;
                    0
                }
                Instruction::Disable => {
                    enabled = false;
                    0
                }
                _ => 0,
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
            Day03.part_1(
                "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
            ),
            Ok(161)
        );
    }

    #[test]
    fn it_solves_part2_example() {
        assert_eq!(
            Day03.part_2(
                "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
            ),
            Ok(48)
        );
    }

    #[test]
    fn it_parses_input() {
        assert_eq!(
            parse("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"),
            Ok(vec![
                Instruction::Mul(2, 4),
                Instruction::Disable,
                Instruction::Mul(5, 5),
                Instruction::Mul(11, 8),
                Instruction::Enable,
                Instruction::Mul(8, 5),
            ])
        );
    }
}
