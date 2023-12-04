use std::collections::HashSet;

use crate::solution::{AocError, Solution};

pub struct Day04;

#[derive(Debug, Clone)]
struct ScratchCard {
    winning: HashSet<u32>,
    numbers: HashSet<u32>,
    copies: usize,
}

fn parse(input: &str) -> Result<Vec<ScratchCard>, AocError> {
    input.trim().lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> Result<ScratchCard, AocError> {
    let (winning, numbers) = line
        .split(": ")
        .nth(1)
        .ok_or_else(|| AocError::parse(line, "Missing ':' in line"))?
        .split_once(" | ")
        .ok_or_else(|| AocError::parse(line, "Missing '|' in line"))?;

    let winning = parse_numbers(winning, line)?;
    let numbers = parse_numbers(numbers, line)?;

    Ok(ScratchCard {
        winning,
        numbers,
        copies: 1,
    })
}

fn parse_numbers(numbers_str: &str, line: &str) -> Result<HashSet<u32>, AocError> {
    numbers_str
        .split_whitespace()
        .map(|num| {
            num.parse::<u32>()
                .map_err(|_| AocError::parse(line, "Error parsing number"))
        })
        .collect()
}

impl Solution for Day04 {
    type F = u32;
    type S = usize;

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2023/day04.txt")
    }

    fn part_1(&self, input: &str) -> Result<u32, AocError> {
        let total = parse(input)?
            .into_iter()
            .map(|card| {
                let intersection = card.winning.intersection(&card.numbers);
                let wins = intersection.count() as u32;

                if wins == 0 {
                    return 0;
                }

                2u32.pow(wins - 1)
            })
            .sum();

        Ok(total)
    }

    fn part_2(&self, input: &str) -> Result<usize, AocError> {
        let mut cards = parse(input)?;

        let mut total = 0;

        for i in 0..cards.len() {
            let intersection = cards[i].winning.intersection(&cards[i].numbers);
            let wins = intersection.count();
            let copies = cards[i].copies;

            total += copies;

            for win in 0..wins {
                if i + win + 1 < cards.len() {
                    cards[i + win + 1].copies += copies;
                }
            }
        }

        Ok(total)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_example() {
        assert_eq!(
            Day04.part_1(
                "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n\
                 Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n\
                 Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n\
                 Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n\
                 Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n\
                 Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11\n"
            ),
            Ok(13)
        );
    }

    #[test]
    fn it_solves_part2_example() {
        assert_eq!(
            Day04.part_2(
                "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n\
                 Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n\
                 Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n\
                 Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n\
                 Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n\
                 Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11\n"
            ),
            Ok(30)
        );
    }
}
