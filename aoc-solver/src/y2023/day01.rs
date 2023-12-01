use crate::solution::{AocError, Solution};

pub struct Day01;

const NUMBERS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

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
                let mut last = None;

                for digit in line.chars() {
                    if let Some(value) = digit.to_digit(10) {
                        if first.is_none() {
                            first = Some(value);
                        }

                        last = Some(value);
                    }
                }

                match (first, last) {
                    (Some(first), Some(last)) => Ok(first * 10 + last),
                    _ => Err(AocError::logic("didn't match both calibration values")),
                }
            })
            .try_fold(0, |acc, result| result.map(|num| acc + num))
    }

    fn part_2(&self, input: &str) -> Result<u32, AocError> {
        input
            .trim()
            .lines()
            .map(|line| {
                let mut first = None;
                let mut last = None;

                for cursor in 0..line.len() {
                    let slice = &line[cursor..];

                    let mut value = NUMBERS.iter().enumerate().find_map(|(index, number)| {
                        if slice.starts_with(number) {
                            return Some((index as u32) + 1);
                        }
                        None
                    });

                    if value.is_none() {
                        if let Some(character) = slice.chars().next() {
                            value = character.to_digit(10);
                        }
                    }

                    if let Some(value) = value {
                        if first.is_none() {
                            first = Some(value)
                        }
                        last = Some(value);
                    }
                }

                match (first, last) {
                    (Some(first), Some(last)) => Ok(first * 10 + last),
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
