use regex::Regex;

use crate::solution::{AocError, Solution};
pub struct Day01;

fn to_number(input: &str) -> Result<u32, AocError> {
    if input.len() == 1 {
        return input
            .parse()
            .map_err(|err| AocError::parse("invalid digit", err));
    }

    match input {
        "one" => Ok(1),
        "two" => Ok(2),
        "three" => Ok(3),
        "four" => Ok(4),
        "five" => Ok(5),
        "six" => Ok(6),
        "seven" => Ok(7),
        "eight" => Ok(8),
        "nine" => Ok(9),
        _ => Err(AocError::logic("invalid text digit")),
    }
}

impl Solution for Day01 {
    type F = u32;
    type S = u32;

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2023/day01.txt")
    }

    fn part_1(&self, input: &str) -> Result<u32, AocError> {
        input
            .trim()
            .lines()
            .map(|line| {
                let mut first = None;
                let mut second = None;

                for digit in line.chars() {
                    if digit.is_numeric() {
                        let value = digit
                            .to_digit(10)
                            .ok_or_else(|| AocError::logic("invalid digit"))?;

                        if first.is_none() {
                            first = Some(value);
                        }

                        second = Some(value);
                    }
                }

                match (first, second) {
                    (Some(first), Some(second)) => Ok(first * 10 + second),
                    _ => Err(AocError::logic("didn't match both calibration values")),
                }
            })
            .try_fold(0, |acc, result| result.map(|num| acc + num))
    }

    fn part_2(&self, input: &str) -> Result<u32, AocError> {
        let num_regex = Regex::new(r"(\d|one|two|three|four|five|six|seven|eight|nine)").unwrap();

        input
            .trim()
            .lines()
            .map(|line| {
                let mut first = None;
                let mut second = None;

                for i in 0..line.len() {
                    if let Some(capture) = num_regex.captures(&line[i..]) {
                        let matched = to_number(
                            capture
                                .get(1)
                                .ok_or_else(|| AocError::logic("invalid capture"))?
                                .as_str(),
                        )?;

                        if first.is_none() {
                            first = Some(matched);
                        }
                        second = Some(matched);
                    }
                }

                match (first, second) {
                    (Some(first), Some(second)) => Ok(first * 10 + second),
                    _ => Err(AocError::logic("didn't match both calibration values")),
                }
            })
            .try_fold(0, |acc, result| result.map(|num| acc + num))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_example() {
        assert_eq!(
            Day01.part_1(
                "1abc2\n\
                pqr3stu8vwx\n\
                a1b2c3d4e5f\n\
                treb7uchet\n"
            ),
            Ok(142)
        );
    }

    #[test]
    fn it_solves_part2_example() {
        assert_eq!(
            Day01.part_2(
                "two1nine\n\
                eightwothree\n\
                abcone2threexyz\n\
                xtwone3four\n\
                4nineeightseven2\n\
                zoneight234\n\
                7pqrstsixteen"
            ),
            Ok(281)
        );
        assert_eq!(Day01.part_2("twone"), Ok(21));
        assert_eq!(Day01.part_2("eighthree"), Ok(83));
    }
}
