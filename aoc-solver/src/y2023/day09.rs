use crate::solution::{AocError, Solution};

pub struct Day09;

enum Direction {
    Forward,
    Back,
}

fn parse(input: &str) -> Result<Vec<Vec<i32>>, AocError> {
    input.trim().lines().map(parse_numbers).collect()
}

fn parse_numbers(numbers: &str) -> Result<Vec<i32>, AocError> {
    numbers
        .split_whitespace()
        .map(|number| {
            number
                .parse()
                .map_err(|_| AocError::parse(number, "Error parsing number"))
        })
        .collect()
}

fn extrapolate(mut sequence: Vec<i32>, direction: Direction) -> i32 {
    let mut edges = match direction {
        Direction::Forward => vec![*sequence.last().unwrap_or(&0)],
        Direction::Back => vec![*sequence.first().unwrap_or(&0)],
    };

    while sequence.iter().any(|diff| *diff != 0) {
        sequence = sequence
            .windows(2)
            .map(|values| values[1] - values[0])
            .collect();

        let edge = match direction {
            Direction::Forward => *sequence.last().unwrap_or(&0),
            Direction::Back => *sequence.first().unwrap_or(&0),
        };

        edges.push(edge);
    }

    let prediction = edges.iter().rev().fold(0, |acc, current| match direction {
        Direction::Forward => current + acc,
        Direction::Back => current - acc,
    });

    prediction
}

impl Solution for Day09 {
    type F = i32;
    type S = i32;

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2023/day09.txt")
    }

    fn part_1(&self, input: &str) -> Result<i32, AocError> {
        let sequences = parse(input)?;

        let sum = sequences
            .into_iter()
            .map(|sequence| extrapolate(sequence, Direction::Forward))
            .sum();

        Ok(sum)
    }

    fn part_2(&self, input: &str) -> Result<i32, AocError> {
        let sequences = parse(input)?;

        let sum = sequences
            .into_iter()
            .map(|sequence| extrapolate(sequence, Direction::Back))
            .sum();

        Ok(sum)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_examples() {
        assert_eq!(Day09.part_1("0 3 6 9 12 15"), Ok(18));
        assert_eq!(Day09.part_1("1 3 6 10 15 21"), Ok(28));
        assert_eq!(Day09.part_1("10 13 16 21 30 45"), Ok(68));

        assert_eq!(
            Day09.part_1(
                "0 3 6 9 12 15\n\
                 1 3 6 10 15 21\n\
                 10 13 16 21 30 45\n"
            ),
            Ok(114)
        );
    }

    #[test]
    fn it_solves_part2_examples() {
        assert_eq!(Day09.part_2("0 3 6 9 12 15"), Ok(-3));
        assert_eq!(Day09.part_2("1 3 6 10 15 21"), Ok(0));
        assert_eq!(Day09.part_2("10 13 16 21 30 45"), Ok(5));

        assert_eq!(
            Day09.part_2(
                "0 3 6 9 12 15\n\
                 1 3 6 10 15 21\n\
                 10 13 16 21 30 45\n"
            ),
            Ok(2)
        );
    }
}
