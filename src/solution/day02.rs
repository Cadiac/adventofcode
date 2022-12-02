use std::error::Error;

use crate::solution::{Answer, Solution};

const DEFAULT_INPUT: &str = include_str!("../../inputs/day02.txt");

#[derive(Clone, Copy)]
enum RPS {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl From<&str> for RPS {
    fn from(input: &str) -> RPS {
        match input {
            "A" | "X" => RPS::Rock,
            "B" | "Y" => RPS::Paper,
            "C" | "Z" => RPS::Scissors,
            _ => unimplemented!(),
        }
    }
}

enum Outcome {
    Lose = 0,
    Draw = 3,
    Win = 6,
}

impl From<&str> for Outcome {
    fn from(input: &str) -> Outcome {
        match input {
            "X" => Outcome::Lose,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => unimplemented!(),
        }
    }
}

pub struct Day02 {
    input: String,
}

impl Day02 {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.to_owned(),
        }
    }
}

fn score(selected: RPS, outcome: Outcome) -> u64 {
    selected as u64 + outcome as u64
}

impl Default for Day02 {
    fn default() -> Self {
        Self {
            input: DEFAULT_INPUT.to_owned(),
        }
    }
}

impl Solution for Day02 {
    fn part_1(&self) -> Result<Answer, Box<dyn Error>> {
        let total = self
            .input
            .lines()
            .map(|line| {
                let mut choices = line.split(" ");
                (
                    choices.next().unwrap().into(),
                    choices.next().unwrap().into(),
                )
            })
            .map(|(opponent, selected): (RPS, RPS)| match opponent {
                RPS::Rock => match selected {
                    RPS::Rock => score(selected, Outcome::Draw),
                    RPS::Paper => score(selected, Outcome::Win),
                    RPS::Scissors => score(selected, Outcome::Lose),
                },
                RPS::Paper => match selected {
                    RPS::Rock => score(selected, Outcome::Lose),
                    RPS::Paper => score(selected, Outcome::Draw),
                    RPS::Scissors => score(selected, Outcome::Win),
                },
                RPS::Scissors => match selected {
                    RPS::Rock => score(selected, Outcome::Win),
                    RPS::Paper => score(selected, Outcome::Lose),
                    RPS::Scissors => score(selected, Outcome::Draw),
                },
            })
            .sum();

        Ok(Answer::U64(total))
    }

    fn part_2(&self) -> Result<Answer, Box<dyn Error>> {
        let total = self
            .input
            .lines()
            .map(|line| {
                let mut choices = line.split(" ");
                (
                    choices.next().unwrap().into(),
                    choices.next().unwrap().into(),
                )
            })
            .map(|(opponent, outcome)| match opponent {
                RPS::Rock => match outcome {
                    Outcome::Lose => score(RPS::Scissors, outcome),
                    Outcome::Draw => score(RPS::Rock, outcome),
                    Outcome::Win => score(RPS::Paper, outcome),
                },
                RPS::Paper => match outcome {
                    Outcome::Lose => score(RPS::Rock, outcome),
                    Outcome::Draw => score(RPS::Paper, outcome),
                    Outcome::Win => score(RPS::Scissors, outcome),
                },
                RPS::Scissors => match outcome {
                    Outcome::Lose => score(RPS::Paper, outcome),
                    Outcome::Draw => score(RPS::Scissors, outcome),
                    Outcome::Win => score(RPS::Rock, outcome),
                },
            })
            .sum();

        Ok(Answer::U64(total))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_example() {
        assert_eq!(
            Day02::new("A Y\nB X\nC Z").part_1().unwrap(),
            Answer::U64(15)
        );
    }

    #[test]
    fn it_solves_part2_example() {
        assert_eq!(
            Day02::new("A Y\nB X\nC Z").part_2().unwrap(),
            Answer::U64(12)
        );
    }
}
