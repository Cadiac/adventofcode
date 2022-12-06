use serde_scan::{self, ScanError};

use crate::solution::{AocError, Solution};

type Instructions = Vec<(usize, usize, usize)>;

pub struct Day05;

fn parse(input: &str) -> Result<(Vec<Vec<char>>, Instructions), AocError> {
    let mut input = input.split("\n\n");
    let crates_input = input
        .next()
        .ok_or_else(|| AocError::parse("crates", "missing"))?;

    // Consume the crates from bottom to up
    let mut crates_input = crates_input.lines().rev();

    // Skip over the crate indices row, just use its length / 4 as the count
    let crates_count = (crates_input
        .next()
        .ok_or_else(|| AocError::parse("crate numbers", "missing"))?
        .len() + 1) / 4;
    let mut crates = vec![Vec::new(); crates_count];

    for line in crates_input {
        for (index, name) in line.chars().skip(1).step_by(4).enumerate() {
            if name != ' ' {
                crates[index].push(name);
            }
        }
    }

    let moves_input = input
        .next()
        .ok_or_else(|| AocError::parse("moves", "missing"))?;

    let moves = moves_input
        .lines()
        .map(|line| serde_scan::scan!("move {} from {} to {}" <- line))
        .collect::<Result<_, ScanError>>()
        .map_err(|err| AocError::parse("moves", err))?;

    Ok((crates, moves))
}

fn top_crates(crates: Vec<Vec<char>>) -> String {
    crates
        .into_iter()
        .flat_map(|mut stack| stack.pop())
        .collect()
}

impl Solution for Day05 {
    type F = String;
    type S = String;

    fn name(&self) -> &'static str {
        "Day 05"
    }

    fn default_input(&self) -> &'static str {
        include_str!("../../inputs/day05.txt")
    }

    fn part_1(&self, input: &str) -> Result<String, AocError> {
        let (mut crates, moves) = parse(input)?;

        for (count, from, to) in moves {
            for _ in 0..count {
                let crate_to_move = crates[from - 1]
                    .pop()
                    .ok_or_else(|| AocError::logic("illegal crate movement"))?;
                crates[to - 1].push(crate_to_move);
            }
        }

        Ok(top_crates(crates))
    }

    fn part_2(&self, input: &str) -> Result<String, AocError> {
        let (mut crates, moves) = parse(input)?;

        for (count, from, to) in moves {
            let mut stack = Vec::new();

            for _ in 0..count {
                stack.push(
                    crates[from - 1]
                        .pop()
                        .ok_or_else(|| AocError::logic("illegal crate movement"))?,
                );
            }

            while let Some(crate_to_move) = stack.pop() {
                crates[to - 1].push(crate_to_move);
            }
        }

        Ok(top_crates(crates))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_example() {
        assert_eq!(
            Day05.part_1(
                vec![
                    "    [D]    ",
                    "[N] [C]    ",
                    "[Z] [M] [P]",
                    " 1   2   3 ",
                    "",
                    "move 1 from 2 to 1",
                    "move 3 from 1 to 3",
                    "move 2 from 2 to 1",
                    "move 1 from 1 to 2"
                ]
                .join("\n")
                .as_str()
            ),
            Ok("CMZ".to_string())
        );
    }

    #[test]
    fn it_solves_part2_example() {
        assert_eq!(
            Day05.part_2(
                vec![
                    "    [D]    ",
                    "[N] [C]    ",
                    "[Z] [M] [P]",
                    " 1   2   3 ",
                    "",
                    "move 1 from 2 to 1",
                    "move 3 from 1 to 3",
                    "move 2 from 2 to 1",
                    "move 1 from 1 to 2"
                ]
                .join("\n")
                .as_str()
            ),
            Ok("MCD".to_string())
        );
    }
}
